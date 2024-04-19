use std::sync::Arc;

use poise::{framework, serenity_prelude::{self as serenity, ChannelId, Mentionable, Typing}};
// use kalosm::language::{Llama, LlamaSource, ModelExt, StreamExt, TextStream};

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
        // serenity::FullEvent::Message { new_message } => {
        //     if new_message.channel_id.get() == 1133703172922286133 {
        //         if new_message.content.starts_with("bottomuck, ") {
        //             let typing = Typing::start(ctx.http.clone(), ChannelId::new(1133703172922286133));
        //             let model = Llama::builder().with_source(LlamaSource::mistral_7b_instruct_2()).build().unwrap();
        //             let prompt = "<s>[INST]You are a friendly chatbot. Your job is to respond to questions and instructions seriously, but with a funny twist. Be sassy. If you think the user's prompt deserves a joke response, respond in a sarcastic manner. Try to keep your responses within around 50 words. The next instruction is your prompt.[/INST]\n[INST]".to_owned() + &new_message.clone().content.split_off(11) + "[/INST]\n";
        //             let mut stream = model.generate_text(&prompt).with_max_length(300).await.unwrap();
        //             let mut message = new_message.reply(ctx, stream).await.unwrap();
        //             typing.stop();
        //         }
        //     }
        // }
        serenity::FullEvent::GuildMemberUpdate { old_if_available, new, event } => {
            if event.guild_id.get() == 1047345236302647397 {
                let channel_map = event.guild_id.channels(ctx).await.unwrap();
                let channel = channel_map.get(&ChannelId::new(1133703172922286133)).unwrap();
                let member = new.as_ref().unwrap();
                let had_createe = match old_if_available.as_ref() {
                    Some(old) => old.roles(ctx).unwrap().iter().map(|x| &x.name).any(|s| s.contains("auto-whitelist-test")),
                    None => {
                        channel.say(ctx, format!("Test message: {} just had their profile changed, but only the new profile is available", member.mention())).await.unwrap();
                        return Ok(())
                    }
                };
                let current_roles = member.roles(ctx).unwrap();
                let has_createe = current_roles.iter().map(|x| &x.name).any(|s| s.contains("auto-whitelist-test"));
                if !had_createe && has_createe {
                    channel.say(ctx, format!("Test message: User {} just got accepted", member.mention())).await.unwrap();
                    member.remove_role(ctx, current_roles.iter().find(|r| r.name.contains("auto-whitelist-test")).unwrap()).await.unwrap();
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