mod new;
mod restore;

use rowifi_framework::prelude::*;

pub use new::*;
pub use restore::*;

pub fn backup_config(cmds: &mut Vec<Command>) {
    let backup_new_cmd = Command::builder()
        .level(RoLevel::Admin)
        .names(&["new"])
        .description("Command to create a new backup")
        .handler(backup_new);

    let backup_restore_cmd = Command::builder()
        .level(RoLevel::Admin)
        .names(&["restore"])
        .description("Command to apply the backup to the server")
        .handler(backup_restore);

    let backup_cmd = Command::builder()
        .level(RoLevel::Admin)
        .names(&["backup"])
        .description("Module to interact with the backup system")
        .group("Premium")
        .sub_command(backup_new_cmd)
        .sub_command(backup_restore_cmd)
        .handler(backup);
    cmds.push(backup_cmd);
}

#[derive(FromArgs)]
pub struct BackupArguments {
    pub name: String,
}

pub async fn backup(ctx: CommandContext) -> CommandResult {
    match ctx.bot.database.get_premium(ctx.author.id.0).await? {
        Some(p) if p.premium_type.has_backup() => {}
        _ => {
            let embed = EmbedBuilder::new()
                .default_data()
                .color(Color::Red as u32)
                .title("Backup Failed")
                .description("This module may only be used by a Beta Tier user")
                .build()
                .unwrap();
            ctx.respond().embed(embed).await?;
            return Ok(());
        }
    };

    let backups = ctx.bot.database.get_backups(ctx.author.id.0).await?;
    let mut embed = EmbedBuilder::new().default_data().title("Backups");

    for backup in backups {
        let val = format!("Prefix: {}\nVerification: {}\nVerified: {}\nRankbinds: {}\nGroupbinds: {}\nCustombinds: {}\nAssetbinds: {}",
            backup.command_prefix.unwrap_or_else(|| "!".into()), backup.verification_role.unwrap_or_default(), backup.verified_role.unwrap_or_default(),
            backup.rankbinds.len(), backup.groupbinds.len(), backup.custombinds.len(), backup.assetbinds.len()
        );
        embed = embed.field(EmbedFieldBuilder::new(backup.name, val));
    }

    ctx.respond().embed(embed.build().unwrap()).await?;
    Ok(())
}
