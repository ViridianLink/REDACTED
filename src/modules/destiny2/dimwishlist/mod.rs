use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, Ready,
    ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use wishlist::Wishlist;
use zayden_core::SlashCommand;

pub mod bungie_api;
pub mod wishlist;

use crate::{Error, Result};

pub struct DimWishlist;

#[async_trait]
impl SlashCommand<Error, Postgres> for DimWishlist {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let strict = match options.pop().map(|o| o.value) {
            Some(ResolvedValue::String(strict)) => strict,
            _ => "soft",
        };

        Wishlist::run(ctx, interaction, pool, strict).await?;

        Ok(())
    }
    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("dimwishlist")
            .description("Get a wishlist from DIM")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "strict", "Soft: All | Regular: S, A, B, C, D | Semi: S, A, B, C | Strict: S, A, B | Very: S, A | Uber: S")
                    .add_string_choice("Soft", "soft")
                    .add_string_choice("Regular", "regular")
                    .add_string_choice("Semi-strict", "semi-strict")
                    .add_string_choice("Strict", "strict")
                    .add_string_choice("Very Strict", "very strict")
                    .add_string_choice("Uber Strict", "uber strict"),
            );

        Ok(command)
    }
}
