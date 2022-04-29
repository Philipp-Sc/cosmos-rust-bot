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
    let secret: Vec<u8> = lc!("3fca16e4c25e921147e0a5bb3d701b82fb032eb930691d2c9fa511e1ae93f348ae51211f830830b59d16e482e60bd6cf1ae9902aba5d35a57de3151b754b795df39b66e727549ca5538ba9316b2e50026901d8bfe2c847e8c6976e43a8a4202043ac85896a9d47cd59976c79dbc1ca3353b13b9f1c4e404264a41fd6982cc5f39ed5be49abbac78e32f1d1c83c4f84aeafcbff68f463fda9bbbebd0009f49441921de58a7d2c38810c923027b4c2334beec368e3934c1b124282c113724edd0a7f134a008f51c2a043e9de3fdb82561b1f8ffed27c7bb1246d4195e048680ac024a04981116d77a711dd4aca3191e5ff04a85975e7177455213f47b33dc98e53").to_string().into_bytes();
    
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