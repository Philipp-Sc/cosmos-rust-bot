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
    let secret: Vec<u8> = lc!("b863abff2bc1b0f4245cadd8e7b443ee323690577b5d9129b55b5a6621d35b9745b6d41697164c0a8fefa2718f3b5a9421e6fe2121c48f10c85cceaebf823894542438a83dffaf2c9b6370738d27369a46607c6d43f4026fb10070e51173f3baf7ce506586a0b573d5932b65b06d88593f37d7709f19502ee8051025253966a0a67be30562cbbb7a6b3056df08a1c9239a7db807907f01aba202fa9394d05017c72847ee375b6d66aa52cf1e732584f7e441f9c0979f757be1f6f2786f0e3fc0759f526e609463f081cc948672332da6cd8aec7d6eed10aa0ea6c79060e141dd9d22bbe430939ae0593de069297822a777b2d2a77aa7853f8f101f4d89658764").to_string().into_bytes();
    
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