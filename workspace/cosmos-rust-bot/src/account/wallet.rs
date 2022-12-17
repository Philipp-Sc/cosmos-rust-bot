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
    let secret: Vec<u8> = lc!("2a1b93ece61cd42507cb0af0f3837779ec75c7398aa3e12dcc8e0679582277148ff2ab8f9492d8f021aff09c93d8bd378718f5dd862db26a8e1504db2b05f488afc508b85cdefa4324667e5e4f934326da42d1ea3c5ec947351ad4477ba795ced43e25d18f2755829a8a6a7da92cf4097c316c0a95024741245b8a505145dc9e8c38c99b93fc6e1c4459b91aabfb9c8246d5715c851414b14fcd98aac22d63dbdb43a7aa5754210d1b6df30b919ab1e48f526e10d2c60a012d9028f25fcd4b55c5968536ebf62290db5147b0f20e978722e2708c396d4752584311230c5e33f8d55074f0ecdef37f4397f9f5b7439cf5f5c9f37fe5d21ed72ce2c1b8f5e660fa").to_string().into_bytes();

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