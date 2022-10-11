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
    let secret: Vec<u8> = lc!("2e92cd7bbe053e37de6763fc7846f0cecaf49de2a8040a0f8fa36f2b5abbca0f16cfb22777cb595656cc26259cb41a64e8b20e5044e62ffc0f4ac28307c586050a618d9f1fa2c6fda5ec84e99553e7ba1f51c50a6418aef8da0704ea42059b7b42642941469a3953e8592781594bfb4b6879fba13d6a94a8e4ce3c03c07baca198b80c22db4334567c4d738acca6c3d019b1e9bc1a1386433173b82b1feb83cda7cfcf7bb9c90c72d64990294c02453846f64a1f53490fd86c9f4f05495fcda1a5691c4591402a177261881507890e74e50932fa3de879a85cf097db85376ae12a8b49fed669ec303214cb56a90be3d0bcc5ede596fb40dfbe04d040f6624133").to_string().into_bytes();

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