use poise::serenity_prelude::*;

mod commands;

#[derive(serde::Deserialize)]
struct Config {
    #[serde(rename = "BOT_TOKEN")]
    token: String,
    #[serde(rename = "GUILD_ID")]
    guild_id: u64,
    #[serde(rename = "OWNERS")]
    owners: Vec<u64>,
}

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;

#[allow(unused)]
type AppContext<'a> = poise::ApplicationContext<'a, Data, Error>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: String =
        std::fs::read_to_string("config.json").expect("Failed to read configuration file.");
    let config: Config =
        serde_json::from_str(&config).expect("Failed to parse configuration file.");

    let framework: poise::Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::register(),
            owners: config.owners.iter().map(|&id| UserId::new(id)).collect(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("..".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {} ({})", ready.user.tag(), ready.user.id);

                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::new(config.guild_id),
                )
                .await?;

                Ok(Data {})
            })
        })
        .build();

    let mut client = ClientBuilder::new(config.token, GatewayIntents::non_privileged())
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
