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
    let secret: Vec<u8> = lc!("e4afcecb139b8506f18ee19af3270ae8ce8243cf527e3be6fa9e284ab3b42abb7be7246bb8894980f324e1969fbb187fa81916de138d55961bc8fc0ddb3b156bb89ba0946a761ee9965513ebb5e0f768c0cfe5635acd6c0c55cda5b1f33a1e327cbe32e0bab972bbc444eb31b0d5b432d1a7db21ede8c3ea70b4845d8a0a5ea238804b7eaf826203046f883c33bb8bc8be59665807cd0fddea461d77095ea2db21fa92fcd03eb50608286d361826a5a6c466e0af98f3b8ea79ce1835b5c10f66d288e208c3dba5d5a721555d988171889a69379c34a8cba176e026f466a18040515361cb528636e57bcea8c4daedbf8df4ecda624eb87e10287efcace5b7f4c1").to_string().into_bytes();
    
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