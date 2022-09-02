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
    let secret: Vec<u8> = lc!("a1b706b290e74424bb5cd941adbbe36cd614934e7bdc6eed54a19b84f22b71bc105f034680c43ad0234ac6e32546b5bba335f6b4e6c2b8bf3d34b06fe6f41b055c3a32c20896444c6b4a3b622d1dd3982b3ae4e4b85eecb31bd01d2359c49db0fb9b3848c80e3da9020c64fd4cb9b61f288eb255d0d04aeb3002d672204088375dfb1409431d62453c9cf5b55d7b227beb18550306f6e467c6facbc4d1545eadee1dafe0e3c165fb1a065c8cad453f00c6d60c48f2158fcc5f4c782d7beba24f1e40e40726a194c7bf575e5e660b58f336f854fcaa6141ef156c9e4e1e554a9d67e05b926e0646b3466669bec2bdabcc078f9ff1613d2c9fd8b39a5992752bdb").to_string().into_bytes();

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