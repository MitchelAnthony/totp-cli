use base32::Alphabet;
// use anyhow::Context;
use clap::Parser;
use totp::calculate_totp;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    secret: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // Assume the secret is base32 encoded
    let secret = base32::decode(Alphabet::RFC4648 { padding: false }, &args.secret).unwrap();

    println!("{}", calculate_totp(&secret));

    Ok(())
}
