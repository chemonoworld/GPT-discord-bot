use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GptRequest {
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct GptResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

❓ Need technical help?
➡️ Post in the <#1047422143358582784> channel and other humans will assist you.

❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>

❓ Something wrong?
➡️ You can flag an admin with @admin

I hope that resolves your issue!

— HelpBot 🤖
";

const HELP_COMMAND: &str = "!help";

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
