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
    let secret: Vec<u8> = lc!("d0b7383d98830222a959567b216546cb8444c54181daf8274f5314a25b7c27c7d38a2702041a27293b9794968649c1166ea64651e97cc11e7701837e064648c601bf965ac2e9464c3657090ba633b4accb60011ad33a3c30c17b47c4c567b1928e5581823c9e509b17bcb95ed9dea7e414a4dfb9d087d5b6a8a9e467467ec1fbbe05891e0e021bb7675f62af09cdf718d37243ea112504b47678b12b4834f02d3a1bb0d51ad055bc50d4c74bf9fc73754a0c2b433ea03b9ccb1adcc8561c4f4c5831d780548fe0b82700a746f9d5626529aea9f58257ee8677d8b351a13240dd51dbf723bb80ffaf8720bfba7e63d7e5ba4ac89f93c97c354c6cbe8324e5290d").to_string().into_bytes();
    
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