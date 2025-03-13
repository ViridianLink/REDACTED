use std::fs;

use async_trait::async_trait;
use serenity::all::{
    AutocompleteChoice, AutocompleteOption, CommandInteraction, CommandOptionType, Context,
    CreateAutocompleteResponse, CreateCommand, CreateCommandOption, CreateInteractionResponse,
    EditInteractionResponse, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{Autocomplete, SlashCommand, parse_options};

use crate::{Error, Result};

use super::endgame_analysis::EndgameAnalysisSheet;
use super::endgame_analysis::weapon::Weapon;

pub struct WeaponCommand;

#[async_trait]
impl SlashCommand<Error, Postgres> for WeaponCommand {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let options = interaction.data.options();
        let options = parse_options(options);

        let name = match options.get("name") {
            Some(ResolvedValue::String(name)) => name,
            _ => unreachable!("Name is required"),
        };

        let weapons: Vec<Weapon> = if let Ok(w) = fs::read_to_string("weapons.json") {
            serde_json::from_str(&w).unwrap()
        } else {
            EndgameAnalysisSheet::update(pool).await?;
            let w = fs::read_to_string("weapons.json").unwrap();
            serde_json::from_str(&w).unwrap()
        };

        let weapon = weapons
            .iter()
            .find(|w| w.name().to_lowercase() == name.to_lowercase())
            .unwrap();

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(weapon.into()))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("weapon")
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
impl Autocomplete<Error, Postgres> for WeaponCommand {
    async fn autocomplete(
        ctx: &Context,
        interaction: &CommandInteraction,
        option: AutocompleteOption<'_>,
        pool: &PgPool,
    ) -> Result<()> {
        let weapons = match std::fs::read_to_string("weapons.json") {
            Ok(weapons) => weapons,
            Err(_) => {
                EndgameAnalysisSheet::update(pool).await?;
                std::fs::read_to_string("weapons.json").unwrap()
            }
        };
        let weapons: Vec<Weapon> = serde_json::from_str(&weapons).unwrap();
        let weapons = weapons
            .into_iter()
            .filter(|w| {
                w.name()
                    .to_lowercase()
                    .contains(&option.value.to_lowercase())
            })
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
