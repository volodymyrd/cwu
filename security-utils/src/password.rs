use rand::prelude::IndexedRandom;
use rand::{rng, seq::SliceRandom};

/// All allowed characters
const ALL_CHARACTERS: [u8; 90] =
    *b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_+=[]{}|;:,.<>?/~";
/// All allowed lowercase ASCII letters (26 characters).
const CHARS_LOWERCASE: [u8; 26] = *b"abcdefghijklmnopqrstuvwxyz";
/// All allowed uppercase ASCII letters (26 characters).
const CHARS_UPPERCASE: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// All allowed numeric digits (10 characters).
const CHARS_NUMBERS: [u8; 10] = *b"0123456789";
/// All allowed symbols (28 characters).
const CHARS_SYMBOLS: [u8; 28] = *b"!@#$%^&*()-_+=[]{}|;:,.<>?/~";

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("The requested length must be greater or equal to 4.")]
    TooShort,
}

/// Generates a cryptographically secure password.
///
/// This function uses a 'strict' approach: it ensures that at least one character
/// from each enabled set is included, guaranteeing complexity compliance.
pub fn generate_secure_password(length: usize) -> Result<String, PasswordError> {
    if length < 4 {
        return Err(PasswordError::TooShort);
    }

    let mut rng = rng();
    let mut password_bytes = Vec::with_capacity(length);

    // Start with one of each required character type.
    // Unwraps are safe as character sets are not empty.
    password_bytes.push(*CHARS_LOWERCASE.choose(&mut rng).unwrap());
    password_bytes.push(*CHARS_UPPERCASE.choose(&mut rng).unwrap());
    password_bytes.push(*CHARS_NUMBERS.choose(&mut rng).unwrap());
    password_bytes.push(*CHARS_SYMBOLS.choose(&mut rng).unwrap());

    // Fill the remaining length with random characters from all sets.
    let remaining = length - password_bytes.len();
    for _ in 0..remaining {
        password_bytes.push(*ALL_CHARACTERS.choose(&mut rng).unwrap());
    }

    // Shuffle the collected bytes to randomize character positions.
    password_bytes.shuffle(&mut rng);

    // The `from_utf8` call is safe because all source characters are valid ASCII.
    Ok(String::from_utf8(password_bytes)
        .expect("Password generation should result in valid UTF-8 ASCII"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to check if a string contains a character from a set
    fn contains_any(password: &str, charset: &[u8]) -> bool {
        password.as_bytes().iter().any(|b| charset.contains(b))
    }

    #[test]
    fn test_password_length() {
        let password = generate_secure_password(16).unwrap();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_strict_complexity_compliance() {
        // Run multiple times to reduce the chance of false positive
        for _ in 0..100 {
            let password = generate_secure_password(20).unwrap();

            // Should contain a lowercase character
            assert!(
                contains_any(&password, CHARS_LOWERCASE.as_slice()),
                "Missing lowercase"
            );

            // Must contain numbers
            assert!(
                contains_any(&password, CHARS_NUMBERS.as_slice()),
                "Missing numbers"
            );

            // Must contain symbols
            assert!(
                contains_any(&password, CHARS_SYMBOLS.as_slice()),
                "Missing symbols"
            );

            // Must contain uppercase
            assert!(
                contains_any(&password, CHARS_UPPERCASE.as_slice()),
                "Missing uppercase"
            );
        }
    }

    #[test]
    fn test_errors() {
        assert!(generate_secure_password(3).is_err());
    }
}
