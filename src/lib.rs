use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBoard {
	pub name: String,
	pub year: String,
	// min_players: i8,
	// max_players: i8,
	pub playtime: i8, // minutes
}

pub async fn fetch_collection(username: &str) -> Vec<GameBoard> {
	let mut games: Vec<GameBoard> = Vec::new();

	let resp = reqwest::get(format!(
		"https://boardgamegeek.com/xmlapi/collection/{}",
		username
	))
	.await
	.unwrap()
	.text()
	.await
	.unwrap();

	let doc = match roxmltree::Document::parse(&resp) {
		Ok(doc) => doc,
		Err(_) => {
			return games;
		}
	};

	for node in
		doc.descendants()
			.filter(|n| n.has_tag_name("item"))
			.filter(|n| n.attribute("subtype") == Some("boardgame"))
			.filter(|n| {
				n.descendants()
					.find(|n| n.has_tag_name("status"))
					.unwrap()
					.attribute("own") == Some("1")
			}) {
		let mut children = node.children();
		let name = children.find(|n| n.has_tag_name("name"));
		let year = children.find(|n| n.has_tag_name("yearpublished"));
		let playtime = children
			.find(|n| n.has_tag_name("stats"))
			.unwrap()
			.attribute("playingtime");

		if let Some(name) = name {
			let name = name.text().unwrap();
			let year = match year {
				// TODO Check if it's negative
				Some(year) => year.text().unwrap(),
				None => "    ",
			};
			let playtime: i8 = match playtime {
				Some(time) => time.parse().unwrap_or(0),
				None => 0,
			};

			games.push(GameBoard {
				name: String::from(name),
				year: String::from(year),
				playtime,
			});
		}
	}

	games
}
