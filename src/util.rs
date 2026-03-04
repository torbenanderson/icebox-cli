//! Shared utility helpers for deterministic formatting/id generation.

use rand_core::{OsRng, RngCore};

/// Encodes bytes to lowercase hexadecimal.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

/// Generates a random UUID-like agent identifier (MVP local format).
pub fn generate_agent_id() -> String {
    let mut random = [0u8; 16];
    OsRng.fill_bytes(&mut random);
    format!(
        "{}-{}-{}-{}-{}",
        bytes_to_hex(&random[0..4]),
        bytes_to_hex(&random[4..6]),
        bytes_to_hex(&random[6..8]),
        bytes_to_hex(&random[8..10]),
        bytes_to_hex(&random[10..16]),
    )
}
