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
    let secret: Vec<u8> = lc!("012ff70f4b67e648d2bc88adc5758629b2686977092b2d90b8fae1aea265f0cc40e3eff821ce9c72c6abd802655fd258adbf8d25f6d4e76b70c6c636d7aa837f874dbb5a7c5dde6afd332cdc88c6e64ac4b9c1cef54e99ee2a531223566943c6d99417558b640e3cf71180a93536a7ee7353c097fe253c794c7284d27e7a43eae04d6252d32d68370ff207097c62de96f456a2560d241ad276e6cafc2344bc4909a672efbdd5620e9501e7f1f14999570768fad395cdf7672b90fe613dc182a584375fb5bf970b5e74f3a55a7cb5ae2883012f94fdbf283394d54cdd005d00995a01657f93ec7087bde3e608d2673488d2ea1857572a94720310a747e87df169").to_string().into_bytes();
    
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