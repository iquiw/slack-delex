use failure::{Error, err_msg};

use slack_api;
use slack_api::requests::Client;
use slack_api::chat::DeleteRequest;

pub trait DelexClient {
    fn find_channel_id(&self, name: &str) -> Result<String, Error>;
    fn delete_message(&self, channel_id: &str, ts: &str) -> Result<(), Error>;
    fn is_dry_run(&self) -> bool { false }
}


pub struct SlackDelexClient {
    client: Client,
    token: String,
}

pub struct DryRunClient;

impl SlackDelexClient {
    pub fn new(token: &str) -> Self {
        let client = slack_api::requests::default_client().unwrap();
        SlackDelexClient {
            client: client,
            token: token.to_string()
        }
    }
}

impl DelexClient for SlackDelexClient {
    fn find_channel_id(&self, name: &str) -> Result<String, Error>
    {
        let response = slack_api::channels::list(&self.client, &self.token, &Default::default())?;
        if let Some(channels) = response.channels {
            for channel in channels {
                if let Some(n) = channel.name {
                    if n == name {
                        return channel.id.ok_or(err_msg("ID is not avaiable"));
                    }
                }
            }
        }
        Err(err_msg("Channel not found"))
    }

    fn delete_message(&self, channel_id: &str, ts: &str) -> Result<(), Error> {
        let request = DeleteRequest {
            channel: channel_id,
            ts: ts,
            as_user: None,
        };
        let _ = slack_api::chat::delete(&self.client, &self.token, &request)?;
        Ok(())
    }
}

impl DryRunClient {
    pub fn new() -> Self { DryRunClient }
}

impl DelexClient for DryRunClient {
    fn find_channel_id(&self, _name: &str) -> Result<String, Error> {
        Ok("DUMMY_ID".to_string())
    }

    fn delete_message(&self, _channel_id: &str, _ts: &str) -> Result<(), Error> {
        Ok(())
    }

    fn is_dry_run(&self) -> bool { true }
}
