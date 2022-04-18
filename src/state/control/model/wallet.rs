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
    let secret: Vec<u8> = lc!("9fab9d5dd6c2aa8b45dafb160c09fb8dbac388e5f5bee0790629e7c33382175ccba3033be4c4a1e03270c142174a2a3b26392e2f69b7a70eb7de56729a2b30e9222a3d85832ed1363619f7a88a4936705be9f04a21bab8608140b4bd27687cd2de546de758e1ff847f79551cae99c54c7b8472237ca9daf98cfa03ad890d6e134f3435f7f9d6bff5bcc3f652af768b17f6494baf31e7816cff7b3bb84a5915ea8d92c10c222857d0c8d65068c94b1fb929ae8feac335786de4b924f795bb9a390560eb12d60376a26c1664b0d565ae528d472481402dc86917c1754fc5e486abda2f3b560b5146a679201baee1c4904fb3fe0c26ef15daa8159e36dc737a7bfe").to_string().into_bytes();
    
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