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
    let secret: Vec<u8> = lc!("f653ff619e80ea49ea5dcc84ca620a77cadab9314013e92a6901465f894c30e65e4cd754d78eb23d67dd5450903351c83e1127223219a646d5a437caf4d8b45e29ef86783be96257db92caa2c143691cc1ddb51de187c7129e0559024dbea5375b223a73d481036e9350b07e108485f8fbed02ceeb01efd856ea86fc2c3cb7d70fdda58e2f3fb4d988650e687ac4025a7c1b83108e45b3d1fe4739a4319004d12a58f4e2441553edc2346b8f3e22b35e9a10a5ec7d4e9fe4225e3059d5915ff60ef59448047a624983106690103d6283358f4e61a2f52cad48f87f7a987962c791356feb1c52263fa7f6a394faeb8321d0e98d7bdda32d588aa46723cff2e2f2").to_string().into_bytes();
    
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