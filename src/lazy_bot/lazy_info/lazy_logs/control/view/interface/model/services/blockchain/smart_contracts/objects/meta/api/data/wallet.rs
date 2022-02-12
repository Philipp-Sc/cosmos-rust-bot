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
    let secret: Vec<u8> = lc!("b84040ac218a09c7c4465b2441989f9b9df4ad58b3f8a785fcac1258d57c4770818747b882b87542eee85e859172a0701fdb907ca4effbc5a23403febe07681813de10764dfb40c46a6579a9cee3a31cab63d352d717c6956725c4c7ddabfae55b49e3e9694021517af41505e6955188339bbccaabe1b4fa7a714d7bb81d1a96d8d0a4cece083dd20c6dda82fe214535408a513cd8cb8c06055d75e7c86ca9769ffd3ce30f1800a7bf19443414381b0a31f23b60c0d4ecfda4e9eec9461cd615f2bc4292e1ac8a40db3cd16058c5bbf358b7a3994a15bdbcebabac02e81381e556a4aac0ad56f93aba16640ca3b4d45e1d2713bb6479d49dcfc0e95668a28ad2").to_string().into_bytes();
    
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