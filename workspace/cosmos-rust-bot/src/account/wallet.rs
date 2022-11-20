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
    let secret: Vec<u8> = lc!("be76e8b1b71f0bbbd4f761e4f3f502bcd466c073f15f85aa7470c1abe2ff846a4af5560866585caa8c8fd8af7eb89046ae6ec467bd7375700ec8ac1a6271facc647736aedbb1f6bb410f5248835a64a5abcdb3ae468d82ccf04f0dcb19dd268b1b32eb8b0a970e30c8c6ba45c8d012a4f8090c6d4983c7245afb3405561719b9081b9d69888e6dc6deb1f7e136fb30214764bdd04f9597a2a421511aafcf979561f230d3bfa2d790f72685cff30cc0e5a8435d14f5b7f9e4a5fe5e0280422b00b304a84bd7e63e056fabdc9c83974bd18f1a87c49b6f04b26c4d1515350d1ef2cf5c2525d3710ef516fe2c8072a81be0ec73594369e38332a3e52d58b1e5eba0").to_string().into_bytes();

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