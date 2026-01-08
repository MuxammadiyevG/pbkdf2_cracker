use crate::parser::ParsedHash;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

/// PBKDF2-HMAC-SHA256 cracker
pub struct Pbkdf2Cracker {
    parsed_hash: ParsedHash,
}

impl Pbkdf2Cracker {
    pub fn new(parsed_hash: ParsedHash) -> Self {
        Self { parsed_hash }
    }

    /// Test a password candidate against the target hash
    /// Uses constant-time comparison to prevent timing attacks
    pub fn test_password(&self, password: &str) -> bool {
        let derived = self.derive_key(password);
        constant_time_compare(&derived, &self.parsed_hash.digest)
    }

    /// Derive PBKDF2-HMAC-SHA256 key from password
    /// This implements the same algorithm used by Flask/Werkzeug
    fn derive_key(&self, password: &str) -> Vec<u8> {
        let mut output = vec![0u8; 32]; // SHA256 produces 32 bytes

        pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            &self.parsed_hash.salt,
            self.parsed_hash.iterations,
            &mut output,
        );

        output
    }

    pub fn iterations(&self) -> u32 {
        self.parsed_hash.iterations
    }

    pub fn salt_len(&self) -> usize {
        self.parsed_hash.salt.len()
    }
}

/// Constant-time comparison to prevent timing attacks
/// Returns true if slices are equal
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_compare() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 3, 4];
        let c = vec![1, 2, 3, 5];

        assert!(constant_time_compare(&a, &b));
        assert!(!constant_time_compare(&a, &c));
    }
}
