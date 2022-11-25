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
    let secret: Vec<u8> = lc!("e46566503ad72a6dcbd5e8a986ad900cecab96f8d9309cfcead2a03e62afa63690e98a2546c6152b4632ed2a6ab17bf5b71a9edbac1f4c6944147046c4b9a519ff8fe0ee5c422c861bc03046391dcccc8cfcd90dbc024e8ad659b7c09ab5835388840ae7e2d8b43260b5126efeb97a47a54c23030cb1ff7b926cde43169c4f8ceb41c345197406559d6e90794ac8c40ccc5916d10aa83ceba793695cb029e3f1cdf384f5a207022b2cc7e16d5fd0af54874fa7d389044b9f377b2867fa4205f146f1e9e5cc4edaf6c1acbf77c23fdcbf18c3683e8c2123a56e3b1a00663587196a14da74e7a2b32d7a1a3f24db17188b0a5b2be0cd70ee019aa5d43a388eaaa1").to_string().into_bytes();

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