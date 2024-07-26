use std::{fmt::format, sync::Arc, io, io::prelude::*};
use regex::Regex;

use poise::{
    execute_modal_on_component_interaction, framework, serenity_prelude::{
        self as serenity, model::channel, ChannelId, CreateEmbed, Embed, GetMessages, GuildId, Interaction, InteractionId, Mentionable, Typing
    }, CreateReply, Modal
};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, guild_only)]
async fn test(ctx: Context<'_>) -> Result<(), Error> {
    if ctx.author_member().await.unwrap().permissions(ctx)?.manage_guild() {
        let channel = ctx.guild_channel().await.unwrap();
        let message = {
            let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("apply")
                    .style(serenity::ButtonStyle::Success)
                    .label("Apply!")
            ])];

            CreateReply::default()
                .content("Application test")
                .components(buttons)
        };

        ctx.send(message).await?;
    }
    
    Ok(())
}

struct AsRefWrapper<'a, T>(&'a T);

impl<'a, T> AsRef<T> for AsRefWrapper<'a, T> {
    fn as_ref(&self) -> &T {
        self.0
    }
}

#[derive(Modal)]
#[name = "Crafttomuck Application"]
struct ApplicationModal {
    #[name = "Is this an answer?"]
    #[placeholder = "This is a text input"]
    #[min_length = 10]
    q1: String,
    #[name = "Is this a longer answer?"]
    #[placeholder = "This is a text input"]
    #[paragraph]
    #[min_length = 50]
    q2: String

}

#[derive(Modal)]
#[name = "Deny"]
struct DenyModal {
    #[name = "Reason for denial (optional)"]
    #[placeholder = "Reason..."]
    reason: Option<String>
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
        serenity::FullEvent::InteractionCreate {interaction} => {
            if let Interaction::Component(interaction) = interaction {

                let member = interaction.member.as_ref().unwrap();

                if interaction.data.custom_id == "apply" {

                    let ApplicationModal {q1, q2} = execute_modal_on_component_interaction::<ApplicationModal>(AsRefWrapper(ctx), interaction.clone(), None, None).await?.unwrap();

                    let message = {
                        let author = serenity::CreateEmbedAuthor::new(member.display_name().to_string())
                            .icon_url(member.user.avatar_url().unwrap());

                        let embed = serenity::CreateEmbed::new()
                            .author(author)
                            .fields(vec![
                                ("Question 1: Is this an answer?", q1, false),
                                ("Question 2: Is this a longer answer?", q2, false),
                                ("", "\u{2800}".to_string(), false)
                            ])
                            .fields(vec![
                                ("Submitted by", member.mention().to_string(), true),
                                ("Status", "Pending".to_string(), true),
                                ("", String::new(), true)
                            ])
                            .color((241, 196, 15));

                        serenity::CreateMessage::new()
                            .embed(embed)
                            .components(vec![serenity::CreateActionRow::Buttons(vec![
                                serenity::CreateButton::new("approve")
                                    .style(serenity::ButtonStyle::Success)
                                    .label("Approve"),
                                serenity::CreateButton::new("deny")
                                    .style(serenity::ButtonStyle::Danger)
                                    .label("Deny"),
                                serenity::CreateButton::new("age")
                                    .style(serenity::ButtonStyle::Danger)
                                    .label("Deny (18+)"),
                                serenity::CreateButton::new("secret")
                                    .style(serenity::ButtonStyle::Danger)
                                    .label("Deny (secret word)"),
                                serenity::CreateButton::new("effort")
                                    .style(serenity::ButtonStyle::Danger)
                                    .label("Deny (low effort)"),
                            ])])
                    };

                    interaction.channel_id.send_message(ctx, message).await?;

                } else if ["approve", "deny", "age", "secret", "effort"].contains(&interaction.data.custom_id.as_str()) {
                    if interaction.data.custom_id == "approve" {
                        let message = &interaction.message;
                        let mut embed = message.embeds[0].clone();
                        let username = &embed.fields[3].value;
                        let approved_by = interaction.member.as_ref().unwrap();

                        embed.colour = Some(serenity::Colour::from_rgb(46, 204, 113));
                        embed.fields[4].value = "Approved".to_string();
                        embed.fields[5].name = "Approved by:".to_string();
                        embed.fields[5].value = approved_by.mention().to_string();

                        let edit_message = {
                            serenity::EditMessage::new()
                                .embed(CreateEmbed::from(embed))
                                .components(vec![serenity::CreateActionRow::Buttons(vec![
                                    serenity::CreateButton::new("disabled")
                                        .style(serenity::ButtonStyle::Success)
                                        .label("Approved!")
                                        .disabled(true)
                                ])])
                        };

                        interaction.message.clone().edit(ctx, edit_message).await?;
                        interaction.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                    } else {
                        let reason = match interaction.data.custom_id.as_str() {
                            "age" => {
                                "Sorry, this server is 18+!".to_string()
                            }
                            "secret" => {
                                "Sorry, you have the wrong secret word! Read the rules carefully, and apply again!".to_string()
                            }
                            "effort" => {
                                "Sorry, we are only accepting high effort applications, if you wish to try again please do so!".to_string()
                            }
                            _ => {
                                let DenyModal { reason } = execute_modal_on_component_interaction(AsRefWrapper(ctx), interaction.clone(), None, None).await?.unwrap();
                                match reason {
                                    Some(reason) => reason,
                                    None => "Sorry, but your application didn't meet our expectations!".to_string()
                                }
                            }
                        };

                        let message = &interaction.message;
                        let mut embed = message.embeds[0].clone();
                        let username = &embed.fields[3].value;
                        let denied_by = interaction.member.as_ref().unwrap();

                        embed.colour = Some(serenity::Colour::from_rgb(231, 76, 60));
                        embed.fields[4].value = "Denied".to_string();
                        embed.fields[5].name = "Denied by:".to_string();
                        embed.fields[5].value = denied_by.mention().to_string();

                        let edit_message = {
                            serenity::EditMessage::new()
                                .embed(
                                    CreateEmbed::from(embed)
                                        .field("Reason:", reason.to_string(), false)
                                )
                                .components(vec![serenity::CreateActionRow::Buttons(vec![
                                    serenity::CreateButton::new("disabled")
                                        .style(serenity::ButtonStyle::Danger)
                                        .label("Denied")
                                        .disabled(true)
                                ])])
                        };

                        interaction.message.clone().edit(ctx, edit_message).await?;
                        interaction.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                    }
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
            commands: vec![test()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, GuildId::new(1047345236302647397)).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}