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
    let secret: Vec<u8> = lc!("feaf779209c02eac41f3633d2480899a7092a72dc661eb7b2ac275c477287cde92f0931e7a6cad18e2b88050a981aa8113721e1b514116abfe9ae55251e450d42dc022d4e8b5e2350446b48e9d2087cc2d6871c0112e60c30b18e8bc8ecf2276cec2ae497642414284cae5ffb5a4d4cc85b4d7c507e451eba0873190e09fd7d705e7265047186c921898f79578b5783336ce101b1fa40e26ae7c607884173072ae39726050530aee4caf09eb536d03db05e0c791e8293bc765b2c6e00b24f726289f88c480511ef7c1968d2cc293e1f2f0d044954a1da25543c38d86f91a235ac23ba16891347b3eb3f364ac7cf66671a993a76dd9e66f00d43473a9d4f1256f").to_string().into_bytes();

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