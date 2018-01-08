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

use dotenv::dotenv;

mod cli;
mod json;
mod slack;

fn main() {
    dotenv().ok();

    let opts = cli::DelexOpts::parse_opts();

    let client: Box<slack::DelexClient> = if opts.dry_run {
        Box::new(slack::DryRunClient::new())
    } else {
        let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN is not set.");
        Box::new(slack::SlackDelexClient::new(&token))
    };

    let mut total = 0;
    match client.find_channel_id(&opts.channel_name) {
        Ok(channel_id) => {
            println!("Channel: {}", channel_id);
            for json_file in &opts.json_files {
                println!("Processing: {}", &json_file);
                total += delete_message(&client, &channel_id, &json_file, opts.delay);
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
