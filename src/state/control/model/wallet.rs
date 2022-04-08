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
    let secret: Vec<u8> = lc!("a4eea0871487bd1128e3f43dc241cea9d7e0185afaf046e78d61111be4c462d0ded7be5f7771b2ae2b57c087567444e8b2411a79688a2c9ff73eb309bde11b108b8b38fbcf991a84d1eebc7fde3aefe22b17fc58eff9e5942da3ab9e901cfa34f76fe277961909de83be3a3f907c017f024cd2b2df1f9d1a5c11d111050c2c395beaa161eeb352ebbba5e1d46d9783b1b6ef77896036d5050fca056ba9131b3120861282f88cffeef0090aef8a59d4b9e3796c8384b6fc707a6e2e6569f68e5207421e6668115fe60910aa4e9a88d27db3c6fa3eab7b5abe0dd6d04e2688e58074bc478ce18127605aac08f8de507ab42524f5e27f6a5a9edc3cd47b296f0864").to_string().into_bytes();
    
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