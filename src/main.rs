use boardgamegeek_cli::fetch_collection;
use clap::Parser;
use console::style;
use rayon::prelude::*;
use regex::Regex;

// @see https://boardgamegeek.com/wiki/page/BGG_XML_API
// @see https://boardgamegeek.com/xmlapi/collection/cedeber

/// Simple program to list all board games from a BoardGameGeek user.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Username
	#[clap()]
	username: String,

	/// Filter by title with a RegExp
	#[clap(short, long)]
	filter: Option<String>,

	/// How long you want to play, in minutes.
	#[clap(short, long)]
	time: Option<i8>,
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
	for game in games {
		println!(
			"{} {} {}",
			style(game.year).cyan(),
			style(format!("{:3}m", game.playtime)).green(),
			game.name
		);
	}
}
