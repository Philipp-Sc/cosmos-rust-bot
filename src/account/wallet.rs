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
    let secret: Vec<u8> = lc!("ced86e5f98ab150ca0336c8ad65f21513340fb88f765fb3ef47f126e63ea35fddb105d5a77e549f3d152e5f06684117197fdbe67632f057d45fb7359184c798b5be7e9e9550528c23ed4b67f725ee09d2bcdd3341d346d344667d51f6616483ae12cc0e18eaebd4fd8324e15c6d444259e62a806a0351133190fbae5afdbfe2f643d275c437294dddb6c550f5b09f6766898990baa6d36c43a1702e8b84cb5b1ce09b74da49368f925cfa73ac377cd9f3020cb6f13abaff9e0227644d6f3a4e1d399d4c54b3c540f91436ad16c595a535fe5d886bf88c4a7248edfc82a37d792958c117cf100b3cdf8c34a55a9a19c2957c8a5eead3af531a9ffc0ac703126e1").to_string().into_bytes();

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