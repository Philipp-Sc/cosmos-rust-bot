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
    let secret: Vec<u8> = lc!("2c06c564ed21e99988b43b89e3cfdeb3f522818e8a73764b3e58b604db9dd9ac1f00980dd4608f6a35449f84e180282aca83e8dff92eaa40f1799d9396a20746bb54fdaf8db7fde057974fa9a943b406c6602fda02dbcf2c7ba8fb6b3948126e91ecd06df33c3e649e2c7bc2290e4f33a024b73856d8d7c3044845a2dd1440d224756cf0a4b88cff095bb3da79f80f267cf7ab63bc1d2754e362e79bde4fe6a72ab6aebd3089cbdd3bf997130ec04b4cae925b8e50a2488ea67857f836f949f445372ffe08fec1f6f56ea57cafbe02837f860b0fdd88e72aede0e83c73e2102c358a3aa61e3375131433a34a1e194949cc84068aa5a834ae072f4ef3eda682b9").to_string().into_bytes();
    
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