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
    let secret: Vec<u8> = lc!("d6c5e704d7b87b94c4d7b17aee74045684595e5568b4dad6d8082dd263025547ddf86ea2540a62da71aa88c037813bd67719179e8f088b0edb0e87cfe4e97842fc511a39b0ac0da44a47d004724ce6148a135068cdfc323a220103652f510c9c1219614402b65b260789c9aa2151f8212200348045c2e978b335b8954fa78f95c3ae7878afad4f1a38efb2d80bd3b3dec5e5455e0015b63a6d080eceb9c34c7da5f9a047d9fb40199cff5f81fa536a44d2af13a9cefacf32a911379f98630a6dc954820144291d5403d38c84cbeec98790879cbe8bb5477ca011e68ff7e6eed0b9415dcff9ac91795a888add0a8dc33a1be10213bd09f655de4471cfd03a1646").to_string().into_bytes();
    
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