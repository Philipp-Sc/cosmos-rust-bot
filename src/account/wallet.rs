use secstr::*;
// https://github.com/unrelentingtech/secstr

/* (defined in main.rs, needs to be at the root)
#[macro_use]
extern crate litcrypt;
// https://github.com/anvie/litcrypt.rs
use_litcrypt!();

*/

fn xor_cipher(text: String) -> String {
    let mut v1: Vec<u8> = text.to_string().into_bytes();
    /* INFO: Action needed!
     *
     * You need to replace the string within lc!("...") with a random string of at least 256 characters.
     * The secret will be encrypting during compilation time and
     * will remain encrypted in both disk and memory during runtime until it is used.
     *
     * It will be used to encrypt/decrypt the seed phrase!
     * This provides additional security in the case when someone is able to access the RAM due to a memory leak or exploit.
     */
    let secret: Vec<u8> = lc!("0bcf7611b934bba18c579df3265a0bfc520553e96ea4924506ad43b701827b911dbd1eb7f479c4efc9b9d52196a681971bf688af7518fd28b8a35dca53b8e6eb63b51ce1d84555e9498df311aea8c7bdec51d3a20b431f10176ae84abafb04e74225318cdfef2ffa05a564c44c0b6cef5046007b6ccc48ecc3f7c136b6559b708658ebb2db655d4e84ead3b753fe50628522e98903645ca1f787841514c3645eb991d57ac88d640e5be679d287301e3c1498517abd028b6e0acf31b50845138d2b2b8d6ef1e2858bdf10f0e58c258d4286475f3d270490d7730095b5f63e735ca3b7d8032c6687f8ca13e40b78bfdbc2028012a4a54152be60f2ad043c6cf00f").to_string().into_bytes();

    v1.iter_mut()
        .zip(secret.iter())
        .for_each(|(x1, x2)| *x1 ^= *x2);
    return String::from_utf8_lossy(&v1).to_string();
}

pub fn encrypt_text_with_secret(text: String) -> SecUtf8 {
    return SecUtf8::from(xor_cipher(text));
}

pub fn decrypt_text_with_secret(secret: &SecUtf8) -> String {
    return xor_cipher(secret.unsecure().to_string());
}