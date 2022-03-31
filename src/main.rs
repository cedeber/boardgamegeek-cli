use boardgamegeek_cli::{fetch_collection, GameBoard};
use clap::Parser;
use console::style;
use rayon::prelude::*;
use regex::Regex;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

// @see https://boardgamegeek.com/wiki/page/BGG_XML_API
// @see https://boardgamegeek.com/xmlapi/collection/cedeber

/// Simple program to list all board games from a BoardGameGeek user.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// BoardGameGeek Username
	#[clap()]
	username: String,

	/// Filter by title with a RegExp
	#[clap(short, long)]
	filter: Option<String>,

	/// How long you want to play, in minutes.
	#[clap(short, long)]
	time: Option<i8>,

	/// Export to a TOML file
	#[clap(short, long)]
	export: bool,
}

#[derive(Debug, Clone, Serialize)]
struct GameExport {
	games: Vec<GameBoard>,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let mut games = fetch_collection(&args.username).await;

	// TODO Fuzzy search
	if let Some(filter) = &args.filter {
		let re = Regex::new(filter).unwrap();

		games = games
			.into_par_iter()
			.filter(|game| re.find(&game.name).is_some())
			.collect();
	}

	// Output
	for game in &games {
		println!(
			"{} {} {}",
			style(&game.year).cyan(),
			style(format!("{:3}m", &game.playtime)).green(),
			&game.name
		);
	}

	// Export
	if args.export {
		let export = GameExport { games };

		let toml = toml::to_string(&export).unwrap();
		let path = "export.toml";

		let mut output = File::create(path).unwrap();
		output.write_all(toml.as_ref()).unwrap();
	}
}
