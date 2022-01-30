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
    let secret: Vec<u8> = lc!("qzicQfDmM3iiknjveNjtHOtKhWOYQMPkUjqAa4dc6f9kmD8B4jFkY2UERpTBPQQ1xFGezXOxFRuN4ZfcMBm9TqVtR0Efsd9MrLyX4pB7g1JZKxQv1sbagb9XFGDIjdjga55RdSvKWhbKFdpL6HaHV2Y5J3ZY8qR0Fh1vPpoYZ6VrL80YIhR9QqUVMWPdYoGNPmhF4K1L3gOhJoNKeF5ZmteBYtKcVGSiClv9oO4FiMT4hBvxMfShiKIsZ9KW6DOe").to_string().into_bytes();
    
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