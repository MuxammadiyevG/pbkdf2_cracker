use crate::cracker::Pbkdf2Cracker;
use crate::errors::Result;
use crate::parser::ParsedHash;

/// Verify a password against a hash
pub fn verify_password(hash: &str, password: &str) -> Result<bool> {
    let parsed = ParsedHash::parse(hash)?;
    let cracker = Pbkdf2Cracker::new(parsed);
    Ok(cracker.test_password(password))
}

/// Verify and print result
pub fn verify_and_report(hash: &str, password: &str) -> i32 {
    println!("🔍 Verifying password...");
    println!("Hash: {}", hash);
    println!("Password: {}", password);
    println!();

    match verify_password(hash, password) {
        Ok(true) => {
            println!("✅ SUCCESS: Password matches!");
            0 // Exit code 0 for match
        }
        Ok(false) => {
            println!("❌ FAILED: Password does not match");
            2 // Exit code 2 for no match
        }
        Err(e) => {
            eprintln!("❌ ERROR: {}", e);
            1 // Exit code 1 for error
        }
    }
}
