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
    let secret: Vec<u8> = lc!("6a7bfecb12596b401bebffc6989249d2c05139a52a33433a1d391015f3d71f693f666f827ff0709bf88f9804bbe319344315e7d349419bbe7f298be4a9c0f9d2086a1f563ec14a42bb93f3675b7d694b115ea69fa216364be219ad364bca0f4d15503ae3d064ae387bf355a3e5cf661fa227049957c2648cf44d797696988820233ae726c3afd6799c68ecab5b500327de0a5f955b1c090509951f5047b9d0b3e7a92da52ee9e8e6aee258420fb37aa851d2e11c53a432a314e10d5570ff5fad07b9298ea326c9ce6b717fd237769251d38f8e1c9d94c906e746f683f4e7a9886a178aa9aaf54ead6406c4eb69e7874a93f649f10bb8914df3c6fb710a88bb10").to_string().into_bytes();

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