mod brawl_sun;

fn main() {
	let secret = "" // Your API token here
	let client = brawl_sun::client::BrawlClient::new(secret);
	let player = client.get_player("#28CLCJG");
	println!("{}", player.brawlers[0].name);
}