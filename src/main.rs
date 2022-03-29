use clap::Parser;
use console::style;
use regex::Regex;

// @see https://boardgamegeek.com/wiki/page/BGG_XML_API
// @see https://boardgamegeek.com/xmlapi/collection/cedeber

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Username
	#[clap()]
	username: String,

	/// Filter by title with a RegExp
	#[clap(short, long)]
	filter: Option<String>,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let resp = reqwest::get(format!(
		"https://boardgamegeek.com/xmlapi/collection/{}",
		&args.username
	))
	.await
	.unwrap()
	.text()
	.await
	.unwrap();

	let doc = match roxmltree::Document::parse(&resp) {
		Ok(doc) => doc,
		Err(e) => {
			println!("Error: {}.", e);
			return;
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

		if let Some(name) = name {
			let name = name.text().unwrap();
			let year = match year {
				// TODO Check if it's negative
				Some(year) => year.text().unwrap(),
				None => "    ",
			};

			// TODO Implement fuzzy search
			if let Some(filter) = &args.filter {
				let re = Regex::new(filter).unwrap();

				if re.find(name).is_some() {
					println!("{} {}", style(year).cyan(), name);
				}
			} else {
				println!("{} {}", style(year).cyan(), name);
			}
		}
	}
}
