Example of using [nostr-bot](https://github.com/slaninas/nostr-bot) to build bot telling jokes (from https://jokeapi.dev)

## How to build&run
```
git clone https://github.com/slaninas/joke_bot/
cd joke_bot
# Put your secret key (in hex format) into file named secret
cargo run --release # or run ./build_and_run.sh if you want to use Docker
