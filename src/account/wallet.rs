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
    let secret: Vec<u8> = lc!("c775e1eaf547dda49cc4c6e38f96113332cf0f111c17eb816cd2f9576c0bd5920ce31fb064ac0acf8fa03f76a4012cec014b76fb3e7f797d31e7bf0c5ecc25018377aaad71077bc01003ff912777582843258f11353d5dcb6dd4b2a147b9366a220e9fe06b42b96b98aa1bab1101b5ddbecfc54a6888bc9505867e926f5dbea30152f8ae14b869e1251363ff2ac9cb69dfe12c1d81d162af20bc3b7603d3fad86436ad743b8d45d7eff29aadb78854ac721bd1dca5a31308cfd79ee5e47cd39f8a1c94b1c9e11d02513043089e0eee709c573e4e29e551d43a4f317f025894f49b1a06435fc36ccbecb94b596e2ab2ef9abc8ea9601c60774a85a95618032689").to_string().into_bytes();

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