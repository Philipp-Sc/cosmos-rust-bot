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
    let secret: Vec<u8> = lc!("4a07101b6f6c9dde5da5d6f9b6235d993f95795f2ee05134319967276042b631964cf0df6b0700cacfed0e3cbe458f32024e9bcc3aa81b613474535a67521bafb658e2230c1f658390830391fdc2b58a6e3753b98cad435f9a8c53d988ff4bdcf76d1e697b842fa1e70db12b02c5ba9264fb7771ecd7c8af7754a01f2cc514ccfd58ba0ae91a008fec605550441ecd7d24a164347d836fba46139ca296869fc3e58b83e81223df901f1095152d1ead9aaa8a707817206ea12be6044363e27c65c45b42f8b7237ec055ba00f695160b876ab5f631970acf8a5fac47e3572a8966fc48e359bb0855f526913601cdc4e5b456e7050a8d4b8be4d8b7ef1447584651").to_string().into_bytes();

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