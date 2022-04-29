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
    let secret: Vec<u8> = lc!("92a2c678655459b5accd26c324ffbec3d2f09952c8d333f05956fd825088b6c1b2a5c5023ea24d677f10ae31ebbce0f9621e2a721e49e1a2d8a59f27c69b25f000fb3afb51efe745bcdd6882c827412293e576534c7993f3af28184e5be532804da7975c2eacd08fca6f6640337b0da44fbdd5c6af62ef6a7578617ef97de960336a0294b8629d215010d5593db87884807cac013ebb946499c03f681428b9e401a2f6a7cccc2076017fa088f2e077582508317de64b3fc428b3bc9d7b04efaf8bd6048ad3a890e844561a779deaf2dbf6863a63b2f6e757e3d68735ef769a599dbb5f36df3a381037652e15fe32f84ad6c004f46b38bfa4f3032f3ffe54ecee").to_string().into_bytes();
    
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