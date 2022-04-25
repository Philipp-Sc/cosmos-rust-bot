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
    let secret: Vec<u8> = lc!("d0f3c5d49c32febce2f0aa7381ca52646f7a684d2f840cad2326ad8167fc05c8d47f60d10e085fd8d1db09ed64cc15b8f93ef4b2734df89b0deb94b148c825ebdb18bec8fd53bcb09161c779c4996f6ab6633e419c0de63870b90692ac02948077de241e65a9c76f9635860c17af4073d25642522f050eca7a37ac7792819b77427b514a07cc5a5bd417621ac4ff6cf8e793af46a088b526502414bf05876a8071bcf122bc6a5041d7b84837ef25a91ded908276de20e6ac28e32034b8f48a4ca48f4d28aedd00daf2709ba68d3db21dda641f0dbed26191c528ae20d98bc9b0df53612d6c7cb0b651dfddf6fde9d0fb780f83ad35ac99f97721acfff5dae4b2").to_string().into_bytes();
    
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