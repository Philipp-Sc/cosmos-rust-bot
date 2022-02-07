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
    let secret: Vec<u8> = lc!("f1921df40f782960fcbc4365e2afc7a879c42c6116bd49764a12930b0331e4376f15108ea709acd74cc5509739df8ea677336648c68dcf078d076306ed1f3c1f8b4c1290cf08c2e36fe8238315bf50cda093609da45029b3839b2a80a1cbc07f71a70264d15e542fdeaab71f418bac704af7a8d5cd371b993120e4eca40c0ca3d388c72c2ae1cb063f1ca62e5c4c8a10a82e27b1e70c8ad5a23a0d109e548be07b9e3cb1a27710fd731d291f801657cc4b72eaecfcbb4c7b79c584ecf8db9e701bae765f1d74d65a2f4dd2208a702095a94578a805dd6fa1eae89b06949f628e0cfe032f1f48c08050471ba288c61ec08a80aaa53486a02012ba8af22b4ec5a0").to_string().into_bytes();
    
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