extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha1;
use self::crypto::sha2;
use self::crypto::ripemd160;

pub struct CryptoUtils;

impl CryptoUtils {
    pub fn ripemd160(input: &[u8]) -> [u8;20] {
        let mut ripemd160 = ripemd160::Ripemd160::new();
        ripemd160.input(input);

        let mut result = [0u8;20];
        ripemd160.result(&mut result[0..20]);

        result
    }

    pub fn sha1(input: &[u8]) -> [u8;20] {
        let mut sha1 = sha1::Sha1::new();
        sha1.input(input);

        let mut result = [0u8;20];
        sha1.result(&mut result[0..20]);

        result
    }

    pub fn sha256(input: &[u8]) -> [u8;32] {
        let mut sha256 = sha2::Sha256::new();
        sha256.input(input);

        let mut result = [0u8;32];
        sha256.result(&mut result[0..32]);

        result
    }
}
