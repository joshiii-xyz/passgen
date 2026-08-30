use anyhow::Result;
use clap::Parser;
use rand::distributions::{Alphanumeric, DistString};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Password length
    #[arg(short, long, default_value_t = 16)]
    length: usize,
    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let password = Alphanumeric.sample_string(&mut rand::thread_rng(), args.length);

    if let Some(path) = args.output {
        std::fs::write(path, &password)?;
        println!("Password written to file");
    } else {
        println!("{}", password);
    }
    Ok(())
}
