use base32::Alphabet;
use url::Url;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TotpConfig {
    pub secret: Vec<u8>,
    algorithm: String, // TODO Use enum?
    pub digits: u32,
    pub period: u64,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            secret: vec![0],
            algorithm: String::from("SHA1"),
            digits: 6,
            period: 30,
        }
    }
}

impl From<&str> for TotpConfig {
    fn from(base32_key: &str) -> Self {
        let secret = base32::decode(Alphabet::RFC4648 { padding: false }, base32_key).unwrap();

        Self {
            secret,
            ..Default::default()
        }
    }
}

impl From<&Url> for TotpConfig {
    fn from(otp_auth_url: &Url) -> Self {
        let mut query_params = otp_auth_url.query_pairs();
        let otp_secret = query_params
            .find(|(key, _)| key == "secret")
            .unwrap()
            .1
            .clone();

        let secret = base32::decode(Alphabet::RFC4648 { padding: false }, &otp_secret).unwrap();

        Self {
            secret,
            ..Default::default()
        }
    }
}
