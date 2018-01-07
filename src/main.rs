#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate slack_api;

use std::env;
use std::time;
use std::thread;

use clap::{Arg, App};
use dotenv::dotenv;

mod json;
mod slack;

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
        .arg(Arg::with_name("delay")
             .short("d")
             .long("delay")
             .value_name("DELAY")
             .help("Specify delay (ms) after one deletion")
             .default_value("900")
             .takes_value(true))
        .arg(Arg::with_name("JSON_FILE")
             .help("Specify JSON file exported from Slack")
             .multiple(true)
             .required(true)
             .index(1))
        .get_matches();
    let channel_name = matches.value_of("channel-name").unwrap();
    let dry_run = matches.is_present("dry-run");
    let delay = time::Duration::from_millis(value_t_or_exit!(matches, "delay", u64));
    let json_files = matches.values_of("JSON_FILE").unwrap();

    let client: Box<slack::DelexClient> = if dry_run {
        Box::new(slack::DryRunClient::new())
    } else {
        let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN is not set.");
        Box::new(slack::SlackDelexClient::new(&token))
    };

    let mut total = 0;
    match client.find_channel_id(&channel_name) {
        Ok(channel_id) => {
            println!("Channel: {}", channel_id);
            for json_file in json_files {
                println!("Processing: {}", &json_file);
                total += delete_message(&client, &channel_id, &json_file, delay);
            }
        },
        Err(err) => eprintln!("Channel list failed: {}", err),
    }
    println!("Total deleted: {}", total);
}

fn delete_message<C: AsRef<slack::DelexClient>>(client: C, channel_id: &str, json_file: &str, delay: time::Duration) -> u32 {
    let mut count = 0;
    let msgs = json::read_json(json_file).unwrap();
    for msg in msgs {
        let ts = msg.ts();
        match client.as_ref().delete_message(&channel_id, ts) {
            Ok(_) => {
                if client.as_ref().is_dry_run() {
                    println!("Would delete: {}", msg);
                } else {
                    println!("Message deleted: {}", msg);
                }
                count += 1;
            },
            Err(err) => eprintln!("Message delete failed: {}", err),
        }
        thread::sleep(delay);
    }
    count
}
