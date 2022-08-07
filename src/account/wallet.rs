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
    let secret: Vec<u8> = lc!("8590543cf1a4c9fd7c45d05bd259522298f6f068836683ba1458482f820eab68c1c1b0d8342fb8e21ad691844610058a55582dec76bb2ff15219ee79f03c193600758ecb4a752e58147b5bb4748fa539f25e3b8c08d01197da48f10166dcfbf533863c93a4da71fb2e29d2a7a422ff9c0470d29b5baed99a56ccf73406a112e6f9745b9e28b2aa92d9ddb8486db86ef525e078a969bc8fe6add5e9502750fbad249b8e62aa286609804bcdff39f28a728b1c1e0c45bac112559a05b18bf5e786d435b105d6c2f271e6cb8156c038aa7109c07f8320c927ad84553aee81d5d8146d4779a39f9830b14c6b8b3a6318082a1f5201bb3dc490972cf1981966537bf9").to_string().into_bytes();

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