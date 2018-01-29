# Slack Delex

Delete Slack messages with exported JSON files.

## Install

``` console
$ cargo install --git https://github.com/iquiw/slack-delex
```

## Usage

1. Start export workspace's messages from https://my.slack.com/services/export

2. Download and extract the zip file.

3. Run `slack-delex` with channel name and extracted JSON files as arguments.

   ``` console
   $ slack-delex -c general general/2017-*.json
   ```

### Options

- `-n`, `--dry-run`: Not actually deletes messages (dry-run)

- `-c`, `--channel-name <CHANNEL_NAME>`: Specify channel name
- `-d`, `--delay <DELAY>`: Specify delay (ms) after one deletion (default: 900)
- `--subtype <subtype>`: Specify subtype to be deleted
