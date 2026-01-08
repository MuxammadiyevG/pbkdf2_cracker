mod cli;
mod cracker;
mod parser;
mod rules;
mod wordlist;
mod checkpoint;
mod verify;
mod stats;
mod errors;

use clap::Parser;
use cli::Cli;
use cracker::Pbkdf2Cracker;
use errors::Result;
use parser::ParsedHash;
use rules::RuleEngine;
use wordlist::WordlistReader;
use checkpoint::{Checkpoint, CheckpointManager};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

fn main() {
    let cli = Cli::parse();

    // Validate arguments
    if let Err(e) = cli.validate() {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }

    // Handle verification mode
    if cli.is_verify_mode() {
        let hash = cli.hash.unwrap();
        let password = cli.verify.unwrap();
        let exit_code = verify::verify_and_report(&hash, &password);
        std::process::exit(exit_code);
    }

    // Run cracking mode
    match run_cracker(cli) {
        Ok(true) => {
            std::process::exit(0);
        }
        Ok(false) => {
            println!("\n😞 Password not found");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("\n❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_cracker(cli: Cli) -> Result<bool> {
    let hash = cli.hash.unwrap();
    let wordlist_path = cli.wordlist.unwrap();

    // Print banner
    print_banner();

    // Parse hash
    println!("🔍 Parsing hash...");
    let parsed_hash = ParsedHash::parse(&hash)?;
    println!("   Iterations: {}", parsed_hash.iterations);
    println!("   Salt: {}", String::from_utf8_lossy(&parsed_hash.salt));
    println!("   Salt length: {} bytes", parsed_hash.salt.len());
    println!("   Digest length: {} bytes", parsed_hash.digest.len());
    println!();

    // Load or create checkpoint
    let (start_offset, _start_rule_index) = if cli.resume {
        match Checkpoint::load(&cli.checkpoint) {
            Ok(checkpoint) => {
                println!("📂 Resuming from checkpoint:");
                println!("   Wordlist offset: {}", checkpoint.wordlist_offset);
                println!("   Total attempts: {}", checkpoint.total_attempts);
                println!();
                (checkpoint.wordlist_offset, checkpoint.rule_index)
            }
            Err(_) => {
                println!("⚠  No checkpoint found, starting from beginning");
                println!();
                (0, 0)
            }
        }
    } else {
        (0, 0)
    };

    // Load rule engine
    println!("📋 Loading rules...");
    let rule_engine = if let Some(rules_path) = cli.rules {
        RuleEngine::from_file(&rules_path)?
    } else if cli.default_rules {
        RuleEngine::default_rules()
    } else {
        RuleEngine::new()
    };
    println!("   Loaded {} rules", rule_engine.count());
    println!();

    // Set up wordlist
    println!("📖 Loading wordlist: {}", wordlist_path);
    let mut wordlist_reader = WordlistReader::from_offset(wordlist_path.clone(), start_offset);
    let total_words = wordlist_reader.count_words()?;
    println!("   Total words: {}", total_words);
    if start_offset > 0 {
        println!("   Starting from offset: {}", start_offset);
    }
    println!();

    println!("🎯 Attack configuration:");
    println!("   Threads: {}", cli.threads);
    println!("   Chunk size: 1000");
    println!();

    // Set thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(cli.threads)
        .build_global()
        .unwrap();

    // Create cracker (shared across threads)
    let cracker = Arc::new(Pbkdf2Cracker::new(parsed_hash));

    // Stats
    let attempts = Arc::new(AtomicU64::new(0));
    let found = Arc::new(AtomicBool::new(false));
    let found_password = Arc::new(std::sync::Mutex::new(String::new()));

    // Checkpoint manager
    let mut checkpoint_mgr = CheckpointManager::new(cli.checkpoint.clone(), 10000);

    println!("🚀 Starting password cracking...");
    println!();

    let start_time = Instant::now();
    let mut last_report = Instant::now();

    // Read wordlist
    let mut words_iter = wordlist_reader.read_words()?;

    // Collect words in chunks for better parallelism
    let mut chunk = Vec::new();
    const CHUNK_SIZE: usize = 1000;

    while let Some(word_result) = words_iter.next() {
        if found.load(Ordering::Relaxed) {
            break;
        }

        let (offset, base_word) = word_result?;
        chunk.push((offset, base_word));

        // Process chunk when full
        if chunk.len() >= CHUNK_SIZE {
            let result = process_chunk(
                &chunk,
                &cracker,
                &rule_engine,
                &attempts,
                &found,
                &found_password,
            );

            // Report progress
            let now = Instant::now();
            if now.duration_since(last_report) >= Duration::from_secs(2) {
                let elapsed = start_time.elapsed().as_secs_f64();
                let total_attempts = attempts.load(Ordering::Relaxed);
                let speed = total_attempts as f64 / elapsed;
                print!(
                    "\r[+] Attempts: {:>10} | Elapsed: {:>6.1}s | Speed: {:>8.2} H/s",
                    total_attempts, elapsed, speed
                );
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                last_report = now;
            }

            // Save checkpoint
            let _ = checkpoint_mgr.maybe_save(offset, 0, attempts.load(Ordering::Relaxed));

            chunk.clear();

            if result {
                break;
            }
        }
    }

    // Process remaining chunk
    if !chunk.is_empty() && !found.load(Ordering::Relaxed) {
        process_chunk(
            &chunk,
            &cracker,
            &rule_engine,
            &attempts,
            &found,
            &found_password,
        );
    }

    let elapsed = start_time.elapsed();
    let total_attempts = attempts.load(Ordering::Relaxed);

    // Print results
    if found.load(Ordering::Relaxed) {
        let password = found_password.lock().unwrap();
        println!("\n");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🔥 PASSWORD FOUND 🔥");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Password: {}", password);
        println!("   Attempts: {}", total_attempts);
        println!("   Time: {:.2}s", elapsed.as_secs_f64());
        println!("   Speed: {:.2} H/s", total_attempts as f64 / elapsed.as_secs_f64());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Delete checkpoint on success
        let _ = Checkpoint::delete(&cli.checkpoint);

        Ok(true)
    } else {
        println!("\n");
        println!("   Total attempts: {}", total_attempts);
        println!("   Time: {:.2}s", elapsed.as_secs_f64());
        Ok(false)
    }
}

fn process_chunk(
    chunk: &[(u64, String)],
    cracker: &Arc<Pbkdf2Cracker>,
    rule_engine: &RuleEngine,
    attempts: &Arc<AtomicU64>,
    found: &Arc<AtomicBool>,
    found_password: &Arc<std::sync::Mutex<String>>,
) -> bool {
    // Generate all candidates from chunk
    let candidates: Vec<(String, String)> = chunk
        .iter()
        .flat_map(|(_offset, word)| {
            rule_engine
                .generate_candidates(word)
                .into_iter()
                .map(|candidate| (word.clone(), candidate))
        })
        .collect();

    // Test in parallel
    let result = candidates
        .par_iter()
        .find_any(|(_base, candidate)| {
            if found.load(Ordering::Relaxed) {
                return false;
            }

            let is_match = cracker.test_password(candidate);
            attempts.fetch_add(1, Ordering::Relaxed);

            if is_match {
                found.store(true, Ordering::Relaxed);
                let mut pwd = found_password.lock().unwrap();
                *pwd = candidate.to_string();
                true
            } else {
                false
            }
        });

    result.is_some()
}

fn print_banner() {
    println!();
    println!("╔══════════════════════════════════════════════════╗");
    println!("║                                                  ║");
    println!("║    PBKDF2-SHA256 Password Cracker v1.0.0         ║");
    println!("║    Flask/Werkzeug Hash Cracker by Mikro          ║");
    println!("║                                                  ║");
    println!("║                                                  ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();
}
