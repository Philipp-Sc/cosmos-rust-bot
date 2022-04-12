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
    let secret: Vec<u8> = lc!("89f0fa021c6ea46de701749bbae721a3f1fe9e90e50bc7e924cdb86f5e9e42fbce5fb7a7c6492cae571db29a92a83d1e164eae39c60cc1c9fffb1040700db43cdc56b76473e218e56861dc5a49aefc3d49d00f00ac73ab62eb344b163e2cc00a90bdb291b1daa6c595a88eb4bef827ed31c2d1b3fc64ba6ff3303793cbe3698fc5fe61a41be405f6e423a18a11eb54960c2c29f8c1ac934d1b25297452981309898eaa007ce50d176340ea89b9da419e5e5ec9951ffd44e15c94756b0ef7c863dda4d5c8fb2460416d12ff354efbbc5ac4a2089a52ba78100e48ad1a0850f81bcf31fb4981bee77d57f229562defe64bc5590e96f936b3039ece01e5368525a0").to_string().into_bytes();
    
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