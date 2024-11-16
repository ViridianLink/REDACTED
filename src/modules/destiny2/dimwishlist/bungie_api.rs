// #![allow(dead_code)]
use std::env;

use bungie_api::types::common::DestinyDisplayPropertiesDefinition;
use bungie_api::types::definitions::DestinyItemInventoryBlockDefinition;
use bungie_api::types::destiny::{DamageType, DestinyItemSubType, DestinyItemType};
use bungie_api::types::TierType;
use bungie_api::{
    BungieClientBuilder, DestinyInventoryItemDefinition, DestinyInventoryItemManifest,
    DestinyPlugSetManifest, DestinySocketCategoryManifest, DestinySocketTypeManifest,
};
use sqlx::PgPool;

use crate::Result;

pub struct BungieApi;

impl BungieApi {
    pub async fn update_dbs(pool: &PgPool) -> Result<()> {
        let client = BungieClientBuilder::new(env::var("BUNGIE_API_KEY")?)
            .build()
            .unwrap();

        let manifest = client.destiny_manifest().await.unwrap();
        let item_manifest = client
            .destiny_inventory_item_definition(&manifest, "en")
            .await
            .unwrap();
        let socket_type_manifest = client
            .destiny_socket_type_definition(&manifest, "en")
            .await
            .unwrap();
        let socket_category_manifest = client
            .destiny_socket_category_definition(&manifest, "en")
            .await
            .unwrap();
        let plug_set_manifest = client
            .destiny_plug_set_definition(&manifest, "en")
            .await
            .unwrap();

        BungieApi::update_weapon_db(
            pool,
            &item_manifest,
            &socket_type_manifest,
            &socket_category_manifest,
            &plug_set_manifest,
        )
        .await?;

        BungieApi::update_perk_db(pool, &item_manifest).await?;

        Ok(())
    }

    async fn update_weapon_db(
        pool: &PgPool,
        item_manifest: &DestinyInventoryItemManifest,
        socket_type_manifest: &DestinySocketTypeManifest,
        socket_category_manifest: &DestinySocketCategoryManifest,
        plug_set_manifest: &DestinyPlugSetManifest,
    ) -> Result<()> {
        let mut tx = pool.begin().await.unwrap();

        sqlx::query!("DELETE FROM destiny_weapons")
            .execute(&mut *tx)
            .await
            .unwrap();

        let valid_weapons = item_manifest
            .values()
            .filter(|item| match item {
                DestinyInventoryItemDefinition {
                    default_damage_type: DamageType::None,
                    ..
                } => false,
                DestinyInventoryItemDefinition {
                    item_type: DestinyItemType::Weapon,
                    inventory:
                        DestinyItemInventoryBlockDefinition {
                            tier_type: TierType::Superior | TierType::Exotic,
                            ..
                        },
                    ..
                } => true,
                _ => false,
            })
            .cloned()
            .map(|mut item| {
                if item.secondary_icon
                    == Some(String::from(
                        "/common/destiny2_content/icons/6774c7855193dff237408fc5295f39c2.png",
                    ))
                {
                    item.display_properties.name.push_str(" (Brave)");
                }
                item
            })
            .collect::<Vec<_>>();

        for weapon in valid_weapons {
            let hash = weapon.hash as i64;
            let name = &weapon.display_properties.name;

            let mut perks = weapon
                .sockets
                .as_ref()
                .unwrap()
                .socket_entries
                .iter()
                .filter(|s| {
                    socket_type_manifest
                        .get(&s.socket_type_hash.to_string())
                        .and_then(|socket_type| {
                            socket_category_manifest
                                .get(&socket_type.socket_category_hash.to_string())
                        })
                        .map_or(false, |socket_category| {
                            socket_category.display_properties.name == "WEAPON PERKS"
                        })
                })
                .take(4) // TODO: Handle weapon traits
                .map(|socket| {
                    let perk_hashs = match (
                        socket.randomized_plug_set_hash,
                        socket.reusable_plug_set_hash,
                    ) {
                        (Some(hash), None) | (None, Some(hash)) => {
                            let plug_set = plug_set_manifest.get(&hash.to_string()).unwrap();
                            plug_set
                                .reusable_plug_items
                                .iter()
                                .map(|plug| plug.plug_item_hash)
                                .collect::<Vec<_>>()
                        }
                        (None, None) => socket
                            .reusable_plug_items
                            .iter()
                            .map(|plug| plug.plug_item_hash)
                            .collect::<Vec<_>>(),
                        _ => panic!("Invalid socket on weapon: {}", weapon.hash),
                    };

                    perk_hashs
                        .into_iter()
                        .map(|hash| item_manifest.get(&hash.to_string()).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            if perks.is_empty() {
                continue;
            }

            perks.reverse();

            let column_1 = perks.pop().unwrap();
            let column_2 = perks.pop().unwrap();
            let perk_1 = perks.pop().unwrap();
            let perk_2 = perks.pop().unwrap_or_default();

            sqlx::query!(
                r#"
            INSERT INTO destiny_weapons (id, name, column_1, column_2, perk_1, perk_2)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
                hash,
                name,
                &column_1
                    .into_iter()
                    .map(|p| p.hash as i64)
                    .collect::<Vec<_>>(),
                &column_2
                    .into_iter()
                    .map(|p| p.hash as i64)
                    .collect::<Vec<_>>(),
                &perk_1
                    .into_iter()
                    .map(|p| p.hash as i64)
                    .collect::<Vec<_>>(),
                &perk_2
                    .into_iter()
                    .map(|p| p.hash as i64)
                    .collect::<Vec<_>>(),
            )
            .execute(&mut *tx)
            .await
            .unwrap();
        }

        tx.commit().await.unwrap();

        Ok(())
    }

    async fn update_perk_db(
        pool: &PgPool,
        item_manifest: &DestinyInventoryItemManifest,
    ) -> Result<()> {
        let valid_perks = item_manifest
        .values()
        .filter(|item| match item {
                DestinyInventoryItemDefinition {
                    item_sub_type:
                        // DestinyItemSubType::Mask
                        | DestinyItemSubType::Shader
                        | DestinyItemSubType::Ornament,
                    ..
                } => false,
                DestinyInventoryItemDefinition {
                    display_properties: DestinyDisplayPropertiesDefinition {
                        name,
                        ..
                    },
                    item_type_display_name: Some(item_type_display_name),
                    // "itemTypeDisplayName": "Weapon Mod",
                    item_type: DestinyItemType::Mod,
                    ..
                } => !(name.is_empty() || item_type_display_name.is_empty() || item_type_display_name.starts_with("Ghost") || item_type_display_name.starts_with("Deprecated") || item_type_display_name == "Artifact Perk" || item_type_display_name == "Material" || item_type_display_name.ends_with("Emote") || item_type_display_name.ends_with("Mod") || item_type_display_name.ends_with("Tonic") || item_type_display_name.ends_with("Effect") || item_type_display_name.ends_with("Ability") || item_type_display_name.ends_with("Grenade") || item_type_display_name.ends_with("Aspect") || item_type_display_name.ends_with("Fragment")),
                _ => false,
            })
        .collect::<Vec<_>>();

        let mut tx = pool.begin().await.unwrap();

        for perk in valid_perks {
            let hash = perk.hash as i64;
            let name = &perk.display_properties.name;

            sqlx::query!(
                r#"
            INSERT INTO destiny_perks (id, name)
            VALUES ($1, $2)
            ON CONFLICT (id) DO NOTHING
            "#,
                hash,
                name,
            )
            .execute(&mut *tx)
            .await
            .unwrap();
        }

        tx.commit().await.unwrap();

        Ok(())
    }
}
