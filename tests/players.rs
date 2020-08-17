use brawl_sun;
use std::fs;


#[test]
fn test_get_players() {
	let secret = fs::read_to_string("resources/secret.txt")
		.expect("Unable to read file");
	let client = brawl_sun::client::client::new(&secret);
	let player = client.get_player("#28CLCJG");
}