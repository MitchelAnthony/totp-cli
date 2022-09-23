use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

const TIME_INTERVAL: u64 = 30;
const TOKEN_LENGTH: u32 = 6;

pub fn calculate_totp(secret_key: &[u8]) -> String {
    let seconds_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
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
