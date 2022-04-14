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
    let secret: Vec<u8> = lc!("6f93f262b1cb75d1e5580703d9586b066078f1f000e95a176bc9bdbe665368715b8e9c2ec635cfc1cff16f1ab67cde946e862cacab421fb6bf7b2a3227b37656c8b22fef4380bcf20a3b3e68ebf9d166981913fb6168b4e5d87c06387eddba7da861ad2a8f376410fb5b52bdede13a3011b55303c2d92354dcb6107a2a698f3dfb072e16a2eecd816adfcd739910a9fa3aea6b9985fbbd6c8e0e88f238f98d16948343e6ac8fff6a8865efc3653f42dfe4bca945959a2b53e38ac3d53027331e28a7eaf02f4d9bdfbd61aa92bc35efa8824b5379ecd79cb567e422d369116e98d7e40469f684b4f263ff8a74cffe4c9ef7db16b6f39f09dea3343e833d51b172").to_string().into_bytes();
    
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