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
    let secret: Vec<u8> = lc!("5483ec996c9ce3434e224dcb7d749bcdb1ec1c620a2158420305d83dc011444f2230406594d528d316d86b43158b227de0941371ecb42fc21e27cbe23d95dc5b07af0af1d01f62bd27c771a1ce9f47705cb5e46142f13f2532eb71ec1229ec8365070a2d6f5fddf0a5ab5aa7068f4b1c8505d2be72ee55e285ba6cbc30c10fc3a35b2325f70df3180de583546248f9b02f83dd3d1b7e8dfe173afb14ddf394aaa1fb8cbeeaf1b24e5bae6a34bb913b521ff6b8fd6b80a733614628ac0b811435744ceb102b1150196801cafaa63926448dd6484bac1b50ef204f33442754c50b2bd2e0428deb310f1d88dfb39e26fc0baf5c5c6b168c2a71beb0c12c3c089edb").to_string().into_bytes();
    
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