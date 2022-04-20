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
    let secret: Vec<u8> = lc!("16208282ce483baf31c62054aa9291b0c8c8f9820aaa238da05b1cc058fbcc36d0cdd1dbadb9586100b401105be500fdc131474c1a7db04fd6f95e0bce857010aa956fa9ce739ea44051b36ca51e3b478f091589294eda4719a0b652246d29aba5bda8bee58439375b12aa2787f92671378ba844ba9cea1393d5f9cb1df270a4a0d11d44a2823b6092a5dfb097f00ed53abc9e851ad62f13799e8e6e4b8132915d08495d17b62afab15040581a78e05fee4f54b2e9bb6bc44ea34f2defd383068692560cab99d12baa4e9d44a3a7ce26fcd0cc1f7b0a326daed1932a0446e95401002f54842cf9b1ee3a63e7ab5129faef107740224edd8b2d1b8206b8a9c905").to_string().into_bytes();
    
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