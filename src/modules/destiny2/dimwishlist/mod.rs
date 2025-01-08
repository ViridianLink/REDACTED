use async_trait::async_trait;
use serenity::all::{
    AutocompleteChoice, CommandInteraction, CommandOptionType, Context, CreateAutocompleteResponse,
    CreateCommand, CreateCommandOption, CreateInteractionResponse, EditInteractionResponse, Ready,
    ResolvedOption, ResolvedValue,
};
use weapon::Weapon;
use wishlist::Wishlist;
use zayden_core::{parse_options, Autocomplete, SlashCommand};

pub mod bungie_api;
pub mod weapon;
mod wishlist;

use crate::{Error, Result};

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![
        DimWishlist::register(ctx, ready)?,
        D2Weapon::register(ctx, ready)?,
    ];

    Ok(commands)
}

pub struct DimWishlist;

#[async_trait]
impl SlashCommand<Error> for DimWishlist {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let options = interaction.data.options();
        let options = parse_options(&options);
        let strict = match options.get("strict") {
            Some(ResolvedValue::String(strict)) => strict,
            _ => "soft",
        };

        Wishlist::run(ctx, interaction, strict).await?;

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

pub struct D2Weapon;

#[async_trait]
impl SlashCommand<Error> for D2Weapon {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let options = interaction.data.options();
        let options = parse_options(&options);

        let name = match options.get("name") {
            Some(ResolvedValue::String(name)) => name,
            _ => unreachable!("Name is required"),
        };

        let weapons = match std::fs::read_to_string("weapons.json") {
            Ok(weapons) => weapons,
            Err(_) => {
                Wishlist::update().await?;
                std::fs::read_to_string("weapons.json").unwrap()
            }
        };
        let weapons: Vec<Weapon> = serde_json::from_str(&weapons).unwrap();

        let weapon = weapons
            .iter()
            .find(|w| w.name.to_lowercase() == name.to_lowercase())
            .unwrap();

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(weapon.into()))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("d2weapon")
            .description("Get a weapon from Destiny 2")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "name",
                    "The name of the weapon",
                )
                .required(true)
                .set_autocomplete(true),
            );

        Ok(command)
    }
}

#[async_trait]
impl Autocomplete<Error> for D2Weapon {
    async fn autocomplete(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let option = interaction.data.autocomplete().unwrap();

        let weapons = match std::fs::read_to_string("weapons.json") {
            Ok(weapons) => weapons,
            Err(_) => {
                Wishlist::update().await?;
                std::fs::read_to_string("weapons.json").unwrap()
            }
        };
        let weapons: Vec<Weapon> = serde_json::from_str(&weapons).unwrap();
        let weapons = weapons
            .into_iter()
            .filter(|w| w.name.to_lowercase().contains(&option.value.to_lowercase()))
            .map(AutocompleteChoice::from)
            .take(25)
            .collect::<Vec<_>>();

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Autocomplete(
                    CreateAutocompleteResponse::new().set_choices(weapons),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }
}
