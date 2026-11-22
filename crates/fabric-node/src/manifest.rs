//! Manifest data and a small explicit JSON writer.
//!
//! No serialization dependency is used: the contract has a fixed set of
//! scalar fields, arrays and nested objects, so the writer stays auditable.

use crate::discovery::{self, Hardware};
use crate::identity::{self, LocalIdentity};

pub const MANIFEST_VERSION: &str = "1.0.0";
const HEX: &[u8; 16] = b"0123456789abcdef";

pub struct Node {
    pub node_id: String,
    pub identity_source: &'static str,
    pub os: &'static str,
    pub arch: &'static str,
    pub hostname: String,
}

pub struct Manifest {
    pub manifest_version: &'static str,
    pub node: Node,
    pub hardware: Hardware,
}

pub fn collect() -> Result<Manifest, String> {
    let LocalIdentity {
        node_id,
        identity_source,
        hostname,
    } = identity::collect();
    Ok(Manifest {
        manifest_version: MANIFEST_VERSION,
        node: Node {
            node_id,
            identity_source,
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
            hostname,
        },
        hardware: discovery::collect()?,
    })
}

fn push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            value if (value as u32) < 0x20 => {
                let code = value as u32;
                output.push_str("\\u00");
                output.push(HEX[((code >> 4) & 0x0f) as usize] as char);
                output.push(HEX[(code & 0x0f) as usize] as char);
            }
            value => output.push(value),
        }
    }
    output.push('"');
}

/// Render the versioned local node manifest. The result has no trailing
/// newline so it can be embedded in the registration state document.
pub fn to_json(manifest: &Manifest) -> String {
    let mut output = String::with_capacity(1024);
    output.push_str("{\n  \"manifest_version\": \"");
    output.push_str(manifest.manifest_version);
    output.push_str("\",\n  \"node\": {\n    \"node_id\": ");
    push_json_string(&mut output, &manifest.node.node_id);
    output.push_str(",\n    \"identity_source\": ");
    push_json_string(&mut output, manifest.node.identity_source);
    output.push_str(",\n    \"os\": ");
    push_json_string(&mut output, manifest.node.os);
    output.push_str(",\n    \"arch\": ");
    push_json_string(&mut output, manifest.node.arch);
    output.push_str(",\n    \"hostname\": ");
    push_json_string(&mut output, &manifest.node.hostname);

    output.push_str("\n  },\n  \"hardware\": {\n    \"cpu_count\": ");
    output.push_str(&manifest.hardware.cpu_count.to_string());
    output.push_str(",\n    \"memory_total_bytes\": ");
    match manifest.hardware.memory_total_bytes {
        Some(bytes) => output.push_str(&bytes.to_string()),
        None => output.push_str("null"),
    }
    output.push_str(",\n    \"memory_source\": ");
    push_json_string(&mut output, manifest.hardware.memory_source);

    output.push_str(",\n    \"block_devices\": [");
    for (index, device) in manifest.hardware.block_devices.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("\n      {\n        \"name\": ");
        push_json_string(&mut output, &device.name);
        output.push_str(",\n        \"size_bytes\": ");
        output.push_str(&device.size_bytes.to_string());
        output.push_str("\n      }");
    }
    if !manifest.hardware.block_devices.is_empty() {
        output.push_str("\n    ");
    }
    output.push(']');

    output.push_str(",\n    \"network_interfaces\": [");
    for (index, interface) in manifest.hardware.network_interfaces.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("\n      {\n        \"name\": ");
        push_json_string(&mut output, &interface.name);
        output.push_str(",\n        \"mac\": ");
        push_json_string(&mut output, &interface.mac);
        output.push_str(",\n        \"state\": ");
        push_json_string(&mut output, &interface.state);
        output.push_str("\n      }");
    }
    if !manifest.hardware.network_interfaces.is_empty() {
        output.push_str("\n    ");
    }
    output.push_str("]\n  }\n}");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::{BlockDevice, NetworkInterface};

    fn synthetic_manifest(hostname: &str) -> Manifest {
        Manifest {
            manifest_version: MANIFEST_VERSION,
            node: Node {
                node_id: "00000000-0000-0000-0000-000000000000".to_string(),
                identity_source: "machine-id",
                os: "linux",
                arch: "x86_64",
                hostname: hostname.to_string(),
            },
            hardware: Hardware {
                cpu_count: 4,
                memory_total_bytes: Some(8589934592),
                memory_source: "proc-meminfo",
                block_devices: vec![BlockDevice {
                    name: "sda".to_string(),
                    size_bytes: 1024,
                }],
                network_interfaces: vec![NetworkInterface {
                    name: "eth0".to_string(),
                    mac: "02:00:00:00:00:01".to_string(),
                    state: "up".to_string(),
                }],
            },
        }
    }

    #[test]
    fn json_contains_all_contract_fields() {
        let json = to_json(&synthetic_manifest("example-host"));
        for expected in [
            "\"manifest_version\": \"1.0.0\"",
            "\"node_id\"",
            "\"identity_source\": \"machine-id\"",
            "\"cpu_count\": 4",
            "\"memory_total_bytes\": 8589934592",
            "\"block_devices\"",
            "\"network_interfaces\"",
        ] {
            assert!(json.contains(expected), "missing {expected} in {json}");
        }
        assert!(!json.ends_with('\n'));
    }

    #[test]
    fn json_escapes_synthetic_hostname() {
        let json = to_json(&synthetic_manifest("ho\"st\\name"));
        assert!(json.contains("ho\\\"st\\\\name"));
    }

    #[test]
    fn json_renders_null_memory() {
        let mut manifest = synthetic_manifest("example-host");
        manifest.hardware.memory_total_bytes = None;
        manifest.hardware.memory_source = "unavailable";
        let json = to_json(&manifest);
        assert!(json.contains("\"memory_total_bytes\": null"));
        assert!(json.contains("\"memory_source\": \"unavailable\""));
    }
}
