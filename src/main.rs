extern crate base64;
extern crate hex;

use hex::{FromHex, ToHex};

trait FromBase64: Sized {
    type Error;

    fn from_base64<T: AsRef<[u8]>>(base64: T) -> Result<Self, Self::Error>;
}

impl FromBase64 for Vec<u8> {
    type Error = base64::DecodeError;

    fn from_base64<T: AsRef<[u8]>>(base64: T) -> Result<Self, Self::Error> {
        base64::decode(base64)
    }
}

trait ToBase64 {
    fn encode_base64(&self) -> String;
}

impl<T: AsRef<[u8]>> ToBase64 for T {
    fn encode_base64(&self) -> String {
        base64::encode(self.as_ref())
    }
}

fn main() {
    // Set 1 - Challenge 1
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let text = Vec::from_hex(input).unwrap();
    assert_eq!(base64::encode(text), result)
}

/// Set 1 - Challenge 1
#[test]
fn challenge1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let text = Vec::from_hex(input).unwrap();
    assert_eq!(base64::encode(text), result)
}