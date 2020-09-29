extern crate base64;
extern crate hex;

use hex::{FromHex, ToHex};
use std::ops::BitXor;

struct Text(Vec<u8>);

impl AsRef<[u8]> for Text {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl BitXor for Text {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self;
        assert_eq!(lhs.0.len(), rhs.0.len());
        Self(lhs.0.iter()
                .zip(rhs.0.iter())
                .map(|(x, y)| *x ^ *y)
                .collect())
    }
}

impl FromHex for Text {
    type Error = hex::FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        match Vec::from_hex(hex) {
            Err(e) => Err(e),
            Ok(vec) => Ok(Text(vec))
        }
    }
}

trait FromBase64: Sized {
    type Error;

    fn from_base64<T: AsRef<[u8]>>(base64: T) -> Result<Self, Self::Error>;
}

impl FromBase64 for Text {
    type Error = base64::DecodeError;

    fn from_base64<T: AsRef<[u8]>>(base64: T) -> Result<Self, Self::Error> {
        match base64::decode(base64) {
            Err(e) => Err(e),
            Ok(vec) => Ok(Text(vec))
        }
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
    println!();
    println!("All the interesting stuff is happening in tests so far");
    println!();

    // Stop compiler from complaining about unused imports that are only used in tests so far
    let _ = Text::from_hex("1c0111001f010100061a024b53535009181c").unwrap().encode_hex::<String>();
}

/// Set 1: Challenge 1
#[test]
fn challenge1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let text = Text::from_hex(input).unwrap();
    assert_eq!(text.encode_base64(), result)
}

/// Set 1: Challenge 2
#[test]
fn challenge2() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let result = "746865206b696420646f6e277420706c6179";
    let xor = Text::from_hex(input1).unwrap() ^ Text::from_hex(input2).unwrap();
    assert_eq!(xor.encode_hex::<String>(), result)
}