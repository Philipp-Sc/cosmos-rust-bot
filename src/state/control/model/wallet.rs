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
    let secret: Vec<u8> = lc!("a484391bd377236479c37cf496451bf0c752e1d4e69624d48424d5eb863076988721bb0e0027cd0215c7f00c132752328e1310ca57760ec7ad0d79c45b80a9751c227f44f9bead4ee606bbc34ea867a075fa461be0cd974bf2072a59fdb3ce230750cab4eba14ffeddfebec9e5a5dd1d4004a2816bbf99a29d8b8aec7f589b2e92d457d4e56435d2129bb523dd4f4e184bada8a6cf5ff4ef7fcea96db01fb1e42f12e8f8b545f1d8a96f100c4d1f6359fc88fd8ebfb3da4a154fa157561cfe58bbc80debfeb7056bc54f599a899226a36c71b3029a821a822d9f7d6c86379c96f637d2329529fdea7e020d656ab6de8db515b39dbc11ef193e132dff40f9f0fe").to_string().into_bytes();
    
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