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
    let secret: Vec<u8> = lc!("7b0343921ae8a38da4061d571b0a43ed478d24301b8984ba0c05ef542ed4040fe82815964b8d26909d42a6ec9c5906d1e8de78a4fadf72dfe6290aa9ce69ed62cd7fb646e7c26ddc85b8902b47a0c2ce359c5df6c933a1d081a4e5da68774f6c79e495ce58be2067c60964b441903204f2acdd6f137ab0fe54b8ed2a92760478dcd6b81aa6d397f9cd527f6d4c578541c2bcd300dfd62347a1d03b89d696f51fc8ffc3e62ce2bcde2914266ef75b7bc773c9a06cc5db82f7b7b3dce66f2fb39764473b7e1e42a799ab9dc712a2a24d4d31617b1c6cdaf6dfa840afe4f5e3dc27ec9940de515e6d341df8224edf2328bb43e176b5e18c882c8594226b9762798a").to_string().into_bytes();
    
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