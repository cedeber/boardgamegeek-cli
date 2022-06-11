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
	username: Option<String>,

	/// Filter by title with a RegExp
	#[clap(short, long, requires = "username")]
	filter: Option<String>,

	/// How long you want to play, in minutes. (+/- 10 minutes)
	#[clap(short, long, requires = "username")]
	time: Option<i64>,

	/// How many players
	#[clap(short, long, requires = "username")]
	players: Option<i64>,

	/// Export to a TOML file
	#[clap(short, long, requires = "username")]
	export: bool,

	/// Export to SQLite
	#[clap(long, requires = "username")]
	db: bool,

	/// Run server
	#[clap(long)]
	serve: bool,
}

#[tokio::main]
async fn main() {
	// parse the CLI arguments
	let args = Args::parse();

	if let Some(username) = &args.username {
		// Fetch all games from BGG
		let games = fetch_collection(username).await;

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
				.filter(|game| {
					game.min_players.unwrap_or_default() <= players
						&& game.max_players.unwrap_or_default() >= players
				})
				.collect()
		}

		// Filter the games by time (+/- 10 minutes)
		if let Some(time) = args.time {
			games = games
				.into_iter()
				.filter(|game| {
					let playtime = game.playtime.unwrap_or_default();
					playtime <= time + 10 && playtime >= time - 10
				})
				.collect()
		}

		// Export to TOML
		if args.export {
			export(&games);
		}

		// Write/Update the list into a SQLite file
		if args.db {
			db(username, &games).await.unwrap();
		}

		// Output the list of filtered games in the console.
		if !args.export || !args.db || !args.serve {
			output(&games);
		}
	}

	if args.serve {
		server::run().await;
	}
}
