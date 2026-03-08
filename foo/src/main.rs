use base64::prelude::*;

fn main() {
    let decoded = BASE64_STANDARD.decode(b"SGVsbG8gTmFlcnNrIQo=").unwrap();
    let msg = str::from_utf8(&decoded).unwrap();
    println!("{msg}");
}
