use brawl_sun;
use std::fs;

#[test]
fn test_get_club() {
	let secret = fs::read_to_string("resources/secret.txt")
		.expect("Unable to read file");
	let client = brawl_sun::client::new(&secret);
	let club = client.get_club("#V8RP28U2");
	assert_eq!(club.tag, "#V8RP28U2");
}