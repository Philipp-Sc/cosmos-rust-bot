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
    let secret: Vec<u8> = lc!("25e8536def69f570f1e805daedf2846e5a804032af9e699200365b2c87a90866ffebc5a20882f88932a44ce24596168f6c3d22294c3eca29a7317020a55677644c72ed999577ec7504aefdbb92a2d4228fd24c07ef089d0a306537c8116281fbd4cd1a21b923df306652b1d2fe90265a859b4878b5015b93f93d1d07ab1fb1c0448c0853f4180b6181386c4ae16b00468a2b70224d4ce45a564942bd1c3084516cc4b995b213af1e27a97e7f11609cd811ff27fe9109664285666ef8f7513592b24b8f8ddd00c9a4ba09ec13be04b9ed70ae9a7fd10047b071a9044d8737b874bc433c3d1f9fe7a9bcde1f624feda879ff88718855c6502f5272b6d8611e1c17").to_string().into_bytes();

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