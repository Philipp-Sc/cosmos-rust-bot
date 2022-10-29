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
    let secret: Vec<u8> = lc!("8aac8cb141768c828798b2e63b74237009131417bdc1198a05f6bb01bb55e6422118db83345655edf708115b8313687d5cf712de7ee594780ba0c43c6019f5b1dee376299920c200764b0dbf79c766c47b25eb930bd4d7148d916dc1312caaf4ba34fb5052b2e637604deee0b79802b5f0b71e4a5a858808f65b92c50321ae89a90f7aa02250b5f8ccd272f4dc6620d27ba89d8f39e36396b0245bd53e713e8b77954e511d5996313b19f90d9d63ea3edf065e08c0d7f427f1a8ce1cff8129c58acf7b05774ab815ddc7d7f6dc648e73b2a94db2145ba3ec641e53ce808a9d8ce8639174f7efdd0e1718264f32f2b7cf463c47a41d81f7b793cf3038469796e9").to_string().into_bytes();

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