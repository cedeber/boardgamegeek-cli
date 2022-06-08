use boardgamegeek_cli::{db, export, fetch_collection, filter, output, server};
use clap::Parser;

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

	/// How long you want to play, in minutes. (+/- 10 minutes)
	#[clap(short, long)]
	time: Option<i16>,

	/// How many players
	#[clap(short, long)]
	players: Option<i8>,

	/// Export to a TOML file
	#[clap(short, long)]
	export: bool,

	/// Export to SQLite
	#[clap(long)]
	db: bool,

	/// Run server
	#[clap(long)]
	serve: bool,
}

#[tokio::main]
async fn main() {
	// parse the CLI arguments
	let args = Args::parse();

	// Fetch all games from BGG
	let games = fetch_collection(&args.username).await;

	if games.is_err() {
		println!("Fetching the games in BGG failed: {}", games.err().unwrap());
		return;
	}

	let mut games = games.unwrap();

	// Apply the regex filter if any
	games = match &args.filter {
		Some(regex) => filter(&games, regex),
		None => games,
	};

	// Filter the games by number of players
	if let Some(players) = args.players {
		games = games
			.into_iter()
			.filter(|game| game.min_players <= players && game.max_players >= players)
			.collect()
	}

	// Filter the games by time (+/- 10 minutes)
	if let Some(time) = args.time {
		games = games
			.into_iter()
			.filter(|game| game.playtime <= time + 10 && game.playtime >= time - 10)
			.collect()
	}

	// Export to TOML
	if args.export {
		export(&games);
	}

	// Write/Update the list into a SQLite file
	if args.db {
		db(&games).await.unwrap();
	}

	if args.serve {
		db(&games).await.unwrap();
		server::run().await;
	}

	// Output the list of filtered games in the console.
	if !args.export || !args.db || !args.serve {
		output(&games);
	}
}
