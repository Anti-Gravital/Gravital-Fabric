//! Integration tests that run the real binary with no external service.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_fabric-node"))
}

fn temporary_directory() -> PathBuf {
    let sequence = COUNTER.fetch_add(1, Ordering::SeqCst);
    let path =
        std::env::temp_dir().join(format!("fabric-node-cli-{}-{sequence}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("create fixture dir");
    path
}

fn node_id_from(json: &str) -> String {
    let marker = "\"node_id\": \"";
    let start = json.find(marker).expect("node_id field in JSON") + marker.len();
    json[start..start + 36].to_string()
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn manifest_prints_versioned_json() {
    let output = binary().arg("manifest").output().expect("run manifest");
    assert_success(&output);
    assert!(output.stderr.is_empty());
    let json = String::from_utf8(output.stdout).expect("utf8 manifest");
    assert!(json.starts_with("{\n"));
    assert!(json.ends_with("}\n"));
    for field in [
        "\"manifest_version\": \"1.0.0\"",
        "\"identity_source\"",
        "\"os\"",
        "\"arch\"",
        "\"hostname\"",
        "\"cpu_count\"",
        "\"memory_total_bytes\"",
        "\"block_devices\"",
        "\"network_interfaces\"",
    ] {
        assert!(json.contains(field), "missing {field} in {json}");
    }
}

#[test]
fn identity_and_hardware_snapshot_are_reproducible() {
    let first = binary().arg("manifest").output().expect("first manifest");
    let second = binary().arg("manifest").output().expect("second manifest");
    assert_success(&first);
    assert_success(&second);
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(
        node_id_from(&String::from_utf8_lossy(&first.stdout)),
        node_id_from(&String::from_utf8_lossy(&second.stdout))
    );
}

#[test]
fn register_writes_a_local_state_file() {
    let directory = temporary_directory();
    let state = directory.join("state.json");
    let output = binary()
        .arg("register")
        .arg("--state")
        .arg(&state)
        .output()
        .expect("run register");
    assert_success(&output);
    assert!(state.exists());
    let content = fs::read_to_string(&state).expect("read state file");
    for field in [
        "\"state_version\": \"1.0.0\"",
        "\"registered_at_unix\"",
        "\"manifest\": {",
        "\"manifest_version\": \"1.0.0\"",
        "\"node_id\"",
    ] {
        assert!(content.contains(field), "missing {field} in {content}");
    }
    let manifest_output = binary().arg("manifest").output().expect("manifest");
    let manifest_json = String::from_utf8(manifest_output.stdout).expect("utf8 manifest");
    assert_eq!(node_id_from(&content), node_id_from(&manifest_json));
    assert!(!directory.join(".state.json.tmp").exists());
    fs::remove_dir_all(&directory).expect("remove fixture dir");
}

#[test]
fn register_accepts_a_custom_state_path() {
    let directory = temporary_directory();
    let state = directory.join("nested-name.json");
    let output = binary()
        .args(["register", "--state"])
        .arg(Path::new(&state))
        .output()
        .expect("run register with custom path");
    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nested-name.json"));
    assert!(state.exists());
    fs::remove_dir_all(&directory).expect("remove fixture dir");
}

#[test]
fn usage_errors_exit_nonzero() {
    let unknown = binary().arg("bogus").output().expect("unknown command");
    assert_eq!(unknown.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&unknown.stderr).contains("unknown command"));

    let extra = binary()
        .args(["manifest", "extra"])
        .output()
        .expect("extra argument");
    assert_eq!(extra.status.code(), Some(2));

    let missing_value = binary()
        .args(["register", "--state"])
        .output()
        .expect("missing state value");
    assert_eq!(missing_value.status.code(), Some(2));
}

#[test]
fn concurrent_registers_to_one_state_path_all_succeed() {
    // Regression for issue #3: the old fixed ".<file>.tmp" name made one
    // writer rename away the file another writer was about to rename.
    const WRITERS: usize = 32;
    let directory = temporary_directory();
    let state = directory.join("state.json");

    let mut children = Vec::with_capacity(WRITERS);
    for _ in 0..WRITERS {
        children.push(
            binary()
                .args(["register", "--state"])
                .arg(&state)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("spawn concurrent register"),
        );
    }

    for (index, child) in children.into_iter().enumerate() {
        let output = child.wait_with_output().expect("wait for register");
        assert!(
            output.status.success(),
            "concurrent writer {index} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // The last rename wins; the resulting document must still be whole JSON.
    let content = fs::read_to_string(&state).expect("read final state file");
    assert!(
        content.starts_with("{\n"),
        "state does not start with an object"
    );
    assert!(content.ends_with("}\n"), "state is truncated");
    assert_eq!(
        content.matches('{').count(),
        content.matches('}').count(),
        "unbalanced braces in final state"
    );
    for field in [
        "\"state_version\": \"1.0.0\"",
        "\"registered_at_unix\"",
        "\"manifest\": {",
        "\"manifest_version\": \"1.0.0\"",
    ] {
        assert!(content.contains(field), "missing {field} in {content}");
    }

    let leftovers: Vec<String> = fs::read_dir(&directory)
        .expect("read fixture dir")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".tmp"))
        .collect();
    assert!(
        leftovers.is_empty(),
        "temporary files left behind: {leftovers:?}"
    );
    fs::remove_dir_all(&directory).expect("remove fixture dir");
}

#[test]
fn help_and_version_succeed() {
    let version = binary().arg("--version").output().expect("version");
    assert_success(&version);
    assert!(String::from_utf8_lossy(&version.stdout).contains("fabric-node 0.1.0"));
    let help = binary().arg("--help").output().expect("help");
    assert_success(&help);
    assert!(String::from_utf8_lossy(&help.stdout).contains("Usage: fabric-node"));
}
