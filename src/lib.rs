//! # Free Edu Email Generator Helper
//! Official Website: <https://www.ues.edu.pl/>

/// Returns the official educational domain for UES temp mail.
pub fn get_edu_domain() -> &'static str {
    "ues.edu.pl"
}

/// Generates a temporary educational email address with the given username.
pub fn generate_email(username: &str) -> String {
    format!("{}@{}", username, get_edu_domain())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_format() {
        assert_eq!(generate_email("student"), "student@ues.edu.pl");
    }
}
