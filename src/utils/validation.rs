#[allow(dead_code)]
pub fn validate_args(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("No command provided. Use --help for usage information.".to_string());
    }
    Ok(())
}

#[allow(dead_code)]
pub fn is_valid_uuid_format(uuid_str: &str) -> bool {
    let uuid_regex = regex::Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
    uuid_regex.is_match(uuid_str)
}

#[allow(dead_code)]
pub fn validate_word_count_range(count: u32, min: u32, max: u32) -> Result<(), String> {
    if count < min {
        return Err(format!("Word count must be at least {}", min));
    }
    if count > max {
        return Err(format!("Word count cannot exceed {}", max));
    }
    Ok(())
}

#[allow(dead_code)]
pub fn sanitize_input(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_args() {
        assert!(validate_args(&["test".to_string()]).is_ok());
        assert!(validate_args(&[]).is_err());
    }

    #[test]
    fn test_is_valid_uuid_format() {
        assert!(is_valid_uuid_format("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!is_valid_uuid_format("invalid-uuid"));
        assert!(!is_valid_uuid_format(""));
    }

    #[test]
    fn test_validate_word_count_range() {
        assert!(validate_word_count_range(10, 1, 100).is_ok());
        assert!(validate_word_count_range(0, 1, 100).is_err());
        assert!(validate_word_count_range(101, 1, 100).is_err());
    }

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("Hello World!@#"), "Hello World");
        assert_eq!(sanitize_input("test_123-abc"), "test_123-abc");
    }
}
