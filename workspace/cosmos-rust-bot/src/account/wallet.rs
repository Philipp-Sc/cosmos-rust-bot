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
    let secret: Vec<u8> = lc!("ed0ee091f94493fa1bf20173992202cf5c7368bdc2b0466d3fb5de795c12325c756e662c997a05398bd80f1fca29f27d273a828749defd7a641d0a955fd0ae00236618fe05f22bdcdabe662fc7d858828df0065a0c33fa27eb55706e2e09f8fe92e4ef66a8ed79abb3eae9c0c07a111fedbadbc9b79a560b5bfb6b5969869bbe191b31f941119c0a5cb337f342cbd58e974b157fc3ba0f182cea5b8f75bdbcc31ae43a10bb5e675f5e3e27c27798ee020f3e63e2129159a47d73bc1e258924284e056da81cf0cb215de361516ca066e088f27aa99f52e26196b8c5563d529cf615812c7dc7362200b86f62c84c006eaab46cf7af4ca9875637849137eeded615").to_string().into_bytes();

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