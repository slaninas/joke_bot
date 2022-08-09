use nostr_bot::{format_reply, wrap, Command, Event, EventNonSigned, FunctorType, Network, State};

use std::time::SystemTime;

struct Info {
    started_at: std::time::SystemTime,
    jokes_told: u64,
}

async fn joke(event: Event, state: State<Info>) -> EventNonSigned {
    println!("Sending request to jokeapi.dev");

    let error_text = String::from("I'm unable to connect to jokeapi.dev. No jokes, I'm serious.");

    let conn = reqwest::get("https://v2.jokeapi.dev/joke/Any?format=txt").await;

    let text = match conn {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => {
                        state.lock().await.jokes_told += 1;
                        text
                    }
                    Err(_) => error_text,
                }
            } else {
                error_text
            }
        }
        Err(_) => error_text,
    };

    format_reply(event, text)
}

async fn stats(event: Event, state: State<Info>) -> EventNonSigned {
    let state = state.lock().await;
    let uptime_seconds = SystemTime::now()
        .duration_since(state.started_at)
        .unwrap()
        .as_secs();
    format_reply(
        event,
        format!(
            "I'm up {} and I told {} jokes since my last crash.",
            compound_duration::format_dhms(uptime_seconds),
            state.jokes_told
        ),
    )
}

#[tokio::main]
async fn main() {
    nostr_bot::init_logger();

    let relays = vec![
        String::from("wss://nostr-pub.wellorder.net"),
        String::from("wss://relay.damus.io"),
        String::from("wss://relay.nostr.info"),
    ];

    let mut secret = std::fs::read_to_string("secret").unwrap();
    secret.pop(); // Remove newline

    let secp = secp256k1::Secp256k1::new();
    let keypair = secp256k1::KeyPair::from_seckey_str(&secp, &secret).unwrap();

    let pic_url = "https://media.istockphoto.com/vectors/yellow-emoticons-and-emojis-vector-id1179177852?k=20&m=1179177852&s=612x612&w=0&h=5UrMXuV5x2WCI-d8twGI9gPZ6810wpgqyF9oy7X-r9M=";

    let mut bot = nostr_bot::Bot::<State<Info>>::new(keypair, relays, Network::Clearnet)
        .set_name("joke_bot")
        .set_about(
            "Just joking around. Blame https://sv443.net/jokeapi/v2/ if you don't like a joke.",
        )
        .set_picture(pic_url)
        .set_intro_message("Wasup, I'm a joke bot. Reply to me with !help.")
        .command(Command::new("!joke", nostr_bot::wrap!(joke)).desc("Tell a random joke."))
        .command(Command::new("!stats", nostr_bot::wrap!(stats)).desc("Show for how long I'm running and how many jokes I told."))
        .help();

    let state = nostr_bot::wrap_state(Info {
        started_at: SystemTime::now(),
        jokes_told: 0,
    });

    bot.run(state).await;
}
