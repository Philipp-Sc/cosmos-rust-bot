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
    let secret: Vec<u8> = lc!("4a6d56278d7c237622e00caba17145eff6f7291ec52a23dce0f39bbcd241c7aab326b57b82a87480416e8617897ea3635e6fe82e8c298ef4982f5344c60f71ba99fbbb95453854e0aee06b7ea13885746815ddf5e899bf0b7a89c55d420c226d0db6bfc8821a1b7c8eee9738925932c29289e9b3fdf6ecc3d7c622c024f3a9f253224320644e17a9eb7902d8b4c3d08e5c0bccb7251c1b7875162be8b033a310e79cbd6c68ddaad560343cd90047e5fd4b275900100bf9ddf03401795e4e1e863344e9599b921ad3b00e542d1b6189f02b05afb3d9efb482839bf64a641cb737aaac4ae0081cbe27c37f3a5c3544bec2fe90d09810e5f2c262f22ad46d210b30").to_string().into_bytes();

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