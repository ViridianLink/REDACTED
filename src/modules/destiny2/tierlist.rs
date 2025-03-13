use std::collections::{HashMap, HashSet};
use std::fs;

use async_trait::async_trait;
use serenity::all::{
    AutocompleteChoice, AutocompleteOption, CommandInteraction, CommandOptionType, Context,
    CreateAutocompleteResponse, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, EditInteractionResponse, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{Autocomplete, SlashCommand, parse_options};

use crate::modules::destiny2::endgame_analysis::EndgameAnalysisSheet;
use crate::modules::destiny2::endgame_analysis::weapon::Weapon;
use crate::{Error, Result};

use super::endgame_analysis::tier::TIERS;

pub struct TierList;

#[async_trait]
impl SlashCommand<Error, Postgres> for TierList {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let options = interaction.data.options();
        let options = parse_options(options);

        let weapon_type = match options.get("type") {
            Some(ResolvedValue::String(weapon_type)) => *weapon_type,
            _ => unreachable!("Type is required"),
        };

        let count = options.get("count").map(|c| match c {
            ResolvedValue::Integer(c) => *c as usize,
            _ => unreachable!("Count must be an integer"),
        });

        let tiers = match options.get("tier") {
            Some(ResolvedValue::String(tier)) => {
                let tier = tier.parse().unwrap();
                let index = TIERS.iter().copied().position(|t| t == tier).unwrap();
                &TIERS[..=index]
            }
            _ => &TIERS,
        };

        let weapons: Vec<Weapon> = if let Ok(w) = fs::read_to_string("weapons.json") {
            serde_json::from_str(&w).unwrap()
        } else {
            EndgameAnalysisSheet::update(pool).await?;
            let w = fs::read_to_string("weapons.json").unwrap();
            serde_json::from_str(&w).unwrap()
        };

        let init_map = tiers
            .iter()
            .copied()
            .map(|t| (t, Vec::new()))
            .collect::<HashMap<_, _>>();

        let weapons = weapons
            .into_iter()
            .filter(|w| w.item_type() == weapon_type)
            .filter(|w| tiers.contains(&w.tier))
            .take(count.unwrap_or(usize::MAX))
            .fold(init_map, |mut map, w| {
                map.get_mut(&w.tier).unwrap().push(w.name);
                map
            });

        let embed = CreateEmbed::new()
            .title(format!("Tier List for {}", weapon_type))
            .footer(CreateEmbedFooter::new("From 'Destiny 2: Endgame Analysis'"))
            .fields(TIERS.iter().filter_map(|t| {
                let weapons = weapons.get(t)?;

                if weapons.is_empty() {
                    return None;
                }

                let weapons = weapons
                    .iter()
                    .enumerate()
                    .map(|(i, w)| format!("{}. {}", i + 1, w))
                    .collect::<Vec<_>>();

                Some((t.to_string(), weapons.join("\n"), false))
            }));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let tier_option = TIERS.iter().fold(
            CreateCommandOption::new(
                CommandOptionType::String,
                "tier",
                "The tier to display up to",
            ),
            |option, tier| option.add_string_choice(tier.to_string(), tier.to_string()),
        );

        let command = CreateCommand::new("tierlist")
            .description("Get a tier list of weapons from Destiny 2")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "type",
                    "The type of weapon to display",
                )
                .required(true)
                .set_autocomplete(true),
            )
            .add_option(tier_option)
            .add_option(CreateCommandOption::new(
                CommandOptionType::Integer,
                "count",
                "The number of weapons to display",
            ));

        Ok(command)
    }
}

#[async_trait]
impl Autocomplete<Error, Postgres> for TierList {
    async fn autocomplete(
        ctx: &Context,
        interaction: &CommandInteraction,
        option: AutocompleteOption<'_>,
        pool: &PgPool,
    ) -> Result<()> {
        let weapons: Vec<Weapon> = match std::fs::read_to_string("weapons.json") {
            Ok(weapons) => serde_json::from_str(&weapons).unwrap(),
            Err(_) => {
                EndgameAnalysisSheet::update(pool).await?;
                let weapons = std::fs::read_to_string("weapons.json").unwrap();
                serde_json::from_str(&weapons).unwrap()
            }
        };

        let choices = match option.name {
            "type" => weapons
                .iter()
                .map(|w| w.item_type())
                .collect::<HashSet<_>>()
                .into_iter()
                .filter(|t| t.to_lowercase().contains(&option.value.to_lowercase()))
                .map(AutocompleteChoice::from)
                .collect(),
            // "tier" => {
            //     tiers = TIERS
            //         .iter()
            //         .map(|t| AutocompleteChoice::from(t.to_string()))
            //         .collect::<Vec<_>>();
            // }
            _ => Vec::new(),
        };

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Autocomplete(
                    CreateAutocompleteResponse::new().set_choices(choices),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }
}
