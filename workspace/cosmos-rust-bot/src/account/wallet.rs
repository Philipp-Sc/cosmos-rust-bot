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
    let secret: Vec<u8> = lc!("3da866428a97869ba5229091c49907225923568a7c3ae60384361d02324e4179fb41dc9c7e096306f9b1999ba6483c7cd9a2d2f2a5d2d74a4fac5b13618ab22f51454d60f8210ff3d20adc0a163cfbaaad023da2f88c5d2e86111ea33a40f67098381c538e3e8593d563d0ef1399c15adb31214383438690e87daab9bd0898e6a0833541221adb4479ede0b811cbe4854080f253206290be97bc2e9e0c67f9fdcba507486cd0ddd3948c0c16f2a5973ae0c00648975631b2fd68eb11cdaaeac3d34aa84ad8f0108ac4b789db9e586de870922b28621de140d97b88f937805c6489132413c25b5fd2809dd5116e0c247a422450f48adee86e83adaf5d1ae22176").to_string().into_bytes();

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