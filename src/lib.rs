use console::{style, Term};
use rayon::prelude::*;
use regex::Regex;
use serde::Serialize;
use sqlx::{query, SqlitePool};
use std::{fs::File, io::Write};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Serialize)]
struct GameExport {
	games: Vec<GameBoard>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameBoard {
	pub id: String,
	pub name: String,
	pub year: String,
	pub min_players: i8,
	pub max_players: i8,
	pub playtime: i16, // minutes
}

pub async fn fetch_collection(username: &str) -> Vec<GameBoard> {
	let mut games: Vec<GameBoard> = Vec::new();

	let resp = reqwest::get(format!(
		"https://boardgamegeek.com/xmlapi/collection/{username}"
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
		let id = node.attribute("objectid").unwrap();
		let mut children = node.children();
		let name = children.find(|n| n.has_tag_name("name"));
		let year = children.find(|n| n.has_tag_name("yearpublished"));
		let stats = children.find(|n| n.has_tag_name("stats"));
		let mut playtime = Some("0");
		let mut min_players = Some("0");
		let mut max_players = Some("0");

		if let Some(stats) = stats {
			playtime = stats.attribute("playingtime");
			min_players = stats.attribute("minplayers");
			max_players = stats.attribute("maxplayers");
		}

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

			games.push(GameBoard {
				id: String::from(id),
				name: String::from(name),
				year: String::from(year),
				playtime,
				min_players,
				max_players,
			});
		}
	}

	games
}

// TODO Fuzzy search
pub fn filter(games: &[GameBoard], regex: &str) -> Vec<GameBoard> {
	let re = Regex::new(regex).unwrap();

	games
		.to_owned()
		.into_par_iter()
		.filter(|game| re.find(&game.name).is_some())
		.collect()
}

pub fn output(games: &[GameBoard]) {
	let mut term = Term::stdout();
	let title_max_length: usize = games
		.into_par_iter()
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

pub fn export(games: &[GameBoard]) {
	let export = GameExport {
		games: games.to_owned(),
	};

	let toml = toml::to_string(&export).unwrap();
	let path = "export.toml";

	let mut output = File::create(path).unwrap();
	output.write_all(toml.as_ref()).unwrap();
}

pub async fn db(games: &[GameBoard]) -> Result<(), sqlx::Error> {
	// Check .env
	let pool = SqlitePool::connect("sqlite:games.sqlite").await?;

	for game in games {
		let _ = query!(
			// language=SQLite
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
