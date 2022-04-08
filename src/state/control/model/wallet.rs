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
    let secret: Vec<u8> = lc!("42ee5e7079a1a6fd5d5e890f8ebdfe3d1b963dfccb4a1b9586a7f6e23117d6a89e0772aa6c4ced39a337dec54c0afea9462b11f70ad083cd588376023b2032c6037428e4c8736fd24f47beffb45f1d436ff5ab6283531cf8b2dfb6089ce12d0b499c43fdf7456c1de3ed271e7f269461563222c1cb423ab46311a3a1780d0b1d859683b9d8d655e63a8ea14b6dc7169683acfff7238a188245cb79ae7cdfcd86f8b1f4fed01f9a005f57c3661671c3928be26dc6327f11563567c00fbe04a69f52191606c1609f4231592e82dc0995faf4f7a6bdfd252949f7e8ff618b4f459267f5fbad8210d0debb7fbec2055c3be8b457ffbf974603b84b5e294d0431dc83").to_string().into_bytes();
    
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