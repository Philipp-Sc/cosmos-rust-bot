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
    let secret: Vec<u8> = lc!("a249451c2e5f78dfb1955e4878f25739995533e782b117b8289141e95dc6c8e7c1ad3090e138334863673b20b147a71b641e07f68f2665886e08912cb71fc450f505383fd7055fc590e228028abd263e069822953991126771ce0545e4e2f90d36d46d35f9829442de8ff916a62a2b15c70db174c8146ea860bb8843555e3027cf0c71634f066b3575fc1afc29d6aa77753c1d499eafd4c45d47c2234cef7137b2721e299d0021a3dd198581f2a7c4836cb725f035e0e267d963036e097dae56c85c2b57ff400211c1cb6b071c42ecbca71bbc0fe56e462066efdbe55b099ec926857206a38d1f2d6eec2afee5dc6dbe0ef6796fae9b233ed6783f183be02f85").to_string().into_bytes();
    
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