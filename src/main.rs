use poise::{framework, serenity_prelude::{self as serenity, ChannelId}};
use kalosm::language::{Llama, LlamaSource, ModelExt, StreamExt};

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
    // let roles: Vec<_> = member.roles(ctx.cache()).unwrap().iter().map(|x| x.name.clone()).collect();
    // let response = format!("{}", roles.join(", "));
    let has_createe = member.roles(ctx).unwrap().iter().map(|x| &x.name).any(|s| s.contains("Createe"));
    let response = format!("{}", if has_createe {"Createe"} else {"Not a createe"});
    ctx.say(response).await?;
    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.channel_id.get() == 1133703172922286133 {
                if new_message.content.starts_with("bottomuck, ") {
                    let model = Llama::builder().with_source(LlamaSource::tiny_llama_1_1b_chat()).build().unwrap();
                    let prompt = "<|system|>\nYou are a friendly chatbot. Your job is to respond to questions and instructions seriously, but with a funny twist.</s>\n<|user|>\n".to_owned() + &new_message.clone().content.split_off(11) + "</s>\n<|assistant|>\n";
                    let mut result = model.stream_text(&prompt).with_max_length(300).await.unwrap();
                    let mut message = new_message.reply(ctx, "‎ ").await.unwrap();

                    while let Some(token) = result.next().await {
                        message.edit(ctx, serenity::EditMessage::new().content(format!("{}{}", message.content, token))).await.unwrap();
                    }
                }
            }
        }
        serenity::FullEvent::GuildMemberUpdate { old_if_available, new, event } => {
            if event.guild_id.get() == 1047345236302647397 {
                let had_createe = old_if_available.as_ref().unwrap().roles(ctx).unwrap().iter().map(|x| &x.name).any(|s| s.contains("Createe"));
                let has_createe = new.as_ref().unwrap().roles(ctx).unwrap().iter().map(|x| &x.name).any(|s| s.contains("Createe"));
                if !had_createe && has_createe {
                    event.guild_id.channels(ctx).await.unwrap().get(&ChannelId::new(1133703172922286133)).unwrap().say(ctx, format!("User {} just got accepted; This is a test message", new.as_ref().unwrap().display_name())).await.unwrap();
                }
            }
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![roles()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
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