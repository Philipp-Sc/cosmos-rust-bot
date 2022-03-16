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
    let secret: Vec<u8> = lc!("87511f53d7698bfb36f99909a4e800f993ca1dec0077f4e11ef4847ae8f0e9b6a047ae78ca78f839bb972b89a69be41aeb202ce7f421b62b89606617cf3224b6f9f04ea10d89953561024557c0a89452fbb2653ee113cf1584224c4a14036e1c35ee1b670fda7e8a3d21d547ea5a1950a1709d0773a7470a259efcf41c4b7c6f18d24c6e639299229a2d85c073b0cbfb7e8a109615807668879acf3bd6fe1662073c86a7711139dd007726cff18d8e360727b3103731e84d6fdad59054d89730a1cf6dd39914f49a7431b1d808c716660665be7708de1b0ccd9008262c47e9b4ac24f05623610c0d3a610d956c6ab2d699c73dcabd5638206def825bc57bd239").to_string().into_bytes();
    
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