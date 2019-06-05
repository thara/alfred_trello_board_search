extern crate reqwest;

extern crate serde;

use std::env;
use structopt::StructOpt;

use serde::{Deserialize, Serialize};

#[derive(StructOpt)]
struct Cli {
    user: String,
}

#[derive(Deserialize, Debug)]
struct TrelloBoard {
    name: String,
    #[serde(rename = "desc")]
    description: String,
    url: String,
    #[serde(rename = "shortUrl")]
    short_url: String,
    starred: bool,
    #[serde(rename = "dateLastView")]
    date_last_view: String,
}

#[derive(Serialize, Debug)]
struct AlfredInputItem {
    uid: String,
    title: String,
    subtitle: String,
    arg: String,
    autocomplete: String,
}

#[derive(Serialize, Debug)]
struct AlfredInput {
    items: Vec<AlfredInputItem>,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let key = env::var("TRELLO_KEY").expect("Required TRELLO_KEY in environment variables");
    let token = env::var("TRELLO_TOKEN").expect("Required TRELLO_TOKEN in environment variables");

    let args = Cli::from_args();

    let url = format!(
        "https://api.trello.com/1/members/{user}/boards",
        user = &args.user
    );

    let client = reqwest::Client::new();
    let mut boards: Vec<TrelloBoard> = client
        .get(&url)
        .query(&[
            ("key", key.as_ref()),
            ("token", token.as_ref()),
            ("filter", "open"),
            ("memberships", "all"),
            ("fields", "name,desc,url,shortUrl,starred,dateLastView"),
        ])
        .send()?
        .json()?;

    boards.sort_by(|a, b| b.date_last_view.cmp(&a.date_last_view));

    let items = boards
        .iter()
        .map(|e| AlfredInputItem {
            uid: (&e).url.to_string(),
            title: (&e).name.to_string(),
            subtitle: (&e).description.to_string(),
            arg: (&e).short_url.to_string(),
            autocomplete: "Trello Boards".to_string(),
        })
        .collect();

    let input = AlfredInput { items: items };

    let j = serde_json::to_string(&input)?;
    println!("{}", j);
    Ok(())
}
