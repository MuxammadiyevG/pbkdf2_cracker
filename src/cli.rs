use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "pbkdf2_cracker",
    version = "1.0.0",
    about = "Production-grade PBKDF2-SHA256 password cracker for Flask/Werkzeug hashes",
    long_about = "A high-performance Rust CLI tool for cracking Flask/Werkzeug PBKDF2-SHA256 hashes in CTF/HTB environments.\n\n\
                  Hash Format: pbkdf2:sha256:<iterations>$<salt>$<hex_digest>\n\
                  Example: pbkdf2:sha256:600000$AMtzteQIG7yAbZIa$0673ad90a0b4afb19d662336f0fce3a9edd0b7b19193717be28ce4d66c887133"
)]
pub struct Cli {
    /// Target hash to crack
    #[arg(long, required_unless_present = "verify")]
    pub hash: Option<String>,

    /// Path to wordlist file
    #[arg(long, required_unless_present = "verify")]
    pub wordlist: Option<String>,

    /// Path to rules file (optional)
    #[arg(long)]
    pub rules: Option<String>,

    /// Number of threads to use (default: CPU cores)
    #[arg(long, default_value_t = num_cpus::get())]
    pub threads: usize,

    /// Enable resume mode
    #[arg(long)]
    pub resume: bool,

    /// Path to checkpoint file
    #[arg(long, default_value = "checkpoint.json")]
    pub checkpoint: String,

    /// Password to verify (verification mode)
    #[arg(long)]
    pub verify: Option<String>,

    /// Enable verbose output
    #[arg(long, short)]
    pub verbose: bool,

    /// Use default rule mutations
    #[arg(long)]
    pub default_rules: bool,
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        // Verify mode validation
        if self.verify.is_some() {
            if self.hash.is_none() {
                return Err("--hash is required for verification mode".to_string());
            }
            return Ok(());
        }

        // Crack mode validation
        if self.hash.is_none() {
            return Err("--hash is required".to_string());
        }

        if self.wordlist.is_none() {
            return Err("--wordlist is required".to_string());
        }

        if self.threads == 0 {
            return Err("--threads must be greater than 0".to_string());
        }

        Ok(())
    }

    pub fn is_verify_mode(&self) -> bool {
        self.verify.is_some()
    }
}
