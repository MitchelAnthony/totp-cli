use anyhow::bail;
use base32::Alphabet;
use clap::Parser;
use totp::calculate_totp;
use url::Url;

#[derive(Debug, Parser)]
struct Cli {
    /// A base32 encoded TOTP secret string
    #[clap(short, long)]
    secret: Option<String>,

    /// An otpauth url with a base32 encoded TOTP secret string
    #[clap(short, long)]
    auth_url: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let secret: Vec<u8>;
    if let Some(arg_secret) = args.secret {
        secret = base32::decode(Alphabet::RFC4648 { padding: false }, &arg_secret).unwrap();
    } else if let Some(auth_url) = args.auth_url {
        let otp_auth_url = Url::parse(&auth_url)?;
        let mut query_params = otp_auth_url.query_pairs();
        let otp_secret = query_params
            .find(|(key, _)| key == "secret")
            .unwrap()
            .1
            .clone();

        secret = base32::decode(Alphabet::RFC4648 { padding: false }, &otp_secret).unwrap();
    } else {
        bail!("Either --secret or --auth-url must be given.");
    }

    println!("{}", calculate_totp(&secret, None));

    Ok(())
}
