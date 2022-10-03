use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

const TIME_INTERVAL: u64 = 30;
const TOKEN_LENGTH: u32 = 6;

pub fn calculate_totp(secret_key: &[u8], input_seconds: Option<u64>) -> String {
    let seconds_since_epoch: u64;
    if let Some(seconds) = input_seconds {
        seconds_since_epoch = seconds;
    } else {
        seconds_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    let counter = seconds_since_epoch / TIME_INTERVAL;

    calculate_hotp(secret_key, counter.to_be_bytes().as_ref())
}

fn calculate_hotp(secret_key: &[u8], counter: &[u8]) -> String {
    type HmacSha1 = Hmac<Sha1>;
    let mut hmac = HmacSha1::new_from_slice(secret_key).unwrap();

    hmac.update(counter);
    let hmac_bytes = hmac.finalize().into_bytes();

    let least_significant_nibble = hmac_bytes.last().unwrap() & 0x0f;
    let offset = least_significant_nibble as usize;
    let extract_31 =
        u32::from_be_bytes(hmac_bytes[offset..offset + 4].try_into().unwrap()) & 0x7fffffff;
    let token = extract_31 % 10u32.pow(TOKEN_LENGTH);

    format!("{:06}", token)
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is the [u8] representation of the base32 secret string "7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2"
    const SECRET_KEY: [u8; 20] = [
        251, 113, 192, 204, 168, 193, 240, 105, 249, 198, 144, 31, 187, 155, 149, 5, 126, 215, 24,
        26,
    ];

    #[test]
    fn it_generates_the_correct_totp_token() {
        let totp = calculate_totp(&SECRET_KEY, Some(1664835697u64));
        assert_eq!(totp, "590012");
    }

    #[test]
    fn it_generates_the_correct_hotp_token() {
        let hotp = calculate_hotp(&SECRET_KEY, 55494523u64.to_be_bytes().as_ref());
        assert_eq!(hotp, "590012");
    }
}
