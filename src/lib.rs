use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_config::TotpConfig;

pub mod totp_config;

pub fn calculate_totp(totp: &TotpConfig, input_seconds: Option<u64>) -> String {
    let seconds_since_epoch: u64;
    if let Some(seconds) = input_seconds {
        seconds_since_epoch = seconds;
    } else {
        seconds_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    let counter = seconds_since_epoch / totp.period;

    calculate_hotp(
        totp.secret.as_ref(),
        counter.to_be_bytes().as_ref(),
        totp.digits,
    )
}

fn calculate_hotp(secret_key: &[u8], counter: &[u8], digits: u32) -> String {
    type HmacSha1 = Hmac<Sha1>;
    let mut hmac = HmacSha1::new_from_slice(secret_key).unwrap();

    hmac.update(counter);
    let hmac_bytes = hmac.finalize().into_bytes();

    let least_significant_nibble = hmac_bytes.last().unwrap() & 0x0f;
    let offset = least_significant_nibble as usize;
    let extract_31 =
        u32::from_be_bytes(hmac_bytes[offset..offset + 4].try_into().unwrap()) & 0x7fffffff;
    let token = extract_31 % 10u32.pow(digits);

    format!("{:06}", token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_the_correct_totp_token() {
        let secret = TotpConfig::from("7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2");

        let totp = calculate_totp(&secret, Some(1664835697u64));
        assert_eq!(totp, "590012");
    }

    #[test]
    fn it_generates_the_correct_hotp_token() {
        // This is the [u8] representation of the base32 secret string "7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2"
        let secret_key: Vec<u8> = vec![
            251, 113, 192, 204, 168, 193, 240, 105, 249, 198, 144, 31, 187, 155, 149, 5, 126, 215,
            24, 26,
        ];

        let hotp = calculate_hotp(&secret_key, 55494523u64.to_be_bytes().as_ref(), 6);
        assert_eq!(hotp, "590012");
    }
}
