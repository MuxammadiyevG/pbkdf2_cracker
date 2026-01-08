use crate::errors::{CrackerError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Wordlist reader with offset support for resuming
pub struct WordlistReader {
    path: String,
    current_offset: u64,
}

impl WordlistReader {
    pub fn new(path: String) -> Self {
        Self {
            path,
            current_offset: 0,
        }
    }

    /// Create reader starting from a specific offset
    pub fn from_offset(path: String, offset: u64) -> Self {
        Self {
            path,
            current_offset: offset,
        }
    }

    /// Get current offset (number of words processed)
    pub fn offset(&self) -> u64 {
        self.current_offset
    }

    /// Read all words from the wordlist starting from current offset
    pub fn read_words(&mut self) -> Result<WordlistIterator> {
        let file = File::open(&self.path).map_err(|e| {
            CrackerError::WordlistNotFound(format!("Failed to open {}: {}", self.path, e))
        })?;

        Ok(WordlistIterator {
            reader: BufReader::new(file),
            current_line: 0,
            start_offset: self.current_offset,
        })
    }

    /// Count total words in wordlist
    pub fn count_words(&self) -> Result<u64> {
        let file = File::open(&self.path).map_err(|e| {
            CrackerError::WordlistNotFound(format!("Failed to open {}: {}", self.path, e))
        })?;

        let reader = BufReader::new(file);
        let count = reader.lines().count() as u64;
        Ok(count)
    }
}

/// Iterator over wordlist lines
pub struct WordlistIterator {
    reader: BufReader<File>,
    current_line: u64,
    start_offset: u64,
}

impl WordlistIterator {
    /// Skip to the start offset
    fn skip_to_offset(&mut self) -> Result<()> {
        while self.current_line < self.start_offset {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line).map_err(|e| {
                CrackerError::WordlistReadError(format!("Failed to skip line: {}", e))
            })?;

            if bytes_read == 0 {
                break;
            }

            self.current_line += 1;
        }
        Ok(())
    }

    pub fn current_line(&self) -> u64 {
        self.current_line
    }
}

impl Iterator for WordlistIterator {
    type Item = Result<(u64, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip to offset on first call
        if self.current_line < self.start_offset {
            if let Err(e) = self.skip_to_offset() {
                return Some(Err(e));
            }
        }

        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(_) => {
                let word = line.trim().to_string();
                let offset = self.current_line;
                self.current_line += 1;

                if word.is_empty() {
                    self.next() // Skip empty lines
                } else {
                    Some(Ok((offset, word)))
                }
            }
            Err(e) => Some(Err(CrackerError::WordlistReadError(format!(
                "Failed to read line: {}",
                e
            )))),
        }
    }
}
