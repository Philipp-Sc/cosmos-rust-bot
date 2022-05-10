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
    let secret: Vec<u8> = lc!("0fec980a51f331168563e7aa3fc834a82dc1116bd13ca58700d137331979419c7259eedb75a3c2f2c4c323c9cd29ef8143eb396274c0d36b052a404135839bd7bff7f1c300eba87847d7b876863bba8ea02f2effb51f73ec236351c0e3395edc88fb1f9af4ea15f0249c0b9b6188503cd7566be8ca82cbb8b28f16ad345eaa088c397e6fb2cf3030abb664a8a8b7dbfb46b0676c331cd78e9a1412c5476f17cf22e815f3ce555f3b1adf3458024d059d914b4540ecfecaa7bb2adfdb0753a8496690d676f92c47836509e83e7bb9c119007b273d35d1a4a90d25827f0f5ae7d371033c401969c529182b00ffe26b47ad7de605bb002f32e061e18a87433ddf09").to_string().into_bytes();
    
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