use anyhow::Result;
use clap::Parser;

/// Say hello to someone.
#[derive(Parser)]
struct Cli {
    /// Name of the person to greet.
    pub name: String,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
    Ok(())
}
