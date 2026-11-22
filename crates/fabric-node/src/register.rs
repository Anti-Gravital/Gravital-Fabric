//! Local registration state file writer.
//!
//! Registration here means writing a JSON document to a caller-selected local
//! path. It does not contact a control plane or claim a Fabric registration.

use std::ffi::{OsStr, OsString};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::manifest::{self, Manifest};

pub const STATE_VERSION: &str = "1.0.0";

// Distinguishes writers that share a process (threads) in addition to the
// process id and timestamp used in the temporary name.
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub fn write_state(path: &Path, manifest: &Manifest) -> Result<u64, String> {
    let registered_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    let json = to_json(registered_at, manifest);

    let file_name = path
        .file_name()
        .ok_or_else(|| "state path has no file name".to_string())?;

    // The temporary file lives in the destination directory so the final
    // rename stays on one filesystem and is atomic. Each writer gets its own
    // name, so no writer can rename or delete another writer's file.
    // create_new is an atomic O_CREAT|O_EXCL: a name collision is detected
    // instead of silently opening a shared path, and is retried with a fresh
    // name. This keeps concurrent registrations independent (last rename wins).
    const MAX_ATTEMPTS: u64 = 64;
    let mut attempt = 0u64;
    let (temporary_path, mut file) = loop {
        let candidate = path.with_file_name(temporary_name(file_name, attempt));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => break (candidate, file),
            Err(error)
                if error.kind() == std::io::ErrorKind::AlreadyExists
                    && attempt + 1 < MAX_ATTEMPTS =>
            {
                attempt += 1;
                continue;
            }
            Err(error) => {
                return Err(format!(
                    "cannot create temporary state file {}: {error}",
                    candidate.display()
                ));
            }
        }
    };

    if let Err(error) = file.write_all(json.as_bytes()) {
        // Only this writer's uniquely named file is removed here.
        let _ = fs::remove_file(&temporary_path);
        return Err(format!(
            "cannot write {}: {error}",
            temporary_path.display()
        ));
    }
    drop(file);

    if let Err(error) = fs::rename(&temporary_path, path) {
        // The rename may have been observed by another writer; removing this
        // path is still safe because no other writer ever uses this name.
        let _ = fs::remove_file(&temporary_path);
        return Err(format!(
            "cannot rename state file to {}: {error}",
            path.display()
        ));
    }
    Ok(registered_at)
}

fn temporary_name(file_name: &OsStr, attempt: u64) -> OsString {
    let pid = std::process::id();
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos() as u64);
    let mut name = OsString::from(".");
    name.push(file_name);
    name.push(format!(".{pid}.{sequence}.{nanos}.{attempt}.tmp"));
    name
}

fn to_json(registered_at: u64, manifest: &Manifest) -> String {
    let mut output = String::with_capacity(1536);
    output.push_str("{\n  \"state_version\": \"");
    output.push_str(STATE_VERSION);
    output.push_str("\",\n  \"registered_at_unix\": ");
    output.push_str(&registered_at.to_string());
    output.push_str(",\n  \"manifest\": ");
    for (index, line) in manifest::to_json(manifest).lines().enumerate() {
        if index > 0 {
            output.push_str("  ");
        }
        output.push_str(line);
        output.push('\n');
    }
    output.push_str("}\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::{BlockDevice, Hardware, NetworkInterface};
    use crate::manifest::{Node, MANIFEST_VERSION};
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn synthetic_manifest() -> Manifest {
        Manifest {
            manifest_version: MANIFEST_VERSION,
            node: Node {
                node_id: "00000000-0000-0000-0000-000000000000".to_string(),
                identity_source: "hostname",
                os: "linux",
                arch: "x86_64",
                hostname: "example-host".to_string(),
            },
            hardware: Hardware {
                cpu_count: 2,
                memory_total_bytes: None,
                memory_source: "unavailable",
                block_devices: Vec::<BlockDevice>::new(),
                network_interfaces: Vec::<NetworkInterface>::new(),
            },
        }
    }

    #[test]
    fn state_json_contains_version_timestamp_and_manifest() {
        let json = to_json(1_700_000_000, &synthetic_manifest());
        assert!(json.contains("\"state_version\": \"1.0.0\""));
        assert!(json.contains("\"registered_at_unix\": 1700000000"));
        assert!(json.contains("\"manifest\": {"));
        assert!(json.contains("\"manifest_version\": \"1.0.0\""));
        assert!(json.ends_with("}\n"));
    }

    #[test]
    fn write_state_creates_file_and_removes_temporary_file() {
        let sequence = COUNTER.fetch_add(1, Ordering::SeqCst);
        let directory = std::env::temp_dir().join(format!(
            "fabric-node-register-{}-{sequence}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(&directory).expect("create fixture dir");
        let path = directory.join("state.json");

        let timestamp = write_state(&path, &synthetic_manifest()).expect("write state");
        assert!(timestamp > 0);
        let content = fs::read_to_string(&path).expect("read state");
        assert!(content.contains("\"registered_at_unix\": "));
        assert!(!directory.join(".state.json.tmp").exists());

        fs::remove_dir_all(&directory).expect("remove fixture dir");
    }
}
