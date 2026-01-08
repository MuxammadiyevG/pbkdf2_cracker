use std::fmt;

#[derive(Debug)]
pub enum CrackerError {
    InvalidHashFormat(String),
    InvalidIterations(String),
    InvalidSalt(String),
    InvalidDigest(String),
    WordlistNotFound(String),
    WordlistReadError(String),
    RulesFileError(String),
    CheckpointError(String),
    VerificationError(String),
    Pbkdf2Error(String),
}

impl fmt::Display for CrackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrackerError::InvalidHashFormat(msg) => write!(f, "Invalid hash format: {}", msg),
            CrackerError::InvalidIterations(msg) => write!(f, "Invalid iterations: {}", msg),
            CrackerError::InvalidSalt(msg) => write!(f, "Invalid salt: {}", msg),
            CrackerError::InvalidDigest(msg) => write!(f, "Invalid digest: {}", msg),
            CrackerError::WordlistNotFound(msg) => write!(f, "Wordlist not found: {}", msg),
            CrackerError::WordlistReadError(msg) => write!(f, "Wordlist read error: {}", msg),
            CrackerError::RulesFileError(msg) => write!(f, "Rules file error: {}", msg),
            CrackerError::CheckpointError(msg) => write!(f, "Checkpoint error: {}", msg),
            CrackerError::VerificationError(msg) => write!(f, "Verification error: {}", msg),
            CrackerError::Pbkdf2Error(msg) => write!(f, "PBKDF2 error: {}", msg),
        }
    }
}

impl std::error::Error for CrackerError {}

pub type Result<T> = std::result::Result<T, CrackerError>;
