#[allow(dead_code)]
pub fn format_command_help(command: &str, description: &str) -> String {
    format!("{:<12} {}", command, description)
}

#[allow(dead_code)]
pub fn validate_word_count(count: u32) -> Result<u32, String> {
    if count == 0 {
        return Err("Word count must be greater than 0".to_string());
    }
    if count > 10000 {
        return Err("Word count cannot exceed 10,000".to_string());
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_word_count() {
        assert!(validate_word_count(10).is_ok());
        assert!(validate_word_count(0).is_err());
        assert!(validate_word_count(10001).is_err());
    }

    #[test]
    fn test_format_command_help() {
        let result = format_command_help("uuid", "Generate UUID");
        assert!(result.contains("uuid"));
        assert!(result.contains("Generate UUID"));
    }
}
