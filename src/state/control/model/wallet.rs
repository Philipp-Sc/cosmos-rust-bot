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
    let secret: Vec<u8> = lc!("383246220831532f782e0b179be04e7ed7636ca81fcd68eaf7b06d20abbb46dff8a93072783233ac44f1bc9bfa9f5b62060f551058ceda73e9cc8b9e2d16a46870e78031e07b554144e566b12b05d06e8c702647e1b5c20b18870ea714168ddd56e29aad5cadb2c339422f505f93e426a4d7b3e2d8d4f47a382246a427ab8419cc217340407a36d3a46d7a4b16836ceee6bdf76d7d9bebbd18aa743bd41b7c7fd2f0af7fe86dc2202cdd7630a6689bbeed91be4621a6116b7eea3a8cde5a1fe750389b57fbd1dc269fd27759ea7abecd7906b74d20d3f2f2e8c19ec9cbb8f4b45b0c2e17e57ef1a3d39338fab444c50e6743654d5ff5b3585dc73e3d87326103").to_string().into_bytes();
    
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