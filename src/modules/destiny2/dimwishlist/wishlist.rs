use std::env;

use futures::future;
use futures::{stream, StreamExt};
use google_sheets_api::types::common::Color;
use google_sheets_api::types::sheet::GridData;
use google_sheets_api::SheetsClientBuilder;
use serenity::all::{CommandInteraction, Context, CreateAttachment, EditInteractionResponse};

use crate::sqlx_lib::PostgresPool;
use crate::Result;

use super::weapon::Weapon;

const ENDGAME_ANALYSIS_ID: &str = "1JM-0SlxVDAi-C6rGVlLxa-J1WGewEeL8Qvq4htWZHhY";

fn primary_colour(color: &Color) -> bool {
    color.red == 1.0 && color.green == 1.0 && color.blue == 1.0
}

fn special_colour(color: &Color) -> bool {
    color.red == 0.0 && color.green == 1.0 && color.blue == 0.0
}

fn heavy_colour(color: &Color) -> bool {
    color.red == 0.6 && color.green == 0.0 && color.blue == 1.0
}

pub struct Wishlist;

impl Wishlist {
    pub async fn update() -> Result<()> {
        let api_key = env::var("GOOGLE_API_KEY").unwrap();

        let client = SheetsClientBuilder::new(api_key).build().unwrap();

        let spreadsheet = client.spreadsheet(ENDGAME_ANALYSIS_ID, true).await.unwrap();

        let weapons = spreadsheet
            .sheets
            .into_iter()
            .filter(|s| !s.properties.hidden)
            .filter(|s| {
                primary_colour(&s.properties.tab_color)
                    || special_colour(&s.properties.tab_color)
                    || heavy_colour(&s.properties.tab_color)
            })
            .map(|mut sheet| sheet.data.pop().unwrap())
            .flat_map(Self::parse_weapon_data)
            .collect::<Vec<_>>();

        let json = serde_json::to_string(&weapons).unwrap();
        std::fs::write("weapons.json", json).unwrap();

        Ok(())
    }

    pub fn parse_weapon_data(data: GridData) -> Vec<Weapon> {
        let mut iter = data.row_data.into_iter().skip(1);

        let (name_index, perk_index) = iter.next().unwrap().values.into_iter().enumerate().fold(
            (None, None),
            |(name_i, perk_i), (i, cell)| match cell.formatted_value.as_deref() {
                Some("Name") => (Some(i), perk_i.or(Some(i))),
                Some("Column 1") => (name_i.or(Some(i)), Some(i)),
                _ => (name_i, perk_i),
            },
        );

        iter.map(|row| Weapon::from_row_data(row, name_index.unwrap(), perk_index.unwrap()))
            .collect::<Vec<_>>()
    }

    pub async fn run(ctx: &Context, interaction: &CommandInteraction, strict: &str) -> Result<()> {
        let tier = match strict {
            "soft" => vec!["S", "A", "B", "C", "D", "E", "F", "G"],
            "regular" => vec!["S", "A", "B", "C", "D"],
            "semi-strict" => vec!["S", "A", "B", "C"],
            "strict" => vec!["S", "A", "B"],
            "very strict" => vec!["S", "A"],
            "uber strict" => vec!["S"],
            _ => unreachable!(),
        };

        let weapons = match std::fs::read_to_string("weapons.json") {
            Ok(weapons) => weapons,
            Err(_) => {
                Self::update().await?;
                std::fs::read_to_string("weapons.json").unwrap()
            }
        };
        let weapons: Vec<Weapon> = serde_json::from_str(&weapons).unwrap();

        let pool = PostgresPool::get(ctx).await;

        let wishlist = stream::iter(weapons)
            .filter(|weapon| future::ready(tier.contains(&weapon.tier.as_str())))
            .then(|weapon| {
                let pool = pool.clone();
                async move { weapon.as_wishlist(&pool).await }
            })
            .collect::<Vec<_>>()
            .await;

        let wishlist = format!("title: DIM Wishlist\n\n{}", wishlist.join("\n\n"));

        let file = CreateAttachment::bytes(
            wishlist.as_bytes(),
            format!("PVE Wishlist ({}).txt", strict),
        );

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .new_attachment(file)
                    .content(format!("PVE Wishlist ({}):", strict)),
            )
            .await
            .unwrap();

        Ok(())
    }
}
