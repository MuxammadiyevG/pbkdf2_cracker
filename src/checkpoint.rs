use crate::errors::{CrackerError, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Checkpoint data for resuming cracking sessions
#[derive(Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Wordlist offset (number of words processed)
    pub wordlist_offset: u64,

    /// Rule index (which rule was being applied)
    pub rule_index: usize,

    /// Total attempts made
    pub total_attempts: u64,

    /// Timestamp of checkpoint
    pub timestamp: String,
}

impl Checkpoint {
    pub fn new(wordlist_offset: u64, rule_index: usize, total_attempts: u64) -> Self {
        Self {
            wordlist_offset,
            rule_index,
            total_attempts,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Save checkpoint to file
    pub fn save(&self, path: &str) -> Result<()> {
        let file = File::create(path).map_err(|e| {
            CrackerError::CheckpointError(format!("Failed to create checkpoint file: {}", e))
        })?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).map_err(|e| {
            CrackerError::CheckpointError(format!("Failed to write checkpoint: {}", e))
        })?;

        Ok(())
    }

    /// Load checkpoint from file
    pub fn load(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            return Err(CrackerError::CheckpointError(
                "Checkpoint file not found".to_string(),
            ));
        }

        let file = File::open(path).map_err(|e| {
            CrackerError::CheckpointError(format!("Failed to open checkpoint file: {}", e))
        })?;

        let reader = BufReader::new(file);
        let checkpoint = serde_json::from_reader(reader).map_err(|e| {
            CrackerError::CheckpointError(format!("Failed to parse checkpoint: {}", e))
        })?;

        Ok(checkpoint)
    }

    /// Delete checkpoint file
    pub fn delete(path: &str) -> Result<()> {
        if Path::new(path).exists() {
            std::fs::remove_file(path).map_err(|e| {
                CrackerError::CheckpointError(format!("Failed to delete checkpoint: {}", e))
            })?;
        }
        Ok(())
    }
}

/// Checkpoint manager for periodic saves
pub struct CheckpointManager {
    path: String,
    save_interval: u64,
    last_save: u64,
}

impl CheckpointManager {
    pub fn new(path: String, save_interval: u64) -> Self {
        Self {
            path,
            save_interval,
            last_save: 0,
        }
    }

    /// Check if checkpoint should be saved based on attempts
    pub fn should_save(&self, total_attempts: u64) -> bool {
        total_attempts - self.last_save >= self.save_interval
    }

    /// Save checkpoint if interval has passed
    pub fn maybe_save(
        &mut self,
        wordlist_offset: u64,
        rule_index: usize,
        total_attempts: u64,
    ) -> Result<()> {
        if self.should_save(total_attempts) {
            let checkpoint = Checkpoint::new(wordlist_offset, rule_index, total_attempts);
            checkpoint.save(&self.path)?;
            self.last_save = total_attempts;
        }
        Ok(())
    }

    /// Force save checkpoint
    pub fn save(
        &mut self,
        wordlist_offset: u64,
        rule_index: usize,
        total_attempts: u64,
    ) -> Result<()> {
        let checkpoint = Checkpoint::new(wordlist_offset, rule_index, total_attempts);
        checkpoint.save(&self.path)?;
        self.last_save = total_attempts;
        Ok(())
    }
}

// Simple implementation without chrono dependency
mod chrono {
    pub struct Utc;

    impl Utc {
        pub fn now() -> DateTime {
            DateTime
        }
    }

    pub struct DateTime;

    impl DateTime {
        pub fn to_rfc3339(&self) -> String {
            // Simple timestamp format
            format!("{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs())
        }
    }
}
