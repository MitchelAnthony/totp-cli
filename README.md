# TOTP CLI

## Testing
Generate QR for testing: https://stefansundin.github.io/2fa-qr/

Or use this secret `7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2` or this otpauth link `otpauth://totp/John%20Doe?secret=7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2&issuer=Acme%20Corp`

## How to use
cargo run -- --secret={secret}
