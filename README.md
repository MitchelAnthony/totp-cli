# TOTP CLI

Generates [RFC 6238](https://www.rfc-editor.org/rfc/rfc6238) TOTP tokens (based on HOTP tokens from [RFC 4226](https://www.rfc-editor.org/rfc/rfc4226))

## How to use
`cargo run -- --secret={secret}`
`cargo run -- --auth-url="{url}"`

Or build the binary `cargo build --release` and then `./target/release/totp --secret=={secret}`

## Testing using a known secret
Generate QR for testing: https://stefansundin.github.io/2fa-qr/

Or use this secret `7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2` or this otpauth url `otpauth://totp/John%20Doe?secret=7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2&issuer=Acme%20Corp`

## Testing the application (dev only)
`cargo fmt && cargo clippy && cargo test && cargo run -- --secret=7NY4BTFIYHYGT6OGSAP3XG4VAV7NOGA2`