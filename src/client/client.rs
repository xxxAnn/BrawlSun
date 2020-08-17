use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use crate::model::structs::{Player, BrawlerList};

pub struct BrawlClient {
	pub token: String,
	web_client: Client
}

// Public implementation
impl BrawlClient {
	// Requests
	fn request(&self, url: &str) -> String {
		let auth: &str = &self.token;
		println!("> Requesting {} <", url);
		// Building the request
		let request = self.web_client.get(url)
			.header(USER_AGENT, "BrawlSun Rust library")
			.header("Authorization", auth)
			.build()
			.unwrap();

		// Executing it
		let response = self.web_client.execute(request).unwrap();
		if response.status().is_client_error() {
			panic!("There was an error with the request")
		}
		let body = response.text().unwrap();

		return body
	}

	pub fn get_brawlers(&self) -> BrawlerList {
		let res = self.request("https://api.brawlstars.com/v1/brawlers");

		let v: BrawlerList = serde_json::from_str(&res)
			.expect("Function has panicked");

		return v
	}

	pub fn get_player(&self, tag: &str) -> Player {
		// Obtaining the url
		let mut url = "https://api.brawlstars.com/v1/players/"
				.to_owned();
		url.push_str(tag);
		let url = url.replace("#", "%23");
		// Making the request
		let res = self.request(&url);
		// Returning the player struct
		let v: serde_json::Value = serde_json::from_str(&res)
			.expect("Function has panicked");

		return Player::new(v)
	}
}

// Factory
pub fn new(token: &str) -> BrawlClient {
	let mut authorization: String = "Bearer ".to_owned();
	authorization.push_str(token);
	let client = Client::new();
	// Construction
	BrawlClient {
	token: authorization,
	web_client: client
	}
}