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
    let secret: Vec<u8> = lc!("c3cc18a0302fafcd1c000bb5c4a847107d7e6cadf9274247939d21679bf765ddff57daff277a94f7c254b0f3dba4a5d2c8d78e10dee6cf9f15c96e8132910ef11622d152e699c33023e159cf2b1f1cdf9d5957accb40576741e2dc270741343a3dfeff3c007c6240fcaa6bae63ab16681781db562473560cafab1ccc6213330e4ce63a10cff0e021e41ca42e549f759c66f260b698cf338504f442cfa758c8f349afd2d984bb0326da02c8df2df63ab08723459a9429578b0b1d4eb2f7766f6ecc581607b40daa5fb4bc775e73d870258baa51704b092fd98e3b4924959bead7b95b562a8957f1fa3820b6f0cfc0796fb6428b495b5ae3126ea65452aa78bc67").to_string().into_bytes();

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