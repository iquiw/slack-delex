extern crate dotenv;
extern crate slack_api;

use std::env;
use std::process::exit;

use dotenv::dotenv;
use slack_api::requests::{Client, Error};
use slack_api::channels::ListError;

fn main() {
    dotenv().ok();

    let mut args = env::args();
    if let Some(channel) = args.nth(1) {
        println!("{}", channel);
        let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN is not set.");
        let client = slack_api::requests::default_client().unwrap();

        let id = find_channel_id(&client, &token, &channel);
        println!("{:?}", id);
    } else {
        eprintln!("usage: slack-delex CHANNEL_NAME");
        exit(1);
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
