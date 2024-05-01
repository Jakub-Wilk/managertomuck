mod engsim;

use std::{fmt::format, sync::Arc, io, io::prelude::*};
use regex::Regex;
use engsim::SimAnalyzer;

use poise::{framework, serenity_prelude::{self as serenity, ChannelId, Embed, GetMessages, Mentionable, Typing}, CreateReply};
// use kalosm::language::{Llama, LlamaSource, ModelExt, StreamExt, TextStream};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(context_menu_command = "IGN Test", guild_only)]
async fn roles(
    ctx: Context<'_>,
    #[description = "message"] message: serenity::Message,
) -> Result<(), Error> {
    let nickname = message.embeds[0].title.as_ref().unwrap().split("'").next().clone().unwrap();
    let channel = message.channel(ctx).await.unwrap().guild().unwrap();
    let messages = channel.messages(ctx, GetMessages::new().limit(100)).await.unwrap();
    let messages_from_fluffy = messages.iter().filter(|&m| m.author.id == 996757931641016371);
    let mff_embeds = messages_from_fluffy.map(|m| m.embeds.clone());
    let mut mff_e_mentioning_user: Vec<Embed> = vec![];
    for embs in mff_embeds {
        if embs.len() == 2 {
            let em = &embs[0];
            if em.title.as_ref().unwrap().starts_with(nickname) {
                mff_e_mentioning_user.push(em.clone());
            }
        }
    }
    let re = Regex::new(r"name\?\*\*\n([\W\w]+)\*\*Q6").unwrap();
    let mut q5answers: Vec<String> = vec![];
    for em in mff_e_mentioning_user {
        let q5answer = &re.captures(em.description.as_ref().unwrap()).unwrap()[1];
        q5answers.push(String::from(q5answer));
    }
    let possible_nickname_message = q5answers.last().unwrap();
    let re2 = Regex::new(r"\w+").unwrap();
    let words = re2.find_iter(&possible_nickname_message).map(|m| m.as_str());
    let analyzer = SimAnalyzer::new();
    let results = words.map(|w| (analyzer.confidence(w.to_owned()), w.to_owned()));
    let nickname_prediciton = results.fold((f64::INFINITY, String::new()), |a, b| if a.0 < b.0 { a } else { b });
    ctx.reply(format!("{} - {}", nickname_prediciton.1, nickname_prediciton.0)).await.unwrap();
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
                let current_roles = member.roles(ctx).unwrap();
                let current_role_names = current_roles.iter().map(|x| x.name.clone());

                let is_to_auto_whitelist = current_role_names.clone().any(|s| s.to_lowercase().contains("auto-whitelist-test"));
                let is_applicant = current_role_names.clone().any(|s| s.to_lowercase().contains("applicant"));

                if is_to_auto_whitelist && !is_applicant {
                    let messages = channel.messages(ctx, GetMessages::new().limit(100)).await.unwrap();
                    let messages_from_fluffy = messages.iter().filter(|&m| m.author.id == 996757931641016371);
                    let mff_embeds = messages_from_fluffy.map(|m| m.embeds.clone());
                    let mut mff_e_mentioning_user = vec![];
                    for embs in mff_embeds {
                        if embs.len() == 2 {
                            let em = &embs[0];
                            if em.title.as_ref().unwrap().starts_with(member.user.name.as_str()) {
                                mff_e_mentioning_user.push(em.clone());
                            }
                        }
                    }
                    let re = Regex::new(r"name\?\*\*\n([\W\w]+)\*\*Q6").unwrap();
                    let mut q5answers: Vec<String> = vec![];
                    for em in mff_e_mentioning_user {
                        let q5answer = &re.captures(em.description.as_ref().unwrap()).unwrap()[1];
                        q5answers.push(String::from(q5answer));
                    }
                    let possible_nickname_message = q5answers.last().unwrap();
                    
                    let nickname: Option<String>;
                    let value: Option<f64>;
                    if possible_nickname_message.contains(" ") || possible_nickname_message.contains("\n") {
                        let re2 = Regex::new(r"\w+").unwrap();
                        let words = re2.find_iter(&possible_nickname_message).map(|m| m.as_str());
                        let analyzer = SimAnalyzer::new();
                        let results = words.map(|w| (analyzer.confidence(w.to_owned()), w.to_owned()));
                        let nickname_prediciton = results.fold((f64::INFINITY, String::new()), |a, b| if a.0 < b.0 { a } else { b });
                        if nickname_prediciton.0 < 0.8 {
                            nickname = Some(nickname_prediciton.1)
                        } else {
                            nickname = None
                        }
                        value = Some(nickname_prediciton.0);
                    } else {
                        nickname = Some(possible_nickname_message.to_owned());
                        value = None
                    }

                    if let Some(nickname) = nickname {
                        channel.say(ctx, format!("Test message: User {} just got accepted, detected nickname: {}", member.mention(), nickname)).await.unwrap();
                    } else {
                        channel.say(ctx, format!("Test message: User {} just got accepted, nickname detection failed. Message: `{}`, Value: {}", member.mention(), possible_nickname_message, value.unwrap())).await.unwrap();
                    }

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