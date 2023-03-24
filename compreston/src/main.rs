use anyhow::anyhow;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::builder::CreateButton;
use serenity::prelude::*;
use serenity::client::{Context, EventHandler};
use serenity::futures::StreamExt;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

fn upload_button(id: &str, text: &str) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.label(text);
    b.style(ButtonStyle::Primary);
    b
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content != "!compreston" {
            return;
        }

        // 
        // This is where I need to begin in order to add stuff that the bot can do!
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel {:?}", why);
                    return;
                }
            };
            // This allows for creating a message by mentioning users dynamically
            let response = MessageBuilder::new() 
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the ping command in the ")
                .mention(&channel)
                .push(" channel")
                .build();
            if let Err(e) = msg.channel_id.say(&ctx.http, &response).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}