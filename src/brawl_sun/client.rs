use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

// Structure declaration
pub struct BrawlClient {
	pub token: String,
	web_client: Client
}

#[derive(Serialize, Deserialize)]
pub struct Ability {
	pub name: String,
	pub id: u32
}

#[derive(Serialize, Deserialize)]
pub struct Brawler {
	pub gadgets: Vec<Ability>,
	pub name: String,
	pub id: u32,
	pub starPowers: Vec<Ability>
}

#[derive(Serialize, Deserialize)]
pub struct BrawlerList {
	pub items: Vec<Brawler>
}


// Public implementation
impl BrawlClient {
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
	// Requests
	fn request(&self, url: &str) -> String {
		let auth: &str = &self.token;
		println!("> Requesting {} <", url);
		let request = self.web_client.get(url)
			.header(USER_AGENT, "BrawlSun Rust library")
			.header("Authorization", auth)
			.build()
			.unwrap();

		let response = self.web_client.execute(request).unwrap();
		let body = response.text().unwrap();

		return body
	}

	// Brawlers
	pub fn get_brawlers(&self) -> BrawlerList {
		let res = self.request("https://api.brawlstars.com/v1/brawlers");

		let v: BrawlerList = serde_json::from_str(&res)
			.expect("Function has panicked");

		return v
	}
}