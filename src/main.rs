use nostr_bot::{get_reply, Command, Event, EventNonSigned, FunctorType, State};

use std::time::SystemTime;

struct Info {
    started_at: std::time::SystemTime,
    jokes_told: u64,
}

async fn fetch_joke() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://v2.jokeapi.dev/joke/Any?format=txt").await?;

    Ok(response.text().await?)
}

async fn joke(event: Event, state: State<Info>) -> EventNonSigned {
    println!("Sending request to jokeapi.dev");

    let response = match fetch_joke().await {
        Ok(joke_text) => joke_text,
        Err(e) => format!("I was unable to get the joke: {}", e),
    };

    get_reply(event, response)
}

async fn stats(event: Event, state: State<Info>) -> EventNonSigned {
    let state = state.lock().await;
    let uptime_seconds = SystemTime::now()
        .duration_since(state.started_at)
        .unwrap()
        .as_secs();
    get_reply(
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
        "wss://nostr-pub.wellorder.net",
        "wss://relay.damus.io",
        "wss://relay.nostr.info",
    ];

    let mut secret = std::fs::read_to_string("secret").unwrap();
    secret.pop(); // Remove newline

    let keypair = nostr_bot::keypair_from_secret(&secret);

    let pic_url = "https://media.istockphoto.com/vectors/yellow-emoticons-and-emojis-vector-id1179177852?k=20&m=1179177852&s=612x612&w=0&h=5UrMXuV5x2WCI-d8twGI9gPZ6810wpgqyF9oy7X-r9M=";

    let state = nostr_bot::wrap_state(Info {
        started_at: SystemTime::now(),
        jokes_told: 0,
    });

    let mut bot = nostr_bot::Bot::<State<Info>>::new(keypair, relays, state)
        .name("joke_bot")
        .about("Just joking around. Blame https://sv443.net/jokeapi/v2/ if you don't like a joke.")
        .picture(pic_url)
        .intro_message("Wasup, I'm a joke bot. Reply to me with !help.")
        .command(Command::new("!joke", nostr_bot::wrap!(joke)).description("Tell a random joke."))
        .command(
            Command::new("!stats", nostr_bot::wrap!(stats))
                .description("Show for how long I'm running and how many jokes I told."),
        )
        .help();

    bot.run().await;
}
