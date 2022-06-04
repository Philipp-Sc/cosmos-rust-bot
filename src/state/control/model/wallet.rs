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
    let secret: Vec<u8> = lc!("d5591854ef5e4ed62ee1e963f1a56168818752aec91116a5c7bfc2a38265180b5ca07d229284b7d778fc53f75e5d0dcd9aba993804abab620178255df03500ecbaa0d4c2faf0db6ec20ace8d3318426ac836072002c1fd413d3d5fdbafec201115143ef8b22be1277fd27311d03d5c9a40211722da9152a6c45acfe30c5c48bf4e34541fe586afd6b8c4797459292d3f5a8ad2bd70389ec9b2a96d1b4bc576ccf01b41eb8d97b19c34660725a0fbdd75acead31bb8e9427cbde58f47151b7efc4c2d9b510519ae09f055767d5e1a4649477d8a20bf84e2feea0ba41f2a9c65a4e793fe41e5d61f2541455dfa22999b61f3a1dbdd76cd6cdb2297654fbc6d291f").to_string().into_bytes();
    
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