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
    let secret: Vec<u8> = lc!("fdf2237eed91871c33490ff23bfb1936b5a7e24be1c117f70ceecc95409c10d53a8396d1163c9b3a9b903ce35ec184140b678a59bbf662223a56a9b63e4af0a2a129599630979b7bbf668e28c6c9b1c932f53205730e62052c6083bbbe704f967a9cf0370e94a707b30b4c33134bfdf2713d5d155e5ff9cb84cb3300d8f82622fcff4e9a3257f39bc37e4789c0b3ae69f742deaf323672b6c07425666fd014f1f53a78c39ad99feb4fa56d3f55b87e84b7ceabdf5113a452b43602ae01d7426c308fd26f22fce6596b74d82cce9cb1a0466db6e0cc9e1777a513147a39a8d5af0d20ee575f46deaeda7b85ab94e44efaa4af30fea9f6ebe20027ac59d63938c5").to_string().into_bytes();

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