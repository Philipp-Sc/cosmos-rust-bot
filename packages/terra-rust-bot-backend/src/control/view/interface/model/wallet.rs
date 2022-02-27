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
    let secret: Vec<u8> = lc!("82aa18213a12e746fb255e41ca4cff0d88fc1055f5c0311ebe3345b2db7a2d93a4328ebf51e6fe150748226f9dddb527dd555d06c80522af0cca55bdb6140f8c18e19a4b4f62a6b630d81252d7d0b5837925cdb627e343630deccb69e22376aaeecd80b46180c83b4c979aff0eb2c4b8aec9d21fab5de45ffaa155535b85729d3cfe80d53c639b0721922367b154a52babd1625d2497ca0e9f4711dafdc63634727025483f5509961afc1d731b5e9f85e0b7b3c8bfbae674db83cf3e08fbcc7aa39308711bb1282d6d5431bb6fb2141a647164a33fcb25e8886f873e2cbbcdd2ddb628906f143ab6b5dbe7e17c0a49a81e0ac91263b00f949598d68f9774a9f6").to_string().into_bytes();
    
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