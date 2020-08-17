// Structure declaration
use serde::{Deserialize, Serialize};
use serde_json;

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

#[derive(Serialize, Deserialize)]
pub struct Icon {
	id: u32
}

#[derive(Serialize, Deserialize)]
pub struct PlayerClub {
	tag: String,
	name: String
}

#[derive(Serialize, Deserialize)]
pub struct Player {
	pub name: String,
	pub club: PlayerClub,
	pub icon: Icon,
	pub brawlers: Vec<Brawler>
}

impl Player {
	pub fn new(data: serde_json::Value) -> Player {
		let club: PlayerClub = serde_json::from_str(&data["club"].to_string())
			.unwrap();
		let name: String = data["name"].to_string();
		let icon: Icon = serde_json::from_str(&data["icon"].to_string())
			.unwrap();
		let brawlers: Vec<Brawler> = serde_json::from_str(&data["brawlers"].to_string())
			.unwrap();

		Player{
			name,
			club,
			icon,
			brawlers
		}
	}
}

