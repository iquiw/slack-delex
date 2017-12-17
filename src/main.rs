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
             .default_value("300")
             .takes_value(true))
        .arg(Arg::with_name("JSON_FILE")
             .help("Specify JSON file exported from Slack")
             .required(true)
             .index(1))
        .get_matches();
    let channel_name = matches.value_of("channel-name").unwrap();
    let dry_run = matches.is_present("dry-run");
    let delay = time::Duration::from_millis(value_t_or_exit!(matches, "delay", u64));
    let json_file = matches.value_of("JSON_FILE").unwrap();

    let client: Box<slack::DelexClient> = if dry_run {
        Box::new(slack::DryRunClient::new())
    } else {
        let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN is not set.");
        Box::new(slack::SlackDelexClient::new(&token))
    };

    match client.find_channel_id(&channel_name) {
        Ok(channel_id) => {
            println!("Channel: {}", channel_id);
            let msgs = json::read_json(json_file).unwrap();
            for msg in msgs {
                let ts = msg.ts();
                match client.delete_message(&channel_id, ts) {
                    Ok(_) => if dry_run {
                        println!("Would delete: {}", ts);
                    } else {
                        println!("Message deleted: {}", ts);
                    },
                    Err(err) => eprintln!("Message delete failed: {}", err),
                }
                thread::sleep(delay);
            }
        },
        Err(err) => eprintln!("Channel list failed: {}", err),
    }
}
