use boardgamegeek_cli::{db, export, fetch_collection, filter, output};
use clap::Parser;
use rayon::prelude::*;

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

	/// How many players
	#[clap(short, long)]
	players: Option<i8>,

	/// Export to a TOML file
	#[clap(short, long)]
	export: bool,

	/// Export to SQLite
	#[clap(long)]
	db: bool,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let mut games = fetch_collection(&args.username).await;
	games = match &args.filter {
		Some(regex) => filter(&games, regex),
		None => games,
	};

	if let Some(players) = args.players {
		games = games
			.into_par_iter()
			.filter(|game| game.min_players <= players && game.max_players >= players)
			.collect()
	}

	output(&games);

	if args.export {
		export(&games);
	}

	if args.db {
		db(&games).await.unwrap();
	}
}
