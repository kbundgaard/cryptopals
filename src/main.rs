extern crate base64;
extern crate hex;

use hex::{FromHex, ToHex};
use std::ops::BitXor;

#[derive(Clone)]
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
    println!("\nAll the interesting stuff is happening in tests so far\n");

    // Stop compiler from complaining about unused imports that are only used in tests so far
    let text = Text::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let _ = text.encode_hex::<String>();
    let _ = score_text(&text);
}

fn score_text(text: &Text) -> Option<f64> {
    text.0.iter()
        .fold(Some(0.0), |acc, &x| match acc {
                                        None => None,
                                        Some(n) => match score_byte(x) {
                                                        None => None,
                                                        Some(m) => Some(m + n)
                                                    }
                                    })
}

/// Wikipedia and the internet archive to the rescue!
/// Space is really important when looking at letter frequencies.
/// Taken from https://web.archive.org/web/20170918020907/http://www.data-compression.com/english.html
fn score_byte(byte: u8) -> Option<f64> {
    match byte {
        0..=31 | 33..=64 | 91..=96 | 123..=127 => Some(0.0),
        32 => Some(0.1918182),
        65 | 97 => Some(0.0651738),
        66 | 98 => Some(0.0124248),
        67 | 99 => Some(0.0217339),
        68 | 100 => Some(0.0349835),
        69 | 101 => Some(0.1041442),
        70 | 102 => Some(0.0197881),
        71 | 103 => Some(0.0158610),
        72 | 104 => Some(0.0492888),
        73 | 105 => Some(0.0558094),
        74 | 106 => Some(0.0009033),
        75 | 107 => Some(0.0050529),
        76 | 108 => Some(0.0331490),
        77 | 109 => Some(0.0202124),
        78 | 110 => Some(0.0564513),
        79 | 111 => Some(0.0596302),
        80 | 112 => Some(0.0137645),
        81 | 113 => Some(0.0008606),
        82 | 114 => Some(0.0497563),
        83 | 115 => Some(0.0515760),
        84 | 116 => Some(0.0729357),
        85 | 117 => Some(0.0225134),
        86 | 118 => Some(0.0082903),
        87 | 119 => Some(0.0171272),
        88 | 120 => Some(0.0013692),
        89 | 121 => Some(0.0145984),
        90 | 122 => Some(0.0007836),
        _ => None
    }
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

/// Set 1: Challenge 3
#[test]
fn challenge3() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let plaintext = "Cooking MC's like a pound of bacon";
    let text = Text::from_hex(ciphertext).unwrap();
    let mut max = Some(0.0);
    let mut best = text.0.iter().map(|x| *x as char).collect::<String>();
    for n in 1..=255 {
        let candidate = Text(text.0.iter().map(|x| *x ^ n).collect());
        let score = score_text(&candidate);
        if score > max {
            max = score;
            best = candidate.0.iter().map(|x| *x as char).collect::<String>();
        }
    }
    assert_eq!(best, plaintext);
}