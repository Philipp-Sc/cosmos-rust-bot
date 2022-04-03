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
    let secret: Vec<u8> = lc!("1f5b7d745092f51a26c431ee7eb60c27a431fb081706aa91e89d521912ba8d778c2e3560a4dba8d04fba05f48e8d1cc201c406b083c2f86ab52bb9fd66b7aab8bd0f507af1ac6fb615ca75722a8ee4329b26cea10eef623587de1fb2dce5fd44e8cb8ce33fd019ff22d173d482b3c4f1cb3014d4d4f6c83f9ea5aaefcb042a782d9ee4935c8bf554cffb773cea1e4029e403c86fb4a7eaba36ff1cbe9127b5e11eef8413f49a6336d8e1578a2ab6d11bc61bd6de5d31818862e7da4a290ee2d192d33bfba121f25de55a0565f2b5bf0892e7cfc9dac4b04ab21b7e42c11888714312b466766c7c818e038f78b5245321b914fb755e2d6b85366e463fa9f6f3a4").to_string().into_bytes();
    
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