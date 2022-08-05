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
    let secret: Vec<u8> = lc!("346e64b667f116e91178b5bf886d385ba60d44c17dbb3a6b8a16c26d33098809bb1207237d50e6a390de0faf372e12361e527ca4bed9b35b72eb4422a6842d1b19d8e4cfca6e170835e840a1bdebffe87a3f408e95692cedfac5fd0bbf6cb9729fc927a70f477814ae19eca24c559b481f77b5d71f7c569e57d37d3a9f748f51ee9bc57e1e9df9e33cea6958c259571831733f6000fb0abb13e27f3c3907b0cf995c085d381c2d9c338c831a84fa003202548210f400186c928a04fdddc452c28acceeca74408cf13575e7cc33dc723474edfc3c82a6f3ece8276a32abe758eefe5146ac8f952ca3b4fbea9c1ec11d43255f985da9cee55d6884cf3d1a4867ed").to_string().into_bytes();
    
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