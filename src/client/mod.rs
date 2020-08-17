pub mod client;

pub fn new(token: &str) -> client::BrawlClient {
	return client::new(&token);
}