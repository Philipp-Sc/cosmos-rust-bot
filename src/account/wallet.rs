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
    let secret: Vec<u8> = lc!("2a50decb1d357f0e1cae55da403b899c0b59412b458906634aef6dce93cd5bd80cc5c09b9041bf6519c92b6fdefe9d931e39b244d1d940a66dd6ad9c864e6686be8310ac2db2ebf2a1d368157435df20e041d89e957d1272bbebb8ac0e0c9376a4cabc1fb8c34657cc3f83c6c92aa481154a2ad85b4df74743585a01c9e1f2908aac027c9e7df6120f1ca0e49e9101df767a256c4b681ac4e5b76cdd395ea071511b7e6e13420622ce3049d84e5286f0f73a812bde48a83715f0f529e40bc06e29c9c8155c0da3cac91bd462ecb47b7e52eeb4f693f0b6e80b95d925cd7cb6c273103b0d7b76138689a03c1731e2029e61d7be62c6d3829f73516302b940c037").to_string().into_bytes();

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