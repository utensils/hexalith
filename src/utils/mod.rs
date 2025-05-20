use crate::Result;
use uuid::Uuid;

#[cfg(test)]
mod tests;

/// Converts a UUID to a deterministic seed value
pub fn uuid_to_seed(uuid: &str) -> Result<u64> {
    let uuid = Uuid::parse_str(uuid)?;
    let bytes = uuid.as_bytes();

    // Convert first 8 bytes to a u64
    let mut seed = 0u64;
    for i in 0..8 {
        seed = (seed << 8) | bytes[i] as u64;
    }

    Ok(seed)
}

/// Returns a default color palette
pub fn default_color_palette() -> Vec<&'static str> {
    vec![
        "#4285F4", // Google Blue
        "#EA4335", // Google Red
        "#FBBC05", // Google Yellow
        "#34A853", // Google Green
        "#9C27B0", // Purple
        "#00BCD4", // Cyan
        "#FF9800", // Orange
        "#607D8B", // Blue Grey
    ]
}
