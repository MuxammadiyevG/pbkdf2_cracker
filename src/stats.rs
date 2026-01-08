use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Statistics tracker for cracking progress
pub struct CrackingStats {
    attempts: Arc<AtomicU64>,
    start_time: Instant,
    progress_bar: Option<ProgressBar>,
}

impl CrackingStats {
    pub fn new(total_candidates: Option<u64>) -> Self {
        let progress_bar = total_candidates.map(|total| {
            let pb = ProgressBar::new(total);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec}) {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
                    );
                    pb
                    });
        Self {
                attempts: Arc::new(AtomicU64::new(0)),
                start_time: Instant::now(),
                progress_bar,
            }
        }

        /// Increment attempt counter
        pub fn increment(&self, count: u64) {
            self.attempts.fetch_add(count, Ordering::Relaxed);
            if let Some(ref pb) = self.progress_bar {
                pb.inc(count);
            }
        }

        /// Get total attempts
        pub fn attempts(&self) -> u64 {
            self.attempts.load(Ordering::Relaxed)
        }

        /// Get attempts per second
        pub fn rate(&self) -> f64 {
            let attempts = self.attempts() as f64;
            let elapsed = self.start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                attempts / elapsed
            } else {
                0.0
            }
        }

        /// Get elapsed time
        pub fn elapsed(&self) -> Duration {
            self.start_time.elapsed()
        }

        /// Set progress bar message
        pub fn set_message(&self, msg: String) {
            if let Some(ref pb) = self.progress_bar {
                pb.set_message(msg);
            }
        }

        /// Finish progress bar
        pub fn finish(&self) {
            if let Some(ref pb) = self.progress_bar {
                pb.finish_with_message("Done");
            }
        }

        /// Print final statistics
        pub fn print_summary(&self) {
            let attempts = self.attempts();
            let elapsed = self.elapsed();
            let rate = self.rate();

            println!();
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📊 Cracking Statistics");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("⏱  Total Time:     {:?}", elapsed);
            println!("🔢 Total Attempts:  {}", attempts);
            println!("⚡ Average Rate:    {:.2} H/s", rate);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
}
/// Clone-able stats handle for sharing across threads
#[derive(Clone)]
pub struct StatsHandle {
attempts: Arc<AtomicU64>,
}
impl StatsHandle {
pub fn new(stats: &CrackingStats) -> Self {
Self {
attempts: Arc::clone(&stats.attempts),
}
}pub fn increment(&self, count: u64) {
    self.attempts.fetch_add(count, Ordering::Relaxed);
}
}
