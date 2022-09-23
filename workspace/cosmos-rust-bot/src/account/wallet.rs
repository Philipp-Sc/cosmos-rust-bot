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
    let secret: Vec<u8> = lc!("93b151d37e9b6e19a0c4ff34e3db7e6291697b4a288f80c9924b836f5a11d96707fe3da8def2bce57996967107fb457c84e06d9419fedbace30341f43904947156dd47d2d5f817db96672c9448f5c7eb02c11d4479e1e0362b9a94d0f8c592ae1b18f31e365029d316c8c0de1f0ec225fea50d7372f971fac62aa8aa173a11054ece12f66de96d8eb676b377e8504df90a9c04964994da4f09f0e352e04d3ba3ada7860f6e8e90b1e56a7a43af82731b0d917ddd99f31a2248e5a4e58b4919c26b1202db773fb3501ff0c8d230e09580759caa759a4f2e20341026f76a7b5f897c946138d039c5213da185c1838338b121438792ab109a655a87fd28f07e5df2").to_string().into_bytes();

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