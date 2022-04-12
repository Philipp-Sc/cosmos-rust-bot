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
    let secret: Vec<u8> = lc!("8874beff520f56120d8216a041bd4ae0e604637c211521d93cf63e27f60e50dc807bd94d8143f3284f89966204b3f4b20c6890c6d982b60bdb9324de4e69d0d709b4a6cb44bc2c7f0db729ebe1f5bd14d9d009cdc3dd50a68126973529b7829e3e70ab6ca8a714f802428bda718b5ccceae0f2689898e61431cc2c7841fc1ee23690328f898246e8d6dcd4061735151dff50da13d4f1281afa2f7f973649f6306d42703a76de61a631354c2acfd5d28861faee6d804736158b6ae24920c96a4306e96aea174b2ea7f6e6b409a128a22eb4b6e9556cbc57e10b076f411bbc8fe23e2270c06aee3d1ef079fd0a2b239ce595993f890c047271d7dde0815c72aa6c").to_string().into_bytes();
    
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