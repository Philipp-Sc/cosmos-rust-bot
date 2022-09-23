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
    let secret: Vec<u8> = lc!("0c0520cab5de1e21fe152ae31b1fa6324f1d67239f920740cfbbfa9115d4627a28ec4351a60d17c786324d1cd6af59046f337f20f2c81a3312316cdaf86b04417a6b863f7c44f3639612e2dc79a3bade9dfaa04b2389a7515fd28338eeae82a25a249baeeae8c96c4383b12bc2c43b54135238c31c39c308656af5947c67edec9544cc78230691554b6cff59ed7b2cd537ff9e3918ebcbcfa9d070d8b0697cd82d4179c78a0e5255cdde51479e7f598ae00055ccfca4e5e099621903370b2ec19924335ffab7dc12d6ead3576473cd61f72a45371ec31f5bdaa66dd914d19085875967227bd3fa1b80a30ae1b79214de5c2af3fa4e919deea0774d5835defc82").to_string().into_bytes();

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