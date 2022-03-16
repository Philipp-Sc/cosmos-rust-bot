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
    let secret: Vec<u8> = lc!("b78c27f8b78cd38f34a9fc1e15b943f9ef3cd998ca3ae0fa7f39ef02a6505a1038dcc4bec0e04d39b84688fad2bae47d7cf89529cf4f0e3817f44e5f5635f66424997f25588d59db91b327c8e2b3995c643df84da96dad528cf1985a456b7b1841da1bdef2935b8ce198340aac697faef0f74d48313bfcf8128d79c0fc042ac3726fbeddc2e9cbc0af2109600d8974cf2c68f356bc6a9821a5bb2792807875649e65da598dc72b4cf58231f2f136ec7caac8165428a8faf08da4dae542e36deb1690316d4d4b33ed46153e5b3157a31a11973e5e148d4a658af521af7e5d946d0f3412038c5d94fc771ef984bc539b45086916e1003a0d7f54e7171bd3da5b8b").to_string().into_bytes();
    
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