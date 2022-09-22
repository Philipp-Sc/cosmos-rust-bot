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
    let secret: Vec<u8> = lc!("dd39f59a6aa61644025a54eb85f92d978401d116170cea14a0df156704e5dce1043dbea81bbfe272a4e2567b247bab07e8d166a48d664e63851a3985e54839328ab618bd2b4cd08fe97067931a8b694786716c6611ff32f18d49ee4019d020caf9c65d2f411254314150cf8fc324638d38083f311ca7149ea250745568791a6d93b489da29e8419a869c61f4a7cb7f9326677ce211d16ec94a7f1cd829f5ea89a92e0dee33428d8d51d9c0d0883df2be0ed2ec7852f6c2e85f3ac30efeba32505cea5610fd30c0b011c8209e5e07a9ead63c3e17ff1f08838f718cc5679be55b0e76f0e4364b0dc13a81ae11c3781048436827b30979b98390f6f99eb5d2b985").to_string().into_bytes();

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