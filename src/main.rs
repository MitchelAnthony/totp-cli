use anyhow::bail;
use clap::Parser;
use totp::calculate_totp;
use totp::totp_config::TotpConfig;
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

    let secret: TotpConfig;
    if let Some(arg_secret) = args.secret {
        secret = TotpConfig::from(arg_secret.as_str());
    } else if let Some(auth_url) = args.auth_url {
        // TODO Should this be handled in the lib?
        let otp_auth_url = Url::parse(&auth_url)?;
        secret = TotpConfig::from(&otp_auth_url);
    } else {
        bail!("Either --secret or --auth-url must be given.");
    }

    println!("{}", calculate_totp(&secret, None));

    Ok(())
}
