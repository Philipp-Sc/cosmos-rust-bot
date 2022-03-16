/*
 * Only use APIs defined here.
 *
 */

pub fn get_spectrumprotocol_api() -> String {
	let terra_endpoint = "https://specapi.azurefd.net";
	terra_endpoint.to_string()
}

pub fn get_anchorprotocol_airdrop_api() -> String {
	let terra_endpoint = "https://airdrop.anchorprotocol.com";
	terra_endpoint.to_string()
}

pub fn get_anchorprotocol_api() -> String {
	let terra_endpoint = "https://api.anchorprotocol.com";
	terra_endpoint.to_string()
}

pub fn get_terra_fcd() -> String {
	let terra_endpoint = "https://fcd.terra.dev";
	terra_endpoint.to_string()
}
pub fn get_terra_lcd() -> String {
	let terra_endpoint = "https://lcd.terra.dev";
	terra_endpoint.to_string()
}
pub fn get_terra_chain() -> String {
	let terra_endpoint = "columbus-5";
	terra_endpoint.to_string()
}