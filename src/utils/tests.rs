#[cfg(test)]
mod tests {
    use crate::utils::{default_color_palette, uuid_to_seed};

    #[test]
    fn test_uuid_to_seed() {
        // Test a valid UUID
        let uuid = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
        let seed = uuid_to_seed(uuid).unwrap();

        // The seed should be deterministic
        let seed2 = uuid_to_seed(uuid).unwrap();
        assert_eq!(seed, seed2);

        // Different UUIDs should produce different seeds
        let uuid2 = "123e4567-e89b-12d3-a456-426614174000";
        let seed3 = uuid_to_seed(uuid2).unwrap();
        assert_ne!(seed, seed3);
    }

    #[test]
    fn test_invalid_uuid() {
        // Test an invalid UUID
        let result = uuid_to_seed("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_default_color_palette() {
        let palette = default_color_palette();

        // Should have colors
        assert!(!palette.is_empty());

        // First color should start with #
        assert!(palette[0].starts_with('#'));
    }
}
