use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet.
    pub name: String,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
    Ok(())
}
