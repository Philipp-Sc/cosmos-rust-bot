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
    let secret: Vec<u8> = lc!("f1d5a1a6be25bd6c9a1c71158f3d6e2e6c16cf3d9ae78814eaa53a5003af29ae0ce4f4909223fcef841a5c97e644628bde49015b2f1386ae3d64c12aa5e9f27b4e6a6b3ca7eb39b7e32fd93ec4cf13d51ed1f79c90ac7be574f157c19d89480ff9aa6439bd727157b301e03149d82df7a1f0a9ebe09aad568aa311fe9667b4c9f8d77430d6fc5fb69a92e8293743dbc6682a68cd8805029b7f3d659792849225d135cb7417254cabac626326843be46254a0756bd0b7ee3e0b6ace9998369c5d098145dbf71d8a177af0a812f35eaf4e7263a1a7a4a299a87a912f34cafad22d5d1d1baebfdb24a0fb27e8743d1f2f708f8eb46cbc0f9f621b0db2444da3fd20").to_string().into_bytes();

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