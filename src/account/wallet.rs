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
    let secret: Vec<u8> = lc!("843fc14e0ec2b7d450475eb071f8ef312313fe5dcf541fd254ab073c22258018be5e19237a4e57d10683976a499fc0392de107d37b23062d0ea48780c636e19f5e4317d2e697ab3a182775ce888820469cea1101695d82cff17e50daa9a0b9171a21b215ab5542b7c6648d3cfdac965e7af5d2a221623d151ce96e1d47a93843abfca91fbcf6eb426c0559d124af66f2ec7169ebe8e501672f364f825cbaca348736229bcf859901a68b69b60496802446fab6da8495fe12bf6e5d510e85e29460acb839279be3630f123767c60c6c3f2505d64856a0ec845038da9c31050d8b6bee13f0ae6f3ff438261d36e6082208d2ffa7000bb1c5bc74db8b98459bead2").to_string().into_bytes();

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