use brawl_sun;
use std::fs;


#[test]
fn test_get_brawlers() {
	let secret = fs::read_to_string("resources/secret.txt")
		.expect("Unable to read file");
	let client = brawl_sun::client::client::BrawlClient::new(&secret);
	let player = client.get_player("#28CLCJG");
}