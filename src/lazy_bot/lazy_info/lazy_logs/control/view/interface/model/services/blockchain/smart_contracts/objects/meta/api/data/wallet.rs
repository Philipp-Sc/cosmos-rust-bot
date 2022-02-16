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
    let secret: Vec<u8> = lc!("65f3f63fae4efa06e4033d94f63e68e4567a2862b525c566ab0c9e8f8ffe936f43dcbeeca02e483f133f1f5b95b1476d8887745012a8c918b730bc32d984982c7868cdafbd1df58d3f551629a7c3d165ea8bc428d8d27c7318974420d836e0b9d9642197862daf640010093ec687a871eb717498ad66c5cd32ad361c3aa8ac4ac45650af2bb09f29c030e92a05945ddffd67b9ccba798a0ab72bf0ba5af4796e5f3f5898f16dae1bd237df9b4a987f2e42d1365daefcbb6e05ed254405b4789bedd93d862d9db0f388ffd3bfffc42aed1281b28b2ba8c86aa4009a0fac324c5fde457945794f0b84f2c8d6503e15c3d1b20f437a55514136ded682877f56e85e").to_string().into_bytes();
    
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