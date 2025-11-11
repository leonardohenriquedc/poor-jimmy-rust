use crate::utils::response::respond_to_followup;
use serenity::{
    all::CommandInteraction,
    builder::{CreateCommand, CreateEmbed},
    client::Context,
    model::Color,
};
use tracing::{error, info};

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    info!("Received damnit-jimmy command from {}", command.user.name);

    // Defer the response since this might take a while
    if let Err(err) = command.defer(&ctx.http).await {
        error!("Failed to defer damnit-jimmy command: {}", err);
        return;
    }

    // Execute the update command
    let output = match tokio::process::Command::new("sh")
        .args(&["-c", "apk update yt-dlp && apk upgrade yt-dlp"])
        .output()
        .await
    {
        Ok(output) => output,
        Err(err) => {
            error!("Failed to execute yt-dlp update command: {}", err);

            let error_embed = CreateEmbed::new()
                .description("Failed to update Jimmy's dependencies!")
                .color(Color::DARK_RED);

            respond_to_followup(command, &ctx.http, error_embed, false).await;
            return;
        }
    };

    // Log the output for debugging
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    info!("yt-dlp update stdout: {}", stdout);
    if !stderr.is_empty() {
        info!("yt-dlp update stderr: {}", stderr);
    }

    // Send the response based on success or failure
    let result_embed = if output.status.success() {
        CreateEmbed::new()
            .description("Successfully updated Jimmy's dependencies!")
            .color(Color::DARK_GREEN)
    } else {
        error!(
            "yt-dlp update failed with exit code: {:?}",
            output.status.code()
        );

        CreateEmbed::new()
            .description("Failed to update Jimmy's dependencies!")
            .color(Color::DARK_RED)
    };

    respond_to_followup(command, &ctx.http, result_embed, false).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("damnit-jimmy")
        .description("Updates Jimmy's dependencies. Use cautiously! Only use this when experiencing playback issues!")
}
