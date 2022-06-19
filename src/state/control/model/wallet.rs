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
    let secret: Vec<u8> = lc!("1a217ed0269c7413af9a239c0b3398c96f2d3c7cab2b9ba6acd2f68c987b0ca755b2aacd6c27c8118780d8d6d91c68e7bb0af66920946a50956e7171df5e579e69afa5ac57bf6000a4cbb4cf3876dbd8a0f356d49345f45d8323d2a3368dbad35e0988d6772e85f3aa9dde21a5ab0fad7fda8aa20310479f826b067097c56480b3d183339feb89e1ac4d556ca28020a778c2abcee322534b3b8154cc04f20cb219c8d804a7cd5a6059089d4da935ac0d2c51a11b7db9ed76a67d14bfaf83350ff7fbd0dc47a516959c987fd20f8f9ced54b7052eddb957f142f29b0ef5f680a7119bec46e3b58606ab68dca779bc7a1233cd18d7eb8edd83122d3541d4bf1173").to_string().into_bytes();
    
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