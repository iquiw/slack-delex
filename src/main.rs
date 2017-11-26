extern crate dotenv;
extern crate slack_api;

use std::env;
use std::process::exit;

use dotenv::dotenv;
use slack_api::requests::{Client, Error};
use slack_api::channels::ListError;
use slack_api::chat::{DeleteError, DeleteRequest};

fn main() {
    dotenv().ok();

    let mut args = env::args();
    let arg1 = args.nth(1);
    let arg2 = args.next();
    if arg1.is_none() || arg2.is_none() {
        eprintln!("usage: slack-delex CHANNEL_NAME TS");
        exit(1);
    }
    let channel_name = arg1.unwrap();
    let ts = arg2.unwrap();

    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN is not set.");
    let client = slack_api::requests::default_client().unwrap();

    match find_channel_id(&client, &token, &channel_name) {
        Ok(channel_id) => {
            match delete_message(&client, &token, &channel_id, &ts) {
                Ok(_) => println!("Message deleted: {}", ts),
                Err(err) => eprintln!("Message delete failed: {}", err),
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
