use async_trait::async_trait;
use futures::stream::TryStreamExt;
use futures::{stream, StreamExt};
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, CreateMessage,
    EditInteractionResponse, Permissions,
};
use zayden_core::SlashCommand;

use crate::sqlx_lib::PostgresPool;
use crate::{Error, Result};

use super::weapons::*;

const ENDGAME_SHOPPING_LIST: [Weapon; 44] = [
    VS_VELOCITY_BATON,
    TINASHAS_MASTERY,
    ABERRANT_ACTION,
    VS_CHILL_INHIBITOR,
    SUNSHOT,
    LEVIATHANS_BREATH,
    WHISPER_OF_THE_WORM,
    TRACTOR_CANNON,
    NO_HESITATION,
    ERGO_SUM,
    PERFECT_PARADOX,
    VELEDA_F,
    CRITICAL_ANOMALY,
    THE_SUPREMACY,
    LOST_SIGNAL,
    PRO_MEMORIA,
    THE_SLAMMER,
    HELIOCENTRIC_QSC,
    PARASITE,
    GJALLARHORN,
    OUTBREAK_PERFECTED,
    IKELOS_SG_V1_0_3,
    HERITAGE,
    OMNISCIENT_EYE,
    SCATTER_SIGNAL,
    ZEALOTS_REWARD,
    THE_MOUNTAINTOP,
    MARTYRS_RETRIBUTION,
    TUST_OF_THE_BOAR,
    LITURGY,
    WILDERFLIGHT,
    RAKE_ANGLE,
    CHRONOPHAGE,
    SUMMUM_BONUM,
    TOMORROWS_ANSWER_1,
    TOMORROWS_ANSWER_2,
    SCINTILLATION,
    MULTIMACH_CCX,
    WARDENS_LAW,
    YESTERDAYS_QUESTION,
    KHVOSTOV_7G_0X,
    THE_FOURTH_HORSEMAN,
    BURIED_BLOODLINE,
    EUPHONY,
];

pub struct ShoppingList;

#[async_trait]
impl SlashCommand<Error> for ShoppingList {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let embeds: Vec<CreateEmbed> = ENDGAME_SHOPPING_LIST
            .iter()
            .map(CreateEmbed::from)
            .collect();

        for embed in embeds {
            interaction
                .channel_id
                .send_message(ctx, CreateMessage::new().embed(embed))
                .await?;
        }

        let data = ctx.data.read().await;
        let pool = data.get::<PostgresPool>().unwrap();

        let wishlist_output = stream::iter(ENDGAME_SHOPPING_LIST)
            .then(|weapon| async move {
                let name = weapon.name.to_string();

                let (item, adept_item) = weapon.as_api(pool).await.unwrap();

                let mut s = format!("// {}\n//notes: tags:pve", name);

                let mut s_perks = String::new();
                for perk_1 in item.perk_1 {
                    for perk_2 in item.perk_2.iter() {
                        s_perks.push_str(&format!(
                            "\ndimwishlist:item={}&perks={},{}",
                            item.hash, perk_1, perk_2
                        ));
                    }
                }

                s.push_str(&s_perks);

                if adept_item.is_some() {
                    let mut adept_s = format!("\n// {} (Adept)\n//notes: tags:pve", name);
                    adept_s.push_str(&s_perks);

                    s.push_str(&adept_s);
                }

                Result::Ok(s)
            })
            .try_collect::<Vec<_>>()
            .await?;

        let wishlist = format!("title: DIM Wishlist\n\n{}", wishlist_output.join("\n\n"));

        let files = vec![CreateAttachment::bytes(
            wishlist.as_bytes(),
            "DIM Wishlist.txt",
        )];

        interaction
            .channel_id
            .send_files(ctx, files, CreateMessage::new().content("DIM Wishlist:"))
            .await?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("Shopping list sent!"),
            )
            .await?;

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("shoppinglist")
            .description("Get Oscar's Destiny 2 shopping list")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    }
}
