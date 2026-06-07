pub fn is_valid_phone(phone: &str) -> bool {
    phone.len() == 11 && phone.chars().all(|ch| ch.is_ascii_digit())
}

pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 8
        && password.chars().any(|ch| ch.is_ascii_alphabetic())
        && password.chars().any(|ch| ch.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_phone() {
        assert!(is_valid_phone("13800138000"));
        assert!(!is_valid_phone("1380013800"));
        assert!(!is_valid_phone("1380013800a"));
    }

    #[test]
    fn validates_password() {
        assert!(is_valid_password("abc123456"));
        assert!(!is_valid_password("12345678"));
        assert!(!is_valid_password("abcdefgh"));
        assert!(!is_valid_password("a1"));
    }
}
