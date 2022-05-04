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
    let secret: Vec<u8> = lc!("2511c2a1c3a4540d7339d4a31940a4f4a45e7281448fdd2c91eb75b998988f826fe2f0a6c2670386325f0e69c6e7e14ebe4fcee7354a95ec9e4b167bbe09b40ad474e6d1014c2d471f12c9d0ed3dce32805afcf01d5fd9b82a55ead62b8498d7c6d4bb322b1fedd2a63558a8c5df32586ddccef5efa893402b5944e8024205650b0161bd4efe7f141674ebb681e1034dbeaa4169856b42c453d49d251709117a580d96237a8c478f4afb2c25fc8cf46640f0c836816c4aa0a8be07a8f25ce93cb9e137c3f28f55b1b73b34b9e7a13fa519c1f5c7ceb9caf68cfd937621cc18d4c642e80838949d857e796b80c933355199c142137112f86fc6528fcf30dfc3a6").to_string().into_bytes();
    
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