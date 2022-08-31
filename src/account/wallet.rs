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
    let secret: Vec<u8> = lc!("0f629a70fbaff31cc39c4f54f613a9e0025df7d1cd180c802230bc91a90f27f589c049c8a74e0382b9f6ebaa891bd0a0fa060ca9e41ac229d96ab2860ac93bb1adffc2595fd9862db77dd96b92eefdb9921afb8bae63c307a8526967c63569540adffe101adf960eb64d0a3cd44e466cf49846a7b92f8356f4821ecdcf24e26a15bd9ebf097828b1b535bad895280593263679b7044501ebdc7d32522325e2123c4df8524bcc391152e4d269cfd31cd03d8911940a3b12f834229ede628d4ee543f413c22127b79cae301a8b68b3920010e4d964b58e9d4bb8d293dd21e560b0c87212b275ad97fc78e5117ec4f5cc2d53d37c73d20d4ba5991a12abbb3ca018").to_string().into_bytes();

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