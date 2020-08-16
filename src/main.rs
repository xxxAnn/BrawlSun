mod brawl_sun;

fn main() {
	let secret = ""; // Secret token here
	let client = brawl_sun::client::BrawlClient::new(secret);
	let brawlers = client.get_brawlers();
	println!("{}", brawlers.items[0].name)
}