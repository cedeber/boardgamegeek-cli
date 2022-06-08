pub mod server;

use async_graphql::SimpleObject;
use console::{style, Term};
use regex::Regex;
use serde::Serialize;
use sqlx::{query, SqlitePool};
use std::{fs::File, io::Write};
use unicode_segmentation::UnicodeSegmentation;

// Used for the TOML export. It will export by something like { games: [..] }
#[derive(Debug, Clone, Serialize)]
struct GameExport {
	games: Vec<BoardGame>,
}

// Each board game struct
#[derive(Debug, Clone, Serialize, SimpleObject)]
pub struct BoardGame {
	pub id: String,
	pub name: String,
	pub year: String,
	pub min_players: i8,
	pub max_players: i8,
	pub playtime: i16, // minutes
}

// Fetch the list of games for a specific user from BGG.
// Also parse the XML and return a list of BoardGames.
pub async fn fetch_collection(username: &str) -> Result<Vec<BoardGame>, reqwest::Error> {
	let mut games: Vec<BoardGame> = Vec::new();

	// Fetch the BGG API ans save the XML as text.
	let resp = reqwest::get(format!(
		"https://boardgamegeek.com/xmlapi/collection/{username}"
	))
	.await?
	.text()
	.await
	.unwrap(); // parsing to text should be fine.

	// Parse the XML.
	let doc = match roxmltree::Document::parse(&resp) {
		Ok(doc) => doc,
		Err(_) => {
			// TODO Export parsing error
			return Ok(games);
		}
	};

	// Check if the returned XML does not contain <message>
	if doc.root_element().has_tag_name("message") {
		let message = doc.root_element().text().unwrap();
		println!("{}", message.trim());
		return Ok(vec![]);
	}

	// Check if the returned XML does not contain <errors><error><message>
	if doc.root_element().has_tag_name("errors") {
		let message = doc
			.root_element() // <errors>
			.first_element_child() // <error>
			.unwrap()
			.first_element_child() // <message>
			.unwrap()
			.text()
			.unwrap();
		println!("Error: {}", message.trim());
		return Ok(vec![]);
	}

	// Go trough the XML Nodes and extract useful information.
	// We use unwrap because we believe the XML API is stable and trustable.
	// Crashing the application here means that the API changed anyway.
	for node in
		// If I own the game
		doc
			.descendants()
			.filter(|n| n.has_tag_name("item"))
			.filter(|n| n.attribute("subtype") == Some("boardgame"))
			.filter(|n| {
				n.descendants()
					.find(|n| n.has_tag_name("status"))
					.unwrap()
					.attribute("own") == Some("1")
			}) {
		let id = node.attribute("objectid").unwrap();
		let mut children = node.children();
		let name = children.find(|n| n.has_tag_name("name"));
		let year = children.find(|n| n.has_tag_name("yearpublished"));
		let stats = children.find(|n| n.has_tag_name("stats"));

		// Default values
		let mut playtime = Some("0");
		let mut min_players = Some("0");
		let mut max_players = Some("0");

		if let Some(stats) = stats {
			playtime = stats.attribute("playingtime");
			min_players = stats.attribute("minplayers");
			max_players = stats.attribute("maxplayers");
		}

		// If the game has a name, extract data and add it to the games list.
		if let Some(name) = name {
			let name = name.text().unwrap();
			let year = match year {
				// TODO Check if it's negative
				Some(year) => year.text().unwrap(),
				None => "    ",
			};

			let playtime: i16 = match playtime {
				Some(time) => time.parse().unwrap_or(0),
				None => 0,
			};

			let min_players: i8 = match min_players {
				Some(qqty) => qqty.parse().unwrap_or(0),
				None => 0,
			};

			let max_players: i8 = match max_players {
				Some(qqty) => qqty.parse().unwrap_or(0),
				None => 0,
			};

			games.push(BoardGame {
				id: String::from(id),
				name: String::from(name),
				year: String::from(year),
				playtime,
				min_players,
				max_players,
			});
		}
	}

	// return the games list
	Ok(games)
}

// TODO Fuzzy search
// Filter the games by name
pub fn filter(games: &[BoardGame], regex: &str) -> Vec<BoardGame> {
	let re = Regex::new(regex).unwrap();

	games
		.iter()
		.cloned()
		.filter(|game| re.find(&game.name).is_some())
		.collect()
}

// Output the games list to the console
pub fn output(games: &[BoardGame]) {
	// Keep to show clippy hint ;-)
	if games.len() == 0 {
		println!("No games found.");
		return;
	}

	let mut term = Term::stdout();
	let title_max_length: usize = games
		.iter()
		.map(|g| g.name.graphemes(true).count())
		.max_by(|x, y| x.cmp(y))
		.unwrap();

	for game in games {
		let text = format!(
			"⎮ {} ⎮ {} ⎮ {} ⎮ {}",
			style(format!(
				"{:1} ⇢{:2}p",
				&game.min_players.min(99),
				&game.max_players.min(99)
			))
			.magenta(),
			style(format!("{:3}m", &game.playtime.min(999))).green(),
			style(format!("{:4}", &game.year)).cyan(),
			&game.name
		);

		// term.move_cursor_right(title_max_length + 1).unwrap();
		// term.write_all("⎮".as_bytes()).unwrap();
		term.write_all(text.as_bytes()).unwrap();
		term.move_cursor_right(title_max_length - game.name.graphemes(true).count() + 1)
			.unwrap();
		term.write_line("⎮").unwrap();
	}
}

// Export to TOML
pub fn export(games: &[BoardGame]) {
	let export = GameExport {
		games: games.to_owned(),
	};

	let toml = toml::to_string(&export).unwrap();
	let path = "export.toml";

	let mut output = File::create(path).unwrap();
	output.write_all(toml.as_ref()).unwrap();
}

// Save to SQLite DB
pub async fn db(games: &[BoardGame]) -> Result<(), sqlx::Error> {
	// Check .env
	let pool = SqlitePool::connect("sqlite:games.sqlite").await?;

	for game in games {
		let _ = query!(
			r#"
INSERT OR REPLACE INTO boardgames (gameid, title, published, playing_time, min_players, max_players)
VALUES ( ?1, ?2, ?3, ?4, ?5, ?6 )
	"#,
			game.id,
			game.name,
			game.year,
			game.playtime,
			game.min_players,
			game.max_players
		)
		.execute(&pool)
		.await?;
	}

	Ok(())
}
