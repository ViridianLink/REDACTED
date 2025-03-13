pub mod affinity;
pub mod frame;
pub mod tier;
pub mod weapon;

use std::env;

use futures::{StreamExt, stream};
use google_sheets_api::SheetsClientBuilder;
use google_sheets_api::types::common::Color;
use google_sheets_api::types::sheet::GridData;
use sqlx::PgPool;
use weapon::{Weapon, WeaponBuilder};

use crate::Result;

const ENDGAME_ANALYSIS_ID: &str = "1JM-0SlxVDAi-C6rGVlLxa-J1WGewEeL8Qvq4htWZHhY";

fn primary_colour(color: &Color) -> bool {
    color.red == 0.9529412 && color.green == 0.9529412 && color.blue == 0.9529412
}

fn special_colour(color: &Color) -> bool {
    color.red == 0.0 && color.green == 1.0 && color.blue == 0.0
}

fn heavy_colour(color: &Color) -> bool {
    color.red == 0.6 && color.green == 0.0 && color.blue == 1.0
}

pub struct EndgameAnalysisSheet;

impl EndgameAnalysisSheet {
    pub async fn update(pool: &PgPool) -> Result<()> {
        let api_key = env::var("GOOGLE_API_KEY").unwrap();

        let client = SheetsClientBuilder::new(api_key).build().unwrap();

        let spreadsheet = client.spreadsheet(ENDGAME_ANALYSIS_ID, true).await.unwrap();

        let iter = spreadsheet
            .sheets
            .into_iter()
            .filter(|s| !s.properties.hidden)
            .filter(|s| {
                primary_colour(&s.properties.tab_color)
                    || special_colour(&s.properties.tab_color)
                    || heavy_colour(&s.properties.tab_color)
            })
            .map(|mut sheet| (sheet.properties.title, sheet.data.pop().unwrap()));

        let weapons = stream::iter(iter)
            .then(|(name, data)| async { Self::parse_weapon_data(pool, name, data).await })
            .flat_map(stream::iter)
            .collect::<Vec<_>>()
            .await;

        let json = serde_json::to_string(&weapons).unwrap();
        std::fs::write("weapons.json", json).unwrap();

        Ok(())
    }

    async fn parse_weapon_data(
        pool: &PgPool,
        name: impl Into<String>,
        data: GridData,
    ) -> Vec<Weapon> {
        let name = name.into();

        let mut iter = data.row_data.into_iter().skip(1);
        let header = iter.next().unwrap();
        let iter = iter.filter_map(|r| WeaponBuilder::from_row_data(name.clone(), &header, r));

        stream::iter(iter)
            .then(|builder| async { builder.build(pool).await })
            .collect()
            .await
    }
}
