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
    let secret: Vec<u8> = lc!("9c9d2f0f9ef476137eb392d5cb347035840786dde834188148daf42d1bbb98ca792fef3fa7a4b32b230737a55cdc8932889cc4741d77bc864c57ba973ca6950c40b1139a6fc8ac744e673307cd7f9a624be5f7bcc2d4f7904e24122a3c5788b94af78ec60c7a0c75981aab5a68b8273466436f177f2136131a625e57ab352ce63ee6d2ad02b1ef715e8f1cd2a6e1b36bf7eaa3492df9405f1ddff55eba673dffeee131fcec54bf79012f82e62ec9ef2e16bdc1985d9ab4a12fc7b4b801eec004dc3d2b89761e66bdcfbebfae37e6a1d6cf634ee4747c91152fbec9df65d7eb134791fb8aafd25cdcda56b2dd96586cb8e48b305e415b0d2475431904bb23649e").to_string().into_bytes();

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