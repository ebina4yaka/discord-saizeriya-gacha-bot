use anyhow::Context as _;
use serenity::all::GuildId;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::info;

mod commands;
mod data;

struct Handler {
    secret_store: SecretStore,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            self.secret_store
                .get("GUILD_ID")
                .context("'GUILD_ID' was not found")
                .unwrap()
                .parse()
                .unwrap(),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![commands::saizeriya_gacha::register()])
            .await;

        info!("I now have the following guild slash commands: {commands:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            info!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "saizeriya_gacha" => Some(commands::saizeriya_gacha::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    info!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler {
            secret_store: secrets,
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
