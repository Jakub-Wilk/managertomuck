use poise::serenity_prelude::{self as serenity};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(context_menu_command = "Roles", guild_only)]
async fn roles(
    ctx: Context<'_>,
    #[description = "User who's roles we get"] user: serenity::User,
) -> Result<(), Error> {
    let member = ctx.partial_guild().await.unwrap().member(ctx, user.id).await.unwrap();
    let roles: Vec<_> = member.roles(ctx.cache()).unwrap().iter().map(|x| x.name.clone()).collect();
    let response = format!("{}", roles.join(", "));
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![roles()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}