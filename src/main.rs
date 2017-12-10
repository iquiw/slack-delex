extern crate clap;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate slack_api;

use std::env;

use clap::{Arg, App};
use dotenv::dotenv;
use slack_api::requests::{Client, Error};
use slack_api::channels::ListError;
use slack_api::chat::{DeleteError, DeleteRequest};

mod json;

fn main() {
    dotenv().ok();

    let matches = App::new("slack-delex")
        .arg(Arg::with_name("channel-name")
             .short("c")
             .long("channel-name")
             .value_name("CHANNEL_NAME")
             .help("Specify channel name")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("dry-run")
             .short("n")
             .long("dry-run"))
        .arg(Arg::with_name("JSON_FILE")
             .help("Specify JSON file exported from Slack")
             .required(true)
             .index(1))
        .get_matches();
    let channel_name = matches.value_of("channel-name").unwrap();
    let dry_run = matches.is_present("dry-run");
    let json_file = matches.value_of("JSON_FILE").unwrap();

    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN is not set.");
    let client = slack_api::requests::default_client().unwrap();

    match find_channel_id(&client, &token, &channel_name) {
        Ok(channel_id) => {
            println!("Channel: {}", channel_id);
            let msgs = json::read_json(json_file).unwrap();
            for msg in msgs {
                let ts = msg.ts();
                if dry_run {
                    println!("Would delete: {}", ts);
                } else {
                    match delete_message(&client, &token, &channel_id, ts) {
                        Ok(_) => println!("Message deleted: {}", ts),
                        Err(err) => eprintln!("Message delete failed: {}", err),
                    }
                }
            }
        },
        Err(err) => eprintln!("Channel list failed: {}", err),
    }
}

fn find_channel_id(client: &Client, token: &str, name: &str) -> Result<String, ListError<Error>>
{
    let response = slack_api::channels::list(client, token, &Default::default())?;
    if let Some(channels) = response.channels {
        for channel in channels {
            if let Some(n) = channel.name {
                if n == name {
                    return channel.id.ok_or("ID is not avaiable".into());
                }
            }
        }
    }
    Err("Channel not found".into())
}

fn delete_message(client: &Client, token: &str, channel_id: &str, ts: &str) -> Result<(), DeleteError<Error>> {
    let request = DeleteRequest {
        channel: channel_id,
        ts: ts,
        as_user: None,
    };
    let _ = slack_api::chat::delete(client, token, &request)?;
    Ok(())
}
