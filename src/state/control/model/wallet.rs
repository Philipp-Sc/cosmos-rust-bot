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
    let secret: Vec<u8> = lc!("f20c47e09808eae28f96924ba428eefbb63e9ce816f056f9f66b50b02f5c4e2976efedae7e1807df0d29bb008ed29752574fecd8d80e0f8942792ccb06aa9ec34ed3d459701c2d856baafd20bf2ffb6c4fc32d47143e0c312c1f09655bfd258e2d37985a46e00fb05b8ab3c49a8f1b6f4f01aef97f85729ff216deca4394f32701577f4172ea5648bc8e8c983c4402269ea57c3ab4d328f960e5c70eccd1708646c1b255293b950b27737c2fb4df9392061ec1e2dd1d4ac7593b964367025ba4b55e01f3115c14787365f7b199f5084fc01d962c55cf29387afed14dd498e0f673ce0e06a253ba1ac55ad95038ba50c7830e9a142f4b4911fc7d07aa6d8b1549").to_string().into_bytes();
    
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