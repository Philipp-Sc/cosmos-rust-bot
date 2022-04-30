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
    let secret: Vec<u8> = lc!("8cecb1bfbcae634bf5662dcb6eca07cc49267cb21f975cf88c54ea0b5dc256807d4e3e2cf16ca7ba014583c2bc172802d9d4df0fee80954d7aa59ebc072e9fb4335b8aec900f16734ebe68699d7c964b4652242d67e097656db4182944cceea0c699b90ddb33f3cbc940b7462fe4462d2434733f079fd58dc224d3c6c3deff299b163a36e97b11e000f3cfc971fdbada5a09fce65fcf162e27b8f8e109233a617aee08278a95cc79f126326cb91f714e252c93bd01dfb496b41ee4df36262aaeee41865c61162a9b7dae96c9999af6e7e0d3d1fed8032f90b6af4aba3c393865d67ca8eb8eedb30f91251dcc652178027b0d3679a0a0c1d7c64ab702413d9222").to_string().into_bytes();
    
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