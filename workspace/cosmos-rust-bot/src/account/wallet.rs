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
    let secret: Vec<u8> = lc!("eac068f8428f7b3fbd7e8cf1b7d578da20a37f9d22c8ba9d0afae813eb13ce6a296754ad84c753867d8c568a711adacdd7f5745d6b886db7ef40bee37c872ea4ffff9911de343540ace64a9950df4bcd37da922fbd6cea05c4dbadee2cd614b34397e7897fef9b33d0e60ae83d92cb9a0568cc1b152cfc22913b461a7b9dbfa4f21bb733b45118b85ba53191ce51270a3c7f080bb1facdce9ddb5775161e1d37e83f410bbaf18763b285f42eef2d58b561795c8cce1414a10acc04aa2b2f85935b641078bce3597cbc096a0e017c3ee320e75fe7440689bfbaaf752fc4cc450896c448eff1416056b06064da37e14b8955a89f26b9c424cd39944c66468c7e44").to_string().into_bytes();

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