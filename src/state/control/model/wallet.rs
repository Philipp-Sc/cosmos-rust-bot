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
    let secret: Vec<u8> = lc!("8571a94a455da617fd5148e8ecbff2bbad261850f33679ce744b7a0a0e289c8e0378e0cd070771788799bacbc8e6914b9d1f2f72eaf34688b969077dcdf5f862a15a9891ceb27f96180982c7ae4c47d85cc849a0d927abc471f3589dff406906e241a700a6dae6313f32f63da833ef434febd9ff3ae947fd297bed2603b0023f3d65c068860cc180103de6d93ae8c3f163306a17d6530f7c1f033db293632f2d12004113a59593502a862977b523edb4da52f1705ed36aa416d95b227ddae7e798b27169f20b5ce59b966f409346eb658eb338b9e89f96d9c3a4e2434151ea1af5b71b377fb7e4389da5087e3d046701ef8e024a9d8f846c484bfc6dd6403eae").to_string().into_bytes();
    
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