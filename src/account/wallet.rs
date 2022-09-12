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
    let secret: Vec<u8> = lc!("b18c2709c1f09a3ed7751f0f910274a22f095534f991e086ddb0b35257425169195328cb4ddb20dadfdb6636c70a2b859bdd6d2aaf9040acd16e0415c584b194a1b6ef8805fd2374fa264cd5dd776663328236aa722517fa9c96f2734bae910de0442ac5ced7ed094828e155122d20ea43373df52d4bdb9ea598ab74aa15b8cf32faf97b9074627e5be24972641a32ba542c9732423db4c07f8aaf62e90674ec8ad27d4049a5ced5164609874ea0ce8954ca2e63a67a5dba8665a930cdde93158c6ba39e18e3923300599a7519b08f735cb1e83bc1c628706e6f9c33139f525b000d5d23c17fb728e2772b778dd60ce57cf5eaa8ac2eee16c93e58006940c6d3").to_string().into_bytes();

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