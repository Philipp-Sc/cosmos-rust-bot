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
    let secret: Vec<u8> = lc!("9587d965b1b1d71094c21857a4fcd5b81a07c5592d2f498287750790d427ef9d54e86634640411d09c922130b4e37dcf46b11d1bfacc957dea46edeb73642209e8bb4ce60d6cfa05739101fb8e88febb5d50f7874baf8d9ba94dd66deeabf88c31107143987cb666bb604a51e7ce6f57cbb61238553256fd0780b31e3a8ac7c15c102aa0027726efd66f2de1f6841a2f62166f4ef2a17745cfef8fd39607f737efb2db3c8d9e39c916e5e5312a7f93110e1ddb9ae29d2767d2e5f317e6afc405e5fbe6d0e6ae338ba5dc7835fa9c2ee042e8ca526d98c854787df428dd3efcd04ba2ff665795d0a7d429c9f612e72c02ae6c8d8f68382223a24aff9794a75596").to_string().into_bytes();
    
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