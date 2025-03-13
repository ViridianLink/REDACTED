use futures::future;
use futures::{StreamExt, stream};
use serenity::all::{CommandInteraction, Context, CreateAttachment, EditInteractionResponse};
use sqlx::PgPool;

use crate::Result;
use crate::modules::destiny2::endgame_analysis::EndgameAnalysisSheet;
use crate::modules::destiny2::endgame_analysis::weapon::Weapon;
use crate::sqlx_lib::PostgresPool;

pub struct Wishlist;

impl Wishlist {
    pub async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &PgPool,
        strict: &str,
    ) -> Result<()> {
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
                EndgameAnalysisSheet::update(pool).await?;
                std::fs::read_to_string("weapons.json").unwrap()
            }
        };
        let weapons: Vec<Weapon> = serde_json::from_str(&weapons).unwrap();

        let pool = PostgresPool::get(ctx).await;

        let wishlist = stream::iter(weapons)
            .filter(|weapon| future::ready(tier.contains(&weapon.tier().as_str())))
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
