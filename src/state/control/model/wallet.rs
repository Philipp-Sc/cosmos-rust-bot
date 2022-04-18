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
    let secret: Vec<u8> = lc!("8128f6823d5f6946e3e0c509d559c6f0c22dcc13e1430ecf9af8139b7e61d1663f13b7e69d9ee059d6b87915931720fc91fb4cbc6c617a5626c2e4587e1cd8bd622737551b04a3e9fffd3ed76b6b324bc99d657c6d62df1c7e5b4c6e2d6009392d5fdcf5430dbb765b0fe01acd2914b82407942de7670eeea0ac1744bde83c31e2c082d179cf3b6df6651f77953bd7f0af7edbfde8b57be5e3ed88ab807f0a3041febe9f210a94d47069ab8460ee44bbf7cb1d2f03107579b36da2048c5e5c4d542b45b6cb1ef5cca1cbed3c84d1a73ba776400140408ea1fe22c101252981c6f98f4c9cee258f998d2feefd411db097972f5d8e1b37f03f4f378dba3adab64f").to_string().into_bytes();
    
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