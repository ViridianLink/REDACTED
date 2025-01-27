use std::ops::Deref;

use futures::{stream, StreamExt};
use google_sheets_api::types::sheet::RowData;
use serde::{Deserialize, Serialize};
use serenity::all::{AutocompleteChoice, CreateEmbed};
use sqlx::{FromRow, PgPool};

// const IDEAL_SHOTGUN_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::BarrelShroud,
//     column_2: Column2::TacticalMag,
// };
// const IDEAL_SNIPER_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::FlutedBarrel,
//     column_2: Column2::TacticalMag,
// };
// const IDEAL_FUSION_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::FlutedBarrel,
//     column_2: Column2::AcceleratedCoils,
// };
// const IDEAL_BGL_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::QuickLaunch,
//     column_2: Column2::SpikeGrenades,
// };
// const IDEAL_GLAIVE_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::None,
//     column_2: Column2::None,
// };
// const IDEAL_TRACE_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::Fluted Barrel,
//     column_2: Column2::Light Battery,
// };
// const IDEAL_ROCKET_SIDEARM_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::VolatileLaunch,
//     column_2: Column2::HighExplosiveOrdnance,
// };
// const IDEAL_LMG_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::FlutedBarrel,
//     column_2: Column2::ExtendedMag,
// };
// const IDEAL_HGL_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::QuickLaunch,
//     column_2: Column2::SpikeGrenades,
// };
// const IDEAL_SWORD_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::JaggedEdge,
//     column_2: Column2::SwordmastersGuard,
// };
// const IDEAL_ROCKET_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::QuickLaunch,
//     column_2: Column2::ImpactCasing,
// };
// const IDEAL_LFR_COLUMN: IdealWeaponColumns = IdealWeaponColumns {
//     column_1: Column1::FlutedBarrel,
//     column_2: Column2::AcceleratedCoils,
// };

#[derive(Deserialize, Serialize)]
pub struct Weapon {
    pub name: String,
    pub perks: Perks,
    pub origin_trait: String,
    pub tier: String,
}

impl Weapon {
    pub fn new(
        name: impl Into<String>,
        perks: impl Into<Perks>,
        origin_trait: impl Into<String>,
        tier: impl Into<String>,
    ) -> Self {
        let name = name
            .into()
            .replace("\nBRAVE version", " (Brave)")
            .replace(" (BRAVE version)", " (Brave)");

        Weapon {
            name,
            perks: perks.into(),
            origin_trait: origin_trait.into(),
            tier: tier.into(),
        }
    }

    pub fn from_row_data(mut value: RowData, name_i: usize, perk_i: usize) -> Self {
        let tier = value
            .values
            .pop()
            .unwrap()
            .formatted_value
            .unwrap_or_default();

        let perks_1 = match value.values.remove(perk_i).formatted_value {
            Some(perks) => perks.split('\n').map(String::from).collect::<Vec<_>>(),
            None => Vec::new(),
        };

        let perks_2 = match value.values.remove(perk_i).formatted_value {
            Some(perks) => perks.split('\n').map(String::from).collect::<Vec<_>>(),
            None => Vec::new(),
        };

        let origin_trait = value
            .values
            .remove(perk_i)
            .formatted_value
            .unwrap_or_default();
        let name = value.values.remove(name_i).formatted_value.unwrap();

        Weapon::new(name, vec![perks_1, perks_2], origin_trait, tier)
    }

    pub async fn as_api(&self, pool: &PgPool) -> Vec<ApiWeapon> {
        let name = match self.name.as_str() {
            "Song of Ir Yut" => "Song of Ir Yût",
            "Just In Case" => "Just in Case",
            "Braytech Osprey" => "BrayTech Osprey",
            _ => self.name.as_str(),
        };

        let weapons = sqlx::query_as!(
            DestinyWeapon,
            "SELECT * FROM destiny_weapons WHERE name LIKE $1 || '%'",
            name
        )
        .fetch_all(pool)
        .await
        .unwrap();

        if weapons.is_empty() {
            // HACK: This is a temporary fix for the missing weapons
            return Vec::new();
            panic!("No weapon found for {}", name);
        }

        let api_perks = self.perks.as_api(pool).await;

        weapons
            .into_iter()
            .map(|w| ApiWeapon {
                hash: w.id as u32,
                perks: api_perks.clone(),
            })
            .collect()
    }

    pub async fn as_wishlist(&self, pool: &PgPool) -> String {
        let weapons = self.as_api(pool).await;

        let mut s = format!("// {}\n//notes: tags:pve", self.name);

        let perks = stream::iter(weapons)
            .then(|w| async move { w.perks.as_wishlist(w.hash).await })
            .collect::<Vec<_>>()
            .await
            .join("\n");
        s.push_str(&perks);

        s
    }
}

impl From<&Weapon> for CreateEmbed {
    fn from(value: &Weapon) -> Self {
        let embed = CreateEmbed::new().title(value.name.to_string()).fields(
            value
                .perks
                .iter()
                .enumerate()
                .map(|(i, p)| (format!("Perk {}", i + 1), p.0.join("\n"), false)),
        );
        // .colour(&value.priority)
        // .description(format!(
        //     "Role: {}\nSource: {}\nPriority: {}",
        //     value.role, value.source, value.priority
        // ));

        // if value.has_column_1() {
        //     embed = embed.field("Column 1", value.column_1().join("\n"), true);
        // }

        // if value.has_column_2() {
        //     embed = embed.field("Column 2", value.column_2().join("\n"), true);
        // }

        // if value.has_perks() {
        //     embed = embed.field(" ", " ", false)
        // }

        // if value.has_perk_1() {
        //     embed = embed.field("Perk 1", value.perk_1().join("\n"), true);
        // }

        // if value.has_perk_2() {
        //     embed = embed.field("Perk 2", value.perk_2().join("\n"), true);
        // }

        // if value.has_alternatives() {
        //     embed = embed.footer(CreateEmbedFooter::new(format!(
        //         "Alternatives:\n{}",
        //         value.alternatives().join("\n")
        //     )));
        // }

        embed
    }
}

impl From<Weapon> for AutocompleteChoice {
    fn from(value: Weapon) -> Self {
        AutocompleteChoice::new(value.name.clone(), value.name)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Perks(pub Vec<PerkColumn>);

impl Perks {
    pub async fn as_api(&self, pool: &PgPool) -> ApiPerks {
        async fn get_perk_ids(pool: &PgPool, perks: &[String]) -> Vec<u32> {
            let perk_records = sqlx::query_as!(
                DestinyPerk,
                "SELECT * FROM destiny_perks WHERE name = ANY($1)",
                &perks
            )
            .fetch_all(pool)
            .await
            .unwrap();

            perk_records
                .into_iter()
                .map(|perk| perk.id as u32)
                .collect()
        }

        let api_perks = stream::iter(&self.0)
            .then(|perks| get_perk_ids(pool, perks))
            .collect::<Vec<_>>()
            .await;

        ApiPerks(api_perks)
    }
}

impl Deref for Perks {
    type Target = Vec<PerkColumn>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Vec<String>>> for Perks {
    fn from(value: Vec<Vec<String>>) -> Self {
        Perks(value.into_iter().map(PerkColumn::from).collect())
    }
}

#[derive(Deserialize, Serialize)]
pub struct PerkColumn(pub Vec<String>);

impl Deref for PerkColumn {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<String>> for PerkColumn {
    fn from(value: Vec<String>) -> Self {
        PerkColumn(value)
    }
}

#[allow(dead_code)]
#[derive(FromRow)]
struct DestinyWeapon {
    id: i64,
    name: String,
    column_1: Vec<i64>,
    column_2: Vec<i64>,
    perk_1: Vec<i64>,
    perk_2: Vec<i64>,
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct DestinyPerk {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct ApiWeapon {
    pub hash: u32,
    pub perks: ApiPerks,
}

#[derive(Debug, Clone)]
pub struct ApiPerks(Vec<Vec<u32>>);

impl ApiPerks {
    pub async fn as_wishlist(&self, item_hash: u32) -> String {
        fn generate_wishlist(
            item_hash: u32,
            perks: &[Vec<u32>],
            s: &mut String,
            current_perks: &mut Vec<u32>,
            depth: usize,
        ) {
            if depth == perks.len() {
                s.push_str("\ndimwishlist:item=");
                s.push_str(&item_hash.to_string());
                s.push_str("&perks=");
                s.push_str(
                    &current_perks
                        .iter()
                        .copied()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            } else {
                for perk in &perks[depth] {
                    current_perks.push(*perk);
                    generate_wishlist(item_hash, perks, s, current_perks, depth + 1);
                    current_perks.pop();
                }
            }
        }

        let mut s = String::new();
        match self.0.len() {
            0 => String::new(),
            len => {
                let mut current_perks = Vec::with_capacity(len);
                generate_wishlist(item_hash, &self.0, &mut s, &mut current_perks, 0);
                s
            }
        }
    }
}
