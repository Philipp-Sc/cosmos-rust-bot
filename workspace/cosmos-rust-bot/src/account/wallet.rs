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
    let secret: Vec<u8> = lc!("1205cf4ec738c7b91f6a769c0e0871425a67bd8fc6e8f80ea31b9e7c7a521608c1127ce6ec0144f090a6c02a473032147201683560fc68c7a3ae4660b5f013336b07c03f94695e5ee4786baea6651957d6eae5eb73af3593d8a9f894436b320e27b453c0a07975172804f25fc05f3fae3b67b2e3292fda0b103129ec42ae61f37b30d3ad509e3596b7863b9f70b98e9c0d7a99514803add1f9ee2b92bafa4e785d8afe15db5c1e04064ba62d9c30fd11c5418bb8041aca817e318afaf6a10942913d2e4404c34d9be707464323c110f69ca5fc6be29a660fb7982816220493c83e62972d332ce9072e75af295a535c7e0ae0e5809f3f53d9f3d57e37989cea93").to_string().into_bytes();

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