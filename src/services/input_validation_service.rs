use crate::error::{Error, Result};
use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    // Precompiled regex patterns for performance
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();

    static ref USERNAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._-]{1,64}$"
    ).unwrap();

    static ref DN_COMPONENT_REGEX: Regex = Regex::new(
        r"^[a-zA-Z]+=.+$"
    ).unwrap();

    // Common SQL injection patterns
    static ref SQL_INJECTION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)(union|select|insert|update|delete|drop|create|alter|exec|execute)").unwrap(),
        Regex::new(r"(?i)(script|javascript|vbscript|onload|onerror|onclick)").unwrap(),
        Regex::new(r#"['";]"#).unwrap(),
    ];

    // LDAP injection patterns
    static ref LDAP_INJECTION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"[()&|!]").unwrap(),
        Regex::new(r"\\[0-9a-fA-F]{2}").unwrap(),
    ];
}

/// Input validation and sanitization service
pub struct InputValidationService;

impl InputValidationService {
    /// Validate and sanitize username input
    pub fn validate_username(username: &str) -> Result<String> {
        // Check for null or empty
        if username.is_empty() {
            return Err(Error::InvalidArgument(
                "Username cannot be empty".to_string(),
            ));
        }

        // Check length limits
        if username.len() > 256 {
            return Err(Error::InvalidArgument("Username too long".to_string()));
        }

        // Check for control characters and null bytes
        if username.chars().any(|c| c.is_control() || c == '\0') {
            return Err(Error::InvalidArgument(
                "Username contains invalid characters".to_string(),
            ));
        }

        // Check for basic format (alphanumeric, dots, underscores, hyphens)
        if !USERNAME_REGEX.is_match(username) {
            return Err(Error::InvalidArgument(
                "Username format is invalid".to_string(),
            ));
        }

        // Check for potential injection patterns
        Self::check_injection_patterns(username)?;

        Ok(username.to_string())
    }

    /// Validate password strength and format
    pub fn validate_password(password: &str) -> Result<()> {
        if password.is_empty() {
            return Err(Error::InvalidArgument(
                "Password cannot be empty".to_string(),
            ));
        }

        if password.len() > 1024 {
            return Err(Error::InvalidArgument("Password too long".to_string()));
        }

        // Check for null bytes
        if password.contains('\0') {
            return Err(Error::InvalidArgument(
                "Password contains invalid characters".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate email address format
    pub fn validate_email(email: &str) -> Result<String> {
        if email.is_empty() {
            return Err(Error::InvalidArgument("Email cannot be empty".to_string()));
        }

        if email.len() > 320 {
            // RFC 5321 limit
            return Err(Error::InvalidArgument("Email address too long".to_string()));
        }

        if !EMAIL_REGEX.is_match(email) {
            return Err(Error::InvalidArgument("Invalid email format".to_string()));
        }

        // Check for potential injection patterns
        Self::check_injection_patterns(email)?;

        Ok(email.to_lowercase())
    }

    /// Validate LDAP Distinguished Name (DN)
    pub fn validate_dn(dn: &str) -> Result<String> {
        if dn.is_empty() {
            return Err(Error::InvalidArgument("DN cannot be empty".to_string()));
        }

        if dn.len() > 1024 {
            return Err(Error::InvalidArgument("DN too long".to_string()));
        }

        // Check for control characters
        if dn.chars().any(|c| c.is_control()) {
            return Err(Error::InvalidArgument(
                "DN contains invalid characters".to_string(),
            ));
        }

        // Basic DN format validation
        let components: Vec<&str> = dn.split(',').collect();
        for component in components {
            let trimmed = component.trim();
            if !DN_COMPONENT_REGEX.is_match(trimmed) {
                return Err(Error::InvalidArgument(
                    "Invalid DN component format".to_string(),
                ));
            }
        }

        Ok(dn.to_string())
    }

    // /// Validate LDAP filter string
    // pub fn validate_ldap_filter(filter: &str) -> Result<String> {
    //     if filter.is_empty() {
    //         return Err(Error::InvalidArgument(
    //             "LDAP filter cannot be empty".to_string(),
    //         ));
    //     }

    //     if filter.len() > 512 {
    //         return Err(Error::InvalidArgument("LDAP filter too long".to_string()));
    //     }

    //     // Check for control characters
    //     if filter.chars().any(|c| c.is_control()) {
    //         return Err(Error::InvalidArgument(
    //             "LDAP filter contains invalid characters".to_string(),
    //         ));
    //     }

    //     // Basic parentheses balance check
    //     let open_count = filter.chars().filter(|&c| c == '(').count();
    //     let close_count = filter.chars().filter(|&c| c == ')').count();
    //     if open_count != close_count {
    //         return Err(Error::InvalidArgument(
    //             "Unbalanced parentheses in LDAP filter".to_string(),
    //         ));
    //     }

    //     Ok(filter.to_string())
    // }

    /// Sanitize string for LDAP filter use (escape special characters)
    pub fn sanitize_ldap_value(value: &str) -> Result<String> {
        if value.is_empty() {
            return Err(Error::InvalidArgument("Value cannot be empty".to_string()));
        }

        if value.len() > 256 {
            return Err(Error::InvalidArgument("Value too long".to_string()));
        }

        // Check for null bytes and control characters
        if value.contains('\0') || value.chars().any(|c| c.is_control()) {
            return Err(Error::InvalidArgument(
                "Value contains invalid characters".to_string(),
            ));
        }

        // Escape special LDAP characters according to RFC 4515
        let mut escaped = String::with_capacity(value.len() * 2);
        for ch in value.chars() {
            match ch {
                '*' => escaped.push_str("\\2a"),
                '(' => escaped.push_str("\\28"),
                ')' => escaped.push_str("\\29"),
                '\\' => escaped.push_str("\\5c"),
                '\0' => escaped.push_str("\\00"),
                _ => escaped.push(ch),
            }
        }

        Ok(escaped)
    }

    /// Validate server URL format
    pub fn validate_server_url(url: &str) -> Result<String> {
        if url.is_empty() {
            return Err(Error::InvalidArgument(
                "Server URL cannot be empty".to_string(),
            ));
        }

        if url.len() > 512 {
            return Err(Error::InvalidArgument("Server URL too long".to_string()));
        }

        // Check for valid schemes
        if !url.starts_with("ldap://") && !url.starts_with("ldaps://") {
            return Err(Error::InvalidArgument(
                "Invalid URL scheme. Must be ldap:// or ldaps://".to_string(),
            ));
        }

        // Check for control characters
        if url.chars().any(|c| c.is_control()) {
            return Err(Error::InvalidArgument(
                "URL contains invalid characters".to_string(),
            ));
        }

        // Basic URL validation - should contain hostname
        let without_scheme = url.replace("ldap://", "").replace("ldaps://", "");
        if without_scheme.is_empty() || without_scheme.starts_with(':') {
            return Err(Error::InvalidArgument("Invalid URL format".to_string()));
        }

        Ok(url.to_string())
    }

    /// Check for common injection attack patterns
    fn check_injection_patterns(input: &str) -> Result<()> {
        // Check for SQL injection patterns
        for pattern in SQL_INJECTION_PATTERNS.iter() {
            if pattern.is_match(input) {
                return Err(Error::InvalidArgument(
                    "Input contains invalid characters".to_string(),
                ));
            }
        }

        // Check for LDAP injection patterns
        for pattern in LDAP_INJECTION_PATTERNS.iter() {
            if pattern.is_match(input) {
                return Err(Error::InvalidArgument(
                    "Input contains invalid characters".to_string(),
                ));
            }
        }

        Ok(())
    }

    // /// Validate configuration value
    // pub fn validate_config_value(key: &str, value: &str) -> Result<String> {
    //     match key {
    //         "AVORED_LDAP_SERVER" => Self::validate_server_url(value),
    //         "AVORED_LDAP_BASE_DN" | "AVORED_LDAP_BIND_DN" | "AVORED_LDAP_USER_SEARCH_BASE" => {
    //             Self::validate_dn(value)
    //         }
    //         "AVORED_LDAP_USER_SEARCH_FILTER" => Self::validate_ldap_filter(value),
    //         "AVORED_LDAP_PORT" => {
    //             let port: u16 = value
    //                 .parse()
    //                 .map_err(|_| Error::InvalidArgument("Invalid port number".to_string()))?;
    //             if port == 0 {
    //                 return Err(Error::InvalidArgument("Port cannot be zero".to_string()));
    //             }
    //             Ok(value.to_string())
    //         }
    //         "AVORED_LDAP_CONNECTION_TIMEOUT" | "AVORED_LDAP_SEARCH_TIMEOUT" => {
    //             let timeout: u64 = value
    //                 .parse()
    //                 .map_err(|_| Error::InvalidArgument("Invalid timeout value".to_string()))?;
    //             if timeout == 0 || timeout > 300 {
    //                 return Err(Error::InvalidArgument(
    //                     "Timeout must be between 1 and 300 seconds".to_string(),
    //                 ));
    //             }
    //             Ok(value.to_string())
    //         }
    //         "AVORED_LDAP_USE_TLS" | "AVORED_LDAP_ENABLED" => {
    //             value
    //                 .parse::<bool>()
    //                 .map_err(|_| Error::InvalidArgument("Invalid boolean value".to_string()))?;
    //             Ok(value.to_string())
    //         }
    //         _ => {
    //             // Generic validation for other config values
    //             if value.len() > 1024 {
    //                 return Err(Error::InvalidArgument(
    //                     "Configuration value too long".to_string(),
    //                 ));
    //             }
    //             if value
    //                 .chars()
    //                 .any(|c| c.is_control() && c != '\t' && c != '\n' && c != '\r')
    //             {
    //                 return Err(Error::InvalidArgument(
    //                     "Configuration value contains invalid characters".to_string(),
    //                 ));
    //             }
    //             Ok(value.to_string())
    //         }
    //     }
    // }

    // /// Sanitize log message to prevent log injection
    // pub fn sanitize_log_message(message: &str) -> String {
    //     message
    //         .chars()
    //         .filter(|&c| !c.is_control() || c == ' ')
    //         .collect::<String>()
    //         .replace('\n', " ")
    //         .replace('\r', " ")
    //         .replace('\t', " ")
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_validation() {
        assert!(InputValidationService::validate_username("valid_user").is_ok());
        assert!(InputValidationService::validate_username("user.name").is_ok());
        assert!(InputValidationService::validate_username("user-123").is_ok());

        assert!(InputValidationService::validate_username("").is_err());
        assert!(InputValidationService::validate_username("user@domain").is_err());
        assert!(InputValidationService::validate_username("user space").is_err());
        assert!(InputValidationService::validate_username("user\0null").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(InputValidationService::validate_email("user@example.com").is_ok());
        assert!(InputValidationService::validate_email("test.email+tag@domain.co.uk").is_ok());

        assert!(InputValidationService::validate_email("").is_err());
        assert!(InputValidationService::validate_email("invalid-email").is_err());
        assert!(InputValidationService::validate_email("user@").is_err());
        assert!(InputValidationService::validate_email("@domain.com").is_err());
    }

    #[test]
    fn test_ldap_sanitization() {
        assert_eq!(
            InputValidationService::sanitize_ldap_value("test*user").unwrap(),
            "test\\2auser"
        );
        assert_eq!(
            InputValidationService::sanitize_ldap_value("test(user)").unwrap(),
            "test\\28user\\29"
        );
    }
}
