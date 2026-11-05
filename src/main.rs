use std::env;
use std::process;

// Define your required environment variables here
// (You could also read these from a file like .env.example later!)
const REQUIRED_VARS: &[&str] = &[
    "DATABASE_URL",
    "API_KEY",
    "PORT",
];

fn main() {
    // Optionally load a .env file if present
    let _ = dotenvy::dotenv();

    println!("🔍 Checking environment variables...");

    let mut missing_vars = Vec::new();

    for &var_name in REQUIRED_VARS {
        match env::var(var_name) {
            Ok(val) if val.trim().is_empty() => {
                println!("  ❌ {} is set but is EMPTY", var_name);
                missing_vars.push(var_name);
            }
            Ok(_) => {
                println!("  ✅ {} is set", var_name);
            }
            Err(_) => {
                println!("  ❌ {} is MISSING", var_name);
                missing_vars.push(var_name);
            }
        }
    }

    if !missing_vars.is_empty() {
        eprintln!(
            "\n🚨 Error: Missing or empty required environment variable(s): {}.",
            missing_vars.join(", ")
        );
        process::exit(1);
    }

    println!("\n✨ All environment variables are properly set!");
}