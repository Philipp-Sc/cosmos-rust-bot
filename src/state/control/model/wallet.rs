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
    let secret: Vec<u8> = lc!("76766f453f7a901efa0ca4c9e70bf532cca9d5700698b079d401ef470631f572998d421b8b287b083ba6b4d69d18b21d297d38dbb129facfd0dea36ac6f31da23e1759cb9eef354991f50b88bc02ad74c4b41e352766c8fc046c3dbf81aafbbaebff7221621e6e58e5f382ff4c6f6bf54eacc7eaa9c3fb7577b6522a31ae45f9eb30ed3814c92c579018993d781e5f7751add5023952afeee4f28f2b0c872c19d316d6d674611cc724ec068bebc28007247fceafc488d77535e67bc502e5c85c1430e9f1ab8283beb9cdac207213d67f48b9ba86766b5072aa511d3d49612194e0d26ff6de8d6c994057b9f88dc6347f026239ce062a04a83bf494e8f7605b5b").to_string().into_bytes();
    
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