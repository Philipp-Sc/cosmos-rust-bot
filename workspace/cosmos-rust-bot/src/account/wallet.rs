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
    let secret: Vec<u8> = lc!("b8dbd05301972219f02088f014bf666bf6cbe8dd380489dd504484db411e4839070787e743ac268fa6a41d0f4d51ee6dbea36237ed6ad23f342be891c71506e44c5cd1bf3f190ffb956cf832d71952ca00aee2894f341a91d9b0a0c7e4e0c9f04354051f46dd54ffbcab4ef43d387abd910b2b6d730ec23de217b6f4ac7e15336198db22a1af9510871457299c9caaf3cc9411712f17052cb22bbdab9a43fef044954d83a8908053028be775ec02efcb1ffc15891c2324c052e68702336bae86db1d0cc5ca4d81e3dd37948e6a085ee8e5212bb1bb941e3c2b6446f687bfc5a33e74e8b56a8b99d1b54892b8588dd642eea24d561d6b578b63150c32b307b93b").to_string().into_bytes();

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