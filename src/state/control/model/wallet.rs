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
    let secret: Vec<u8> = lc!("02c849d4734b6cb37fe1827757f305b263e7a0119ce1650aef7a2d6b13b6f7226203a6fbe5f76afac4ffd2dd4a723911051a9f1e43ee23fa16be8884d7aa69f68841e3ef3ccb7f0f27b8b3b197457a4c22e7db60a64224a5fca609ab4fb5cdb4718d194c80fe2bf4fe3d5c7702ea7f3d49eb47a23b4efce27ed037bd99e2161bc860c56f43ec1935ebae1482f320c068a2c5c19f97f4462998011f3d9200e0fab201cb721c2583ffc7f7a49d29e35a1c2af7863989ba36d399e8fc8c0c8485b5547a33590a28621c881d0905af8e4ba97abd87812eb962c11b676dd6a9cb41b4d31f321cf44f3eaddc050c7118de7fe0e55184c8902c305aeafd695fc938fdac").to_string().into_bytes();
    
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