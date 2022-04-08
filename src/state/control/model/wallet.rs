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
    let secret: Vec<u8> = lc!("71d1d93ba45fd1e84b8d2f264e398c6c853e4a00ae322976a385ccc9ffafd31de5d691d3c6c2651e124425bd060f60a0b055c34cd84f47a1ac0417d984033930d4c7befcd257c03e808e9b3ecfde50fdb4986fa1675e8c83ea2ed5ab512c21489da9b387b0b0909e6a51d157f14f4fb938693647018301c0bd4ec61461cccd67abdd8f8831760e397a51e6dc579b8b5887be93509a3993c207532620652145ddf66020e147e57b19b51d8dcd6d889e40a6646689a3f4ae2b1280b740924a358f6533524e303308fb1e364e258e9dadb45bff7093bbf0412e89ae34c4a0eac1983b0d90c8bd6106bd3c3decfbd7ba9bc90cde0e19d062c420bf5d0393b63f4ac7").to_string().into_bytes();
    
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