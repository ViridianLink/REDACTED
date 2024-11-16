use std::env;

use futures::future;
use futures::{stream, StreamExt};
use google_sheets_api::types::common::Color;
use google_sheets_api::types::sheet::Sheet;
use google_sheets_api::SheetsClientBuilder;
use serenity::all::{CommandInteraction, Context, CreateAttachment, EditInteractionResponse};

use crate::sqlx_lib::PostgresPool;
use crate::Result;

use super::weapon::Weapon;

const ENDGAME_ANALYSIS_ID: &str = "1JM-0SlxVDAi-C6rGVlLxa-J1WGewEeL8Qvq4htWZHhY";

fn special_colour(color: &Color) -> bool {
    color.red == 0.0 && color.green == 1.0 && color.blue == 0.0
}

fn heavy_colour(color: &Color) -> bool {
    color.red == 0.6 && color.green == 0.0 && color.blue == 1.0
}

pub struct Wishlist;

impl Wishlist {
    pub async fn update() -> Result<()> {
        let api_key = env::var("GOOGLE_API_KEY")?;

        let client = SheetsClientBuilder::new(api_key).build().unwrap();

        let spreadsheet = client.spreadsheet(ENDGAME_ANALYSIS_ID, true).await.unwrap();

        let (special_heavy, others): (Vec<_>, Vec<_>) = spreadsheet
            .sheets
            .into_iter()
            .filter(|s| !s.properties.hidden)
            .partition(|s| {
                special_colour(&s.properties.tab_color) || heavy_colour(&s.properties.tab_color)
            });

        let primary = others
            .into_iter()
            .find(|s| s.properties.title == "Primaries")
            .unwrap();

        let weapons: Vec<Weapon> = Self::update_primaries(primary)?
            .into_iter()
            .chain(Self::update_special_heavy(special_heavy)?)
            .collect();

        let json = serde_json::to_string(&weapons).unwrap();
        std::fs::write("weapons.json", json)?;

        Ok(())
    }

    pub fn update_special_heavy(sheets: Vec<Sheet>) -> Result<Vec<Weapon>> {
        let weapons = sheets
            .into_iter()
            .map(|mut sheet| sheet.data.pop().unwrap())
            .flat_map(|data| {
                let mut iter = data.row_data.into_iter().skip(1);

                let (name_index, perk_index) =
                    iter.next().unwrap().values.into_iter().enumerate().fold(
                        (None, None),
                        |(name_i, perk_i), (i, cell)| match cell.formatted_value.as_deref() {
                            Some("Name") => (Some(i), perk_i.or(Some(i))),
                            Some("Column 1") => (name_i.or(Some(i)), Some(i)),
                            _ => (name_i, perk_i),
                        },
                    );

                iter.map(move |mut row| {
                    let tier = row.values.pop().unwrap().formatted_value.unwrap();
                    let perks_1 = row
                        .values
                        .remove(perk_index.unwrap())
                        .formatted_value
                        .unwrap()
                        .split('\n')
                        .map(String::from)
                        .collect::<Vec<_>>();
                    let perks_2 = row
                        .values
                        .remove(perk_index.unwrap())
                        .formatted_value
                        .unwrap()
                        .split('\n')
                        .map(String::from)
                        .collect::<Vec<_>>();
                    let origin_trait = row
                        .values
                        .remove(perk_index.unwrap())
                        .formatted_value
                        .unwrap();
                    let name = row
                        .values
                        .remove(name_index.unwrap())
                        .formatted_value
                        .unwrap();

                    Weapon::new(name, vec![perks_1, perks_2], origin_trait, tier)
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(weapons)
    }

    fn update_primaries(mut sheet: Sheet) -> Result<Vec<Weapon>> {
        let data = sheet.data.pop().unwrap();

        let mut iter = data.row_data.into_iter().skip(1);

        let (name_index, perk_index) = iter.next().unwrap().values.into_iter().enumerate().fold(
            (None, None),
            |(name_i, perk_i), (i, cell)| match cell.formatted_value.as_deref() {
                Some("Name") => (Some(i), perk_i),
                Some("Column 1") => (name_i, Some(i)),
                _ => (name_i, perk_i),
            },
        );

        let weapons = iter
            .flat_map(move |mut row| {
                let alternatives = row.values.pop().unwrap().formatted_value.unwrap();

                let perks_1 = row
                    .values
                    .remove(perk_index.unwrap())
                    .formatted_value
                    .unwrap()
                    .split('\n')
                    .map(String::from)
                    .collect::<Vec<_>>();
                let perks_2 = row
                    .values
                    .remove(perk_index.unwrap())
                    .formatted_value
                    .unwrap()
                    .split('\n')
                    .map(String::from)
                    .collect::<Vec<_>>();
                let origin_trait = row
                    .values
                    .remove(perk_index.unwrap())
                    .formatted_value
                    .unwrap();
                let name = row
                    .values
                    .remove(name_index.unwrap())
                    .formatted_value
                    .unwrap();

                let main_weapon = Weapon::new(name, vec![perks_1, perks_2], origin_trait, "S");

                let (name, perks) = match alternatives.split_once('\n') {
                    Some((n, p)) => (n, p),
                    None => return vec![main_weapon],
                };
                let (perk_1, perk_2) = match perks.split_once('+') {
                    Some((p1, p2)) => (p1, p2),
                    None => perks.split_once('/').unwrap(),
                };

                let alt_weapon = Weapon::new(
                    name,
                    vec![
                        perk_1.split("/").map(String::from).collect(),
                        perk_2.split("/").map(String::from).collect(),
                    ],
                    "",
                    "B",
                );

                vec![main_weapon, alt_weapon]
            })
            .collect::<Vec<_>>();

        Ok(weapons)
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
                std::fs::read_to_string("weapons.json")?
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
            .await?;

        Ok(())
    }
}
