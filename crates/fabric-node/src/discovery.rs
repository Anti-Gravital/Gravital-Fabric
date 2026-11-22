//! Read-only host discovery from procfs, sysfs and the Rust standard library.
//!
//! Every probe degrades to an empty value when the corresponding Linux pseudo
//! file is absent. No network call is made.

use std::fs;
use std::io::ErrorKind;
use std::path::Path;

pub struct Hardware {
    pub cpu_count: usize,
    pub memory_total_bytes: Option<u64>,
    pub memory_source: &'static str,
    pub block_devices: Vec<BlockDevice>,
    pub network_interfaces: Vec<NetworkInterface>,
}

pub struct BlockDevice {
    pub name: String,
    pub size_bytes: u64,
}

pub struct NetworkInterface {
    pub name: String,
    pub mac: String,
    pub state: String,
}

pub fn collect() -> Result<Hardware, String> {
    let cpu_count = std::thread::available_parallelism().map_or(1, |count| count.get());
    let (memory_total_bytes, memory_source) = memory_total()?;
    let block_devices = match fs::read_to_string("/proc/partitions") {
        Ok(text) => parse_partitions(&text),
        Err(error) if error.kind() == ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(format!("cannot read /proc/partitions: {error}")),
    };
    let network_interfaces = match fs::read_dir("/sys/class/net") {
        Ok(directory) => read_network_interfaces(directory)?,
        Err(error) if error.kind() == ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(format!("cannot read /sys/class/net: {error}")),
    };
    Ok(Hardware {
        cpu_count,
        memory_total_bytes,
        memory_source,
        block_devices,
        network_interfaces,
    })
}

fn memory_total() -> Result<(Option<u64>, &'static str), String> {
    match fs::read_to_string("/proc/meminfo") {
        Ok(text) => match parse_meminfo_kb(&text) {
            Some(bytes) => Ok((Some(bytes), "proc-meminfo")),
            None => Ok((None, "unavailable")),
        },
        Err(error) if error.kind() == ErrorKind::NotFound => Ok((None, "unavailable")),
        Err(error) => Err(format!("cannot read /proc/meminfo: {error}")),
    }
}

/// Parse the `MemTotal` line of a procfs meminfo snapshot into bytes.
fn parse_meminfo_kb(input: &str) -> Option<u64> {
    for line in input.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if key.trim() != "MemTotal" {
            continue;
        }
        let mut fields = value.split_whitespace();
        let amount = fields.next()?.parse::<u64>().ok()?;
        let multiplier = match fields.next().unwrap_or("kB") {
            "B" => 1,
            "kB" => 1024,
            "mB" => 1024 * 1024,
            _ => return None,
        };
        return Some(amount * multiplier);
    }
    None
}

/// Parse a procfs partitions snapshot; the header is skipped by the numeric
/// parse of the block-count column.
fn parse_partitions(input: &str) -> Vec<BlockDevice> {
    let mut devices = Vec::new();
    for line in input.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() != 4 {
            continue;
        }
        let Ok(blocks) = fields[2].parse::<u64>() else {
            continue;
        };
        devices.push(BlockDevice {
            name: fields[3].to_string(),
            size_bytes: blocks * 1024,
        });
    }
    devices
}

/// Read interface name, MAC address and operational state from a sysfs net
/// directory. Sorted by name so the manifest is reproducible.
fn read_network_interfaces(directory: fs::ReadDir) -> Result<Vec<NetworkInterface>, String> {
    let mut interfaces = Vec::new();
    for entry in directory {
        let entry = entry.map_err(|error| format!("cannot read interface entry: {error}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let base = entry.path();
        interfaces.push(NetworkInterface {
            name,
            mac: read_trimmed(&base.join("address")),
            state: read_trimmed(&base.join("operstate")),
        });
    }
    interfaces.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(interfaces)
}

fn read_trimmed(path: &Path) -> String {
    fs::read_to_string(path)
        .map(|text| text.trim().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn fixture_dir(label: &str) -> std::path::PathBuf {
        let sequence = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "fabric-node-discovery-{label}-{}-{sequence}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create fixture dir");
        path
    }

    #[test]
    fn parses_memtotal_from_synthetic_fixture() {
        let fixture = concat!(
            "MemTotal:       16384 kB\n",
            "MemFree:         2048 kB\n",
            "MemAvailable:    8192 kB\n"
        );
        assert_eq!(parse_meminfo_kb(fixture), Some(16384 * 1024));
    }

    #[test]
    fn memtotal_missing_returns_none() {
        assert_eq!(parse_meminfo_kb("MemFree: 2048 kB\n"), None);
    }

    #[test]
    fn parses_partitions_from_synthetic_fixture() {
        let fixture = concat!(
            "major minor  #blocks  name\n",
            "\n",
            "   7        0       1024 loop0\n",
            "   8        0    1048576 sda\n",
            "   8        1     524288 sda1\n"
        );
        let devices = parse_partitions(fixture);
        assert_eq!(devices.len(), 3);
        assert_eq!(devices[0].name, "loop0");
        assert_eq!(devices[0].size_bytes, 1024 * 1024);
        assert_eq!(devices[2].name, "sda1");
        assert_eq!(devices[2].size_bytes, 524288 * 1024);
    }

    #[test]
    fn reads_network_interfaces_from_synthetic_sysfs() {
        let root = fixture_dir("net");
        for (name, mac, state) in [
            ("eth0", "02:00:00:00:00:01", "up"),
            ("lo", "00:00:00:00:00:00", "unknown"),
        ] {
            let interface = root.join(name);
            fs::create_dir(&interface).expect("create interface dir");
            fs::write(interface.join("address"), format!("{mac}\n")).expect("write mac");
            fs::write(interface.join("operstate"), format!("{state}\n")).expect("write state");
        }
        let directory = fs::read_dir(&root).expect("read fixture dir");
        let interfaces = read_network_interfaces(directory).expect("read interfaces");
        assert_eq!(interfaces.len(), 2);
        assert_eq!(interfaces[0].name, "eth0");
        assert_eq!(interfaces[0].mac, "02:00:00:00:00:01");
        assert_eq!(interfaces[1].name, "lo");
        assert_eq!(interfaces[1].state, "unknown");
        fs::remove_dir_all(&root).expect("remove fixture dir");
    }

    #[test]
    fn collects_real_host_without_network_access() {
        let hardware = collect().expect("discover this host");
        assert!(hardware.cpu_count >= 1);
        assert!(hardware.memory_total_bytes.unwrap_or(0) > 0);
    }
}
