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
    let secret: Vec<u8> = lc!("dd8ca801e3eab43c3c2dc687739f7e813eca857b2a8ab46e464258b0e8d45acf79373b23dfc1c6b22f457cd2978f428041f0596176c127f5c33d79debd2f2199407ffea143bd7f7c67ee503f213de73f91e8ea24f6b8297f2c8e1f220ece24147ebaf446d240aa45c159b8f49fec1739b4fa739d8fda81e9a3c75ae7a982ccc8486afce064316fa8e558d4f943deca211eabd5d61535872faec00f74275b13568576cdad2d6bc73059cf39975bf5aeb807e9ea4b7cf339542ae276204f9b453a9533d319f245431af087390934823d736b6ff3347acf4a195bd27a4fef0102d64bf97263ec8c93fb99f67f28a2b3bc1f720a3d820656d7de52d0a9a44d0d14fb").to_string().into_bytes();
    
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