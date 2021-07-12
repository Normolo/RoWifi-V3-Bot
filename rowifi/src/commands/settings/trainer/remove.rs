use mongodb::bson::doc;
use rowifi_framework::prelude::*;
use rowifi_models::guild::GuildType;

#[derive(FromArgs)]
pub struct TrainerRemoveArguments {
    #[arg(rest, help = "List of all roles to be removed from `RoWifi Trainer`")]
    pub roles: String,
}

pub async fn trainer_remove(ctx: CommandContext, args: TrainerRemoveArguments) -> CommandResult {
    let guild_id = ctx.guild_id.unwrap();
    let guild = ctx.bot.database.get_guild(guild_id.0).await?;

    if guild.settings.guild_type == GuildType::Normal {
        let embed = EmbedBuilder::new()
            .default_data()
            .color(Color::Red as u32)
            .title("Command Failed")
            .description("This command is only available on Premium servers")
            .build()
            .unwrap();
        ctx.respond().embed(embed).await?;
        return Ok(());
    }

    let mut role_ids = Vec::new();
    for r in args.roles.split_ascii_whitespace() {
        if let Some(r) = parse_role(r) {
            role_ids.push(r);
        }
    }

    let filter = doc! {"_id": guild.id};
    let update = doc! {"$pullAll": {"Settings.TrainerRoles": role_ids.clone()}};
    ctx.bot.database.modify_guild(filter, update).await?;

    ctx.bot
        .trainer_roles
        .entry(guild_id)
        .or_default()
        .retain(|r| role_ids.contains(&r.0));

    let mut description = "Removed Trainer Roles:\n".to_string();
    for role in role_ids {
        description.push_str(&format!("- <@&{}>\n", role));
    }

    let embed = EmbedBuilder::new()
        .default_data()
        .color(Color::DarkGreen as u32)
        .title("Settings Modification Successful")
        .description(description)
        .build()
        .unwrap();
    ctx.respond().embed(embed).await?;

    Ok(())
}
