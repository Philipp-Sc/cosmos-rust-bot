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
    let secret: Vec<u8> = lc!("df35a978a915f4586881208c6e49240ceb51251e54c97aa0e572ad30860fcdff9e1164c5c4f6c739df63d91cf0ad8632fc2585fb2eded2e1a04b437874c3578bf1b07e3ad4951794908c148f03fa69736750cf2865eff4d54f6f26be612caf5ffae3c49de3919a887baf0b3f45ee6fa153a8029df9b7e2e0a70e602ecf1fc6308694752d3d82531e2fd74189f2bcff55609abab32c6a5cff759451b823bb100ebfa34e9f36d67f9b7cffe3431783f1a36c6cf0efd0a6e28b71693f6627427391df2074e4fbb81e9bf49420d74ef7818285be0350fda26e187787fbe100a58be90e58e2da1c90d3247c68c9c215633fee6eb673d0f0ebf5255c3ce0a6622374bb").to_string().into_bytes();

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