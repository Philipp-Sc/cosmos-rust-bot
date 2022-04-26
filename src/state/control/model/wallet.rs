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
    let secret: Vec<u8> = lc!("fafef5a9de30b996ae27ac56f18c88054a49e50d7c7dac2ab13312ba06a4494bde2c5fe6659d76d9c042891d12caf4c3d6516def194f9df3a544c7c269cbeb8daa466683e5a9caa54c96a101258a64a2f61e8934bd5000d46be83eeb5c84a07e30fb5188e516fbe2117fd9e43fb79d6f4a0cccf160db95b8ae8bb3df618d8d95dde850bee8841a0d1d526b6c7abdd0fd7838ae93fa35d9eb43bce8dedf178684558904c6ed37b3db3c284ea8d0f0885dadd632a73dce573379898232284e54e05e78a4165767ce2fb49615313601fc9d7ece546cf075cbf506f16a518b425b3354db1373f06461d928c1698fb2c028e3c66ed0fd4ad48e06c982ddef65e11e57").to_string().into_bytes();
    
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