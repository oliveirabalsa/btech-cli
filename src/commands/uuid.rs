use uuid::Uuid;
use crate::utils::copy;

pub fn execute() {
    let new_uuid = Uuid::new_v4();
    println!("{}", new_uuid);
    copy::copy_to_clipboard(&new_uuid.to_string());
    println!("Copied to clipboard");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_generation() {
        let uuid = Uuid::new_v4();
        assert!(!uuid.to_string().is_empty());
        assert_eq!(uuid.to_string().len(), 36);
    }
}
