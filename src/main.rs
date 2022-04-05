use boardgamegeek_cli::{db, export, fetch_collection, filter, output};
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

	/// How long you want to play, in minutes.
	#[clap(short, long)]
	time: Option<i8>,

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

	let games = fetch_collection(&args.username).await;
	let games = match &args.filter {
		Some(regex) => filter(&games, regex),
		None => games,
	};

	output(&games);

	if args.export {
		export(&games);
	}

	if args.db {
		db(&games).await.unwrap();
	}
}
