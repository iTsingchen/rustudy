use anyhow::{bail, Result};
use clap::Parser;
use encryptor::password::generate_password;

/// A sample password generator for nay account
#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Args {
    /// Seed to generate password
    #[clap(short, long)]
    seed: String,

    /// Length of password
    #[clap(short, long, default_value_t = 16)]
    length: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // 种子不能太短
    if args.seed.len() < 4 {
        bail!("seed {} length must >= 4", &args.seed);
    }

    let (seed, length) = (args.seed, args.length);
    let password = generate_password(&seed.as_str(), length);
    match password {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }

    Ok(())
}
