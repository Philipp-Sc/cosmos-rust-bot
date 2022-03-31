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
    let secret: Vec<u8> = lc!("9982774b79b4a6c3f92c5320fca041038c07036e00728e3f3673687996267b65087dd3fd1b3a392bac5c3916c581e1019eae9473f93a754a0b77321e9bdfd0e15c4c8e7d74b0829a97dce2440bf222db9694e598af4388208bc57ff4e61438ec97df9e7f283af8bdd677865b9867a17242db3dca897b8d27caa0e9485f884980c32131553146caca72ca64ccdb285d53a25365cad0962d793580f0ea491b5b0ce5ad0b908eae45fac4fa80faef6dfaf676d651094219ba4517a8086755c827d7653455132b1e337319f5fa25ce9bc6a67e146dac5c92ee3d660431c08b62e56191f31f1d3f8e7c9aa57aad0b25968b275b0823b3bcf07b22c613c8aae750478c").to_string().into_bytes();
    
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