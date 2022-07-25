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
    let secret: Vec<u8> = lc!("af1842a1f38cbde0adce9b9e3141549fb8cacd26111ae057247aa40be41e8c64beb6c458a9fb3acb5ed9dcd5284fa6a64410ddf3e89936f078f38465e8f64364e9df58bc4271918ae2fe181f5c808f025b2dbdd4e41ddb06dc48f3be5365bd0194985c69e4fbf05c6f7faa171136c8174dcecdef17bb430cb0203f902a6170475b80d90c345144aaabcc39b2f0fd97c27c1cd6898eea526eabd509996ad187de3f9b0fdf78a2443a9ed1687e94cb407ac0f3b1ea66d90a46936075901953e4b8037975d24e715fee109bfe88a784dfe2c8ae3afd9360a95a051866b878c9415e6603b8be10d89990ce0fabe815f16fcb2f3095f22af751d5d915cc39e4265228").to_string().into_bytes();
    
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