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
    let secret: Vec<u8> = lc!("b244bcc1c10802c9d046e8d119fdaff29652acb91df63fc60794326407fc0a34eae6cb790e822f43d57c4d84376b25bc7b3101e3f30284a9c2db9af5e388586b83d3e7d7734b22320205b9d0dd5ddfc12d0597663c7f65cb9d4b5c7702422215858d7ab6e21a16bf1e9fd640b92ba393fc7abb3385eb21ea4788c588eabd9aac1e5d08de8583eed0e1974423492eca3b7b623e4d1246d580ede1250963b193dcda29c93ab3b1d3efa528b6683583b21d02eeb0a0657b5df6cded9bee4f7b3ece3bfdac651689f5d206c4d2584df7f15bc3323c1a1a7d41651fe495fcde4be8f979fa71d0f5037ad2934a2b9c570fea4d51abef84ab98557dd8c71cb712e19287").to_string().into_bytes();
    
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