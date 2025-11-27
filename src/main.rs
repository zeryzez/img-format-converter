mod cli;
mod server;

use std::env;

#[actix_web::main] // Keep this so server mode works
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Scenario 1: Not enough arguments
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    // Scenario 2: Server mode
    if args[1] == "server" {
        return server::run().await;
    }

    // Scenario 3: File mode (CLI)
    if args.len() == 3 {
        let input_file = &args[1];
        let output_file = &args[2];
        return cli::run(input_file, output_file);
    }

    // If the command is not understood
    print_help();
    Ok(())
}

fn print_help() {
    println!("--- Rust Converter ---");
    println!("Usage 1 (Web Server):");
    println!("   cargo run -- server");
    println!("");
    println!("Usage 2 (Direct command):");
    println!("   cargo run -- <source> <destination>");
    println!("   Example: cargo run -- image.jpg result.png");
}
