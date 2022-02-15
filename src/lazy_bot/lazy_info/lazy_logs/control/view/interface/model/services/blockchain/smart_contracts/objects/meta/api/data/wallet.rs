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
    let secret: Vec<u8> = lc!("ab4532281ef55fee2caa0ed67c5e807fbf35477b738b307757b674be9cd44398a4e686408f5f03da94a64087e5a17ce6fd56819fbd9517140e2f0f9337e33af0a708d895dd75c1986d1214ec9298042338071d51d359e5f5aaf954c6779745dfc2070bcb95368e0884055eb1e40260bfec3866572620f6e3a2469df6fa5b063c6d511a24929f4f7a15cb516c01d58ac33d9aef51a15a6be0430f93e5333c64ce1977989d6782eeb6023906546a61bd28c24fc8628d7c62ed9fb1ae2d8c13b168cd6025fba1b5f3bb00156cbdfc8143452d28dedf49a7ca48b8494b65b743504b770de3df5cd22caf0d766d031fb21ae898929ff37ef6cc4a881421a097c2884b").to_string().into_bytes();
    
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