use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use crate::model::structs::{Player, BrawlerList, Club};

/// a client that makes request to the API
pub struct BrawlClient {
	pub token: String,
	web_client: Client
}

// Public implementation
impl BrawlClient {
	/// Private meth that makes a request to the API with the passed url
	///
	/// # Panics
	/// 
	/// Code will Panic if it receives an HTTP code in the 400's range
	///
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
			panic!("\x1b[0;31mThere was an error with the request, Status: {}\x1b[0m", response.status().as_str())
		}
		let body = response.text().unwrap();

		return body
	}
	
	/// Public meth that returns a list of all Brawlers
	///
	/// Will call the `request` meth internally
	///
	/// Will Panic if serde_json::from_str fails to convert the HTTP respbody
	///
	pub fn get_brawlers(&self) -> BrawlerList {
		let res = self.request("https://api.brawlstars.com/v1/brawlers");

		let v: BrawlerList = serde_json::from_str(&res)
			.expect("\x1b[0;31mCould not convert string to struct\x1b[0m");

		return v
	}

	/// Public meth that returns a list a `Player` struct
	///
	/// Will call the `request` meth internally
	///
	/// Will Panic if serde_json::from_str fails to convert the HTTP respbody
	///
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
			.expect("\x1b[0;31mCould not convert string to struct\x1b[0m");

		return Player::new(v)
	}

	/// Public meth that returns a list a `Club` struct
	///
	/// Will call the `request` meth internally
	///
	/// Will Panic if serde_json::from_str fails to convert the HTTP respbody
	///
	pub fn get_club(&self, tag: &str) -> Club {
		// Obtaining the url
		let mut url = "https://api.brawlstars.com/v1/clubs/"
			.to_owned();
		url.push_str(tag);
		let url = url.replace("#", "%23");
		// Making the request
		let res = self.request(&url);
		// Returning a serde_json::Value (temporarily)
		let v: Club = serde_json::from_str(&res)
			.expect("\x1b[0;31mCould not convert string to struct\x1b[0m");

		return v
	}
}

/// Public meth for creating `BrawlClient` structs
///
/// De facto way of obtaining a `BrawlClient` struct
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