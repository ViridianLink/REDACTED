use std::{collections::HashMap, ops::Deref};

use futures::{StreamExt, stream};
use google_sheets_api::types::sheet::RowData;
use serde::{Deserialize, Serialize};
use serenity::all::{AutocompleteChoice, CreateEmbed, CreateEmbedFooter};
use sqlx::{FromRow, PgPool};

use super::{affinity::Affinity, frame::Frame, tier::Tier};

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

#[derive(Default)]
pub struct WeaponBuilder {
    pub name: String,
    pub item_type: String,
    pub affinity: String,
    pub frame: Option<String>,
    pub enhanceable: bool,
    pub shield: Option<u8>,
    pub reserves: Option<u16>,
    pub column_1: String,
    pub column_2: String,
    pub origin_trait: String,
    pub rank: u8,
    pub tier: String,
}

impl WeaponBuilder {
    pub fn new(name: impl Into<String>, item_type: impl Into<String>) -> Self {
        let name = name.into();

        let name = match name.as_str() {
            "Song of Ir Yut" => String::from("Song of Ir Yût"),
            "Fang of Ir Yut" => String::from("Fang of Ir Yût"),
            "Just In Case" => String::from("Just in Case"),
            "Braytech Osprey" => String::from("BrayTech Osprey"),
            "Braytech Werewolf" => String::from("BrayTech Werewolf"),
            "Arsenic Bite-4B" => String::from("Arsenic Bite-4b"),
            "Lunulata-4B" => String::from("Lunulata-4b"),
            "IKELOS_HC_V1.0.3" => String::from("IKELOS_HC_v1.0.3"),
            "IKELOS_SMG_V1.0.3" => String::from("IKELOS_SMG_v1.0.3"),
            "Elsie's Rifle" => String::from("Elsie's Rifle (Brave)"),
            "Jararaca-3SR" => String::from("Jararaca-3sr"),
            "Redback-5SI" => String::from("Redback-5si"),
            _ => name
                .trim()
                .replace("\nBRAVE version", " (Brave)")
                .replace(" (BRAVE version)", " (Brave)"),
        };

        WeaponBuilder {
            name,
            item_type: item_type.into(),
            ..Default::default()
        }
    }

    pub fn affinity(mut self, affinity: impl Into<String>) -> Self {
        self.affinity = affinity.into();
        self
    }

    pub fn frame(mut self, frame: Option<impl Into<String>>) -> Self {
        self.frame = frame.map(|f| f.into());
        self
    }

    pub fn enhanceable(mut self, enhanceable: bool) -> Self {
        self.enhanceable = enhanceable;
        self
    }

    pub fn shield(mut self, shield: Option<u8>) -> Self {
        self.shield = shield;
        self
    }

    pub fn reserves(mut self, reserves: Option<u16>) -> Self {
        self.reserves = reserves;
        self
    }

    pub fn column_1(mut self, column_1: impl Into<String>) -> Self {
        self.column_1 = column_1.into();
        self
    }

    pub fn column_2(mut self, column_2: impl Into<String>) -> Self {
        self.column_2 = column_2.into();
        self
    }

    pub fn origin_trait(mut self, origin_trait: impl Into<String>) -> Self {
        self.origin_trait = origin_trait.into();
        self
    }

    pub fn rank(mut self, rank: u8) -> Self {
        self.rank = rank;
        self
    }

    pub fn tier(mut self, tier: impl Into<String>) -> Self {
        self.tier = tier.into();
        self
    }

    pub fn from_row_data(name: impl Into<String>, header: &RowData, row: RowData) -> Option<Self> {
        let mut data = header
            .values
            .iter()
            .zip(row.values)
            .map(|(h, r)| {
                (
                    h.formatted_value
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase(),
                    r.formatted_value.unwrap_or_default(),
                )
            })
            .collect::<HashMap<String, String>>();

        let weapon_name = data.remove("name").unwrap();

        if weapon_name == "Ideal" {
            return None;
        }

        let reserves = data
            .remove("reserves")
            .filter(|s| s != "?")
            .map(|s| s.parse().unwrap());
        let shield = data
            .remove("shield")
            .filter(|s| s != "?")
            .map(|s| s.parse().unwrap());

        let weapon = Self::new(weapon_name, name)
            .affinity(data.remove("affinity").unwrap())
            .frame(data.remove("frame"))
            .enhanceable(data.remove("enhance").unwrap() == "Yes")
            .shield(shield)
            .reserves(reserves)
            .column_1(data.remove("column 1").unwrap())
            .column_2(data.remove("column 2").unwrap())
            .origin_trait(data.remove("origin trait").unwrap())
            .rank(data.remove("rank").unwrap().parse().unwrap())
            .tier(data.remove("tier").unwrap());

        Some(weapon)
    }

    pub async fn build(self, pool: &PgPool) -> Weapon {
        let icon = sqlx::query!(
            "SELECT icon FROM destiny_weapons WHERE name = $1",
            self.name
        )
        .fetch_one(pool)
        .await
        .unwrap()
        .icon;

        Weapon {
            icon,
            name: self.name,
            item_type: self.item_type,
            affinity: self.affinity.parse().unwrap(),
            frame: self.frame.map(|f| f.parse().unwrap()),
            enhanceable: self.enhanceable,
            reserves: self.reserves,
            column_1: self.column_1,
            column_2: self.column_2,
            origin_trait: self.origin_trait,
            rank: self.rank,
            tier: self.tier.parse().unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weapon {
    pub icon: String,
    pub name: String,
    pub item_type: String,
    pub affinity: Affinity,
    pub frame: Option<Frame>,
    pub enhanceable: bool,
    pub reserves: Option<u16>,
    column_1: String,
    column_2: String,
    pub origin_trait: String,
    pub rank: u8,
    pub tier: Tier,
}

impl Weapon {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn item_type(&self) -> &str {
        &self.item_type
    }

    pub fn perks(&self) -> Perks {
        let column_1 = self.column_1.split('\n').collect::<Vec<_>>();
        let column_2 = self.column_2.split('\n').collect::<Vec<_>>();

        Perks([column_1, column_2])
    }

    pub fn tier(&self) -> String {
        self.tier.to_string()
    }

    pub async fn as_api(&self, pool: &PgPool) -> Vec<ApiWeapon> {
        let name = self.name();

        let weapons = sqlx::query_as!(
            DestinyWeapon,
            "SELECT * FROM destiny_weapons WHERE name LIKE $1 || '%'",
            name
        )
        .fetch_all(pool)
        .await
        .unwrap();

        if weapons.is_empty() {
            panic!("No weapon found for {}", name);
        }

        let api_perks = self.perks().as_api(pool).await;

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
        let mut description = format!("Affinity: {}", value.affinity);
        if let Some(frame) = &value.frame {
            description.push_str(&format!("\nFrame: {}", frame));
        }
        if let Some(reserves) = value.reserves {
            description.push_str(&format!("\nReserves: {}", reserves));
        }
        description.push_str(&format!("\nTier: {} (#{})", value.tier, value.rank));

        let embed = CreateEmbed::new()
            .title(value.name.to_string())
            .thumbnail(format!("https://www.bungie.net{}", value.icon))
            .footer(CreateEmbedFooter::new("From 'Destiny 2: Endgame Analysis'"))
            // .colour()
            .description(description)
            .fields(
                value
                    .perks()
                    .iter()
                    .enumerate()
                    .map(|(i, p)| (i + 1, p))
                    .map(|(i, p)| {
                        (
                            i,
                            p.iter()
                                .enumerate()
                                .map(|(i, line)| format!("{}. {}", i + 1, line))
                                .collect::<Vec<_>>(),
                        )
                    })
                    .map(|(i, p)| (format!("Perk {}", i), p.join("\n"), true)),
            );

        embed
    }
}

impl From<Weapon> for AutocompleteChoice {
    fn from(value: Weapon) -> Self {
        AutocompleteChoice::new(value.name.clone(), value.name)
    }
}

pub struct Perks<'a>([Vec<&'a str>; 2]);

impl Perks<'_> {
    pub async fn as_api(&self, pool: &PgPool) -> ApiPerks {
        async fn get_perk_ids(pool: &PgPool, perks: &[&str]) -> Vec<u32> {
            let perks = perks.iter().map(|s| s.to_string()).collect::<Vec<_>>();

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

impl<'a> Deref for Perks<'a> {
    type Target = [Vec<&'a str>; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
#[derive(FromRow)]
struct DestinyWeapon {
    id: i64,
    icon: String,
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
