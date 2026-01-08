use crate::errors::{CrackerError, Result};

/// Parsed Flask/Werkzeug PBKDF2-SHA256 hash components
#[derive(Debug, Clone)]
pub struct ParsedHash {
    pub iterations: u32,
    pub salt: Vec<u8>,
    pub digest: Vec<u8>,
}

impl ParsedHash {
    /// Parse a Flask/Werkzeug PBKDF2-SHA256 hash
    /// Format: pbkdf2:sha256:<iterations>$<salt>$<hex_digest>
    /// CRITICAL: Salt is UTF-8 encoded string, NOT base64!
    pub fn parse(hash: &str) -> Result<Self> {
        // Split by colons to get main components
        let parts: Vec<&str> = hash.split(':').collect();

        if parts.len() != 3 {
            return Err(CrackerError::InvalidHashFormat(
                "Expected format: pbkdf2:sha256:<iterations>$<salt>$<digest>".to_string(),
            ));
        }

        // Verify method is pbkdf2
        if parts[0] != "pbkdf2" {
            return Err(CrackerError::InvalidHashFormat(
                format!("Expected 'pbkdf2', got '{}'", parts[0]),
            ));
        }

        // Verify algorithm is sha256
        if parts[1] != "sha256" {
            return Err(CrackerError::InvalidHashFormat(
                format!("Expected 'sha256', got '{}'", parts[1]),
            ));
        }

        // Parse the remaining part: <iterations>$<salt>$<digest>
        let components: Vec<&str> = parts[2].split('$').collect();

        if components.len() != 3 {
            return Err(CrackerError::InvalidHashFormat(
                "Expected format: <iterations>$<salt>$<digest>".to_string(),
            ));
        }

        // Parse iterations
        let iterations = components[0]
            .parse::<u32>()
            .map_err(|e| CrackerError::InvalidIterations(format!("Failed to parse iterations: {}", e)))?;

        if iterations == 0 {
            return Err(CrackerError::InvalidIterations(
                "Iterations must be greater than 0".to_string(),
            ));
        }

        // CRITICAL FIX: Salt is UTF-8 encoded, not base64!
        // Flask/Werkzeug uses the salt string directly as bytes
        let salt = components[1].as_bytes().to_vec();

        if salt.is_empty() {
            return Err(CrackerError::InvalidSalt("Salt cannot be empty".to_string()));
        }

        // Decode hex digest
        let digest = hex::decode(components[2])
            .map_err(|e| CrackerError::InvalidDigest(format!("Failed to decode hex digest: {}", e)))?;

        if digest.len() != 32 {
            return Err(CrackerError::InvalidDigest(
                format!("Expected 32 bytes (SHA256), got {}", digest.len()),
            ));
        }

        Ok(ParsedHash {
            iterations,
            salt,
            digest,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_hash() {
        let hash = "pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133";
        let parsed = ParsedHash::parse(hash).unwrap();
        assert_eq!(parsed.iterations, 600000);
        assert_eq!(parsed.salt, b"AMtzteQIG7yAbZIa");
        assert_eq!(parsed.digest.len(), 32);
    }
}
