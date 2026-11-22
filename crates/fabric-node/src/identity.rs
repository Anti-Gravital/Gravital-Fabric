//! Stable, non-cryptographic local identifier for the prototype.
//!
//! The algorithm is fixed so the identifier is reproducible on the same host
//! and across toolchains. It is not a credential and provides no security
//! guarantee; see ADR-0008.

use std::fs;

pub struct LocalIdentity {
    pub node_id: String,
    pub identity_source: &'static str,
    pub hostname: String,
}

const DOMAIN: &[u8] = b"gravital-fabric-node-v1\0";
const OFFSET_A: u64 = 0xcbf2_9ce4_8422_2325;
const OFFSET_B: u64 = 0x8422_2325_cbf2_9ce4;
const PRIME: u64 = 0x0000_0100_0000_01b3;
const HEX: &[u8; 16] = b"0123456789abcdef";

pub fn collect() -> LocalIdentity {
    let hostname =
        read_non_empty("/proc/sys/kernel/hostname").unwrap_or_else(|| "unknown-host".to_string());
    collect_from(
        read_non_empty("/etc/machine-id"),
        read_non_empty("/var/lib/dbus/machine-id"),
        &hostname,
    )
}

fn collect_from(
    machine_id: Option<String>,
    dbus_id: Option<String>,
    hostname: &str,
) -> LocalIdentity {
    let (material, source) = if let Some(value) = machine_id {
        (value, "machine-id")
    } else if let Some(value) = dbus_id {
        (value, "dbus-machine-id")
    } else {
        (hostname.to_string(), "hostname")
    };
    LocalIdentity {
        node_id: format_id(&material),
        identity_source: source,
        hostname: hostname.to_string(),
    }
}

fn read_non_empty(path: &str) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn fnv1a(bytes: &[u8], offset: u64) -> u64 {
    let mut hash = offset;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

/// Derive the lowercase 8-4-4-4-12 identifier from a material string.
pub fn format_id(material: &str) -> String {
    let mut input = DOMAIN.to_vec();
    input.extend_from_slice(material.as_bytes());
    let first = fnv1a(&input, OFFSET_A);
    let second = fnv1a(&input, OFFSET_B);
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&first.to_le_bytes());
    bytes[8..].copy_from_slice(&second.to_le_bytes());

    let mut identifier = String::with_capacity(36);
    for (index, byte) in bytes.iter().enumerate() {
        if matches!(index, 4 | 6 | 8 | 10) {
            identifier.push('-');
        }
        identifier.push(HEX[usize::from(*byte >> 4)] as char);
        identifier.push(HEX[usize::from(*byte & 0x0f)] as char);
    }
    identifier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_matches_fixed_vector() {
        assert_eq!(
            format_id("fixed-material"),
            "04804226-5ce1-b3b2-d54b-6733fed9ea99"
        );
    }

    #[test]
    fn identifier_is_stable_and_input_sensitive() {
        assert_eq!(format_id("host-a"), format_id("host-a"));
        assert_ne!(format_id("host-a"), format_id("host-b"));
    }

    #[test]
    fn identifier_has_expected_shape() {
        let identifier = format_id("shape");
        assert_eq!(identifier.len(), 36);
        for index in [8, 13, 18, 23] {
            assert_eq!(identifier.as_bytes()[index], b'-');
        }
        assert!(identifier
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() || byte == b'-'));
    }

    #[test]
    fn machine_id_is_preferred_over_dbus_and_hostname() {
        let identity = collect_from(
            Some("local-machine-id".to_string()),
            Some("dbus-id".to_string()),
            "example-host",
        );
        assert_eq!(identity.identity_source, "machine-id");
        assert_eq!(identity.node_id, format_id("local-machine-id"));
        assert_eq!(identity.hostname, "example-host");
    }

    #[test]
    fn hostname_is_last_resort_source() {
        let identity = collect_from(None, None, "fallback-host");
        assert_eq!(identity.identity_source, "hostname");
        assert_eq!(identity.node_id, format_id("fallback-host"));
    }

    #[test]
    fn dbus_id_is_second_source() {
        let identity = collect_from(None, Some("dbus-id".to_string()), "example-host");
        assert_eq!(identity.identity_source, "dbus-machine-id");
        assert_eq!(identity.node_id, format_id("dbus-id"));
    }
}
