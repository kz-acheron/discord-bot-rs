use poise::serenity_prelude::*;

use crate::{Context, Error};

/// Use basic command and register the command to be used with prefix & slash.
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;

    Ok(())
}

/// Modal Structure (To take user input data).
#[derive(Debug, poise::Modal)]
struct Modal {
    nama: String,
}

/// Slash command with Button component and show modal when the button is interacted/clicked.
#[poise::command(slash_command)]
pub async fn button(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![CreateActionRow::Buttons(vec![CreateButton::new(
            "button_id",
        )
        .label("Open Modal")
        .style(ButtonStyle::Primary)])];

        poise::CreateReply::default().components(components)
    };

    ctx.send(reply).await?;

    while let Some(ci) = ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(30))
        .filter(move |ci| ci.data.custom_id == "button_id")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<Modal>(ctx, ci, None, None).await?;
        println!("{:?}", data);
    }

    Ok(())
}

/// Slash command with Select Menu component.
#[poise::command(slash_command)]
pub async fn select_menu(ctx: Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![CreateActionRow::SelectMenu(CreateSelectMenu::new(
            "select_menu_id",
            CreateSelectMenuKind::String {
                options: vec![
                    CreateSelectMenuOption::new("Menu 1", "menu1_value"),
                    CreateSelectMenuOption::new("Menu 2", "menu2_value"),
                    CreateSelectMenuOption::new("Menu 3", "menu3_value"),
                ],
            },
        ))];

        poise::CreateReply::default().components(components)
    };

    ctx.send(reply).await?;

    while let Some(ci) = ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(30))
        .await
    {
        match ci.data.custom_id.as_str() {
            "select_menu_id" => {
                if let ComponentInteractionDataKind::StringSelect { values } = &ci.data.kind {
                    if let Some(value) = values.first() {
                        ci.create_response(
                            ctx,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content(format!("{}", value))
                                    .ephemeral(true),
                            ),
                        )
                        .await?;
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}

/// Slash and Context Menu (Right Click User) command to get user information.
#[poise::command(context_menu_command = "User Information", slash_command)]
pub async fn user_information(
    ctx: Context<'_>,
    #[description = "Check User Information"] user: User,
) -> Result<(), Error> {
    let res = format!(
        "**Name**: {}\n**Created At**: {}",
        user.name,
        user.created_at()
    );

    ctx.say(res).await?;

    Ok(())
}
