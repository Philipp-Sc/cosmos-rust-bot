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
    let secret: Vec<u8> = lc!("de5d9687a1d1989fd7668bb6cb0342d39585c628030f4f77896284a95abab93e5adbbc3184f8665c8e21372f91485d00e29560b3b7c09ec74e19d9193fc843a164ba83c57aecdd2e7e8f0ae14893d9321673b74717f03caaba82e5bae174152595f27285667d9b80d872ce290f6f61a188e5944be5e9e771eba653a47c368e7d89be1efb071f361cbaeeb40634c5b1fc804f54a49b8ad6b4ef4a31196da9d66a5569164454b7ed6063c3c223fc9e49fc26ab818d8f0a3bd301be65c240ae6b43de38f95f8faf141066edff7e0ec1df85c581afe0a6fdd3e7eb388b7e2cc2a349cfa072484bed03fa20881c4044ab420eb2ae2ddea0571735a1680699e85b476a").to_string().into_bytes();

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