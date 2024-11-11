use std::fmt;

use sqlx::PgPool;

use super::perk::DestinyPerk;

#[derive(Clone, PartialEq)]
pub enum Column1 {
    None,
    QuickLaunch,
    VolatileLaunch,
    BarrelShroud,
    HardLaunch,
    TemperedEdge,
    FlutedBarrel,
    JaggedEdge,
}

impl Column1 {
    pub async fn as_api(&self, pool: &PgPool) -> Vec<u32> {
        let perk = sqlx::query_as!(
            DestinyPerk,
            "SELECT * FROM destiny_perks WHERE name = $1",
            self.to_string()
        )
        .fetch_all(pool)
        .await
        .unwrap();

        perk.into_iter().map(|p| p.id as u32).collect()
    }
}

impl fmt::Display for Column1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Column1::None => write!(f, "None"),
            Column1::QuickLaunch => write!(f, "Quick Launch"),
            Column1::VolatileLaunch => write!(f, "Volatile Launch"),
            Column1::BarrelShroud => write!(f, "Barrel Shroud"),
            Column1::HardLaunch => write!(f, "Hard Launch"),
            Column1::TemperedEdge => write!(f, "Tempered Edge"),
            Column1::FlutedBarrel => write!(f, "Fluted Barrel"),
            Column1::JaggedEdge => write!(f, "Jagged Edge"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Column2 {
    None,
    HighVelocityRounds,
    HighExplosiveOrdnance,
    SpikeGrenades,
    TacticalMag,
    LightMag,
    ImplosionRounds,
    StickyGrenades,
    HeavyHuard,
    EnduringGuard,
    AppendedMag,
    ArmorPiercingRounds,
    ImpactCasing,
    LightBattery,
    ExtendedMag,
    AssaultMag,
    EnhancedBattery,
    DisorientingGrenades,
    SwordmastersGuard,
    FlaredMagwell,
    AcceleratedCoils,
}

impl Column2 {
    pub async fn as_api(&self, pool: &PgPool) -> Vec<u32> {
        let perk = sqlx::query_as!(
            DestinyPerk,
            "SELECT * FROM destiny_perks WHERE name = $1",
            self.to_string()
        )
        .fetch_all(pool)
        .await
        .unwrap();

        perk.into_iter().map(|p| p.id as u32).collect()
    }
}

impl fmt::Display for Column2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Column2::None => write!(f, "None"),
            Column2::HighVelocityRounds => write!(f, "High-Velocity Rounds"),
            Column2::HighExplosiveOrdnance => write!(f, "High-Explosive Ordnance"),
            Column2::SpikeGrenades => write!(f, "Spike Grenades"),
            Column2::TacticalMag => write!(f, "Tactical Mag"),
            Column2::LightMag => write!(f, "Light Mag"),
            Column2::ImplosionRounds => write!(f, "Implosion Rounds"),
            Column2::StickyGrenades => write!(f, "Sticky Grenades"),
            Column2::HeavyHuard => write!(f, "Heavy Huard"),
            Column2::EnduringGuard => write!(f, "Enduring Guard"),
            Column2::AppendedMag => write!(f, "Appended Mag"),
            Column2::ArmorPiercingRounds => write!(f, "Armor-Piercing Rounds"),
            Column2::ImpactCasing => write!(f, "Impact Casing"),
            Column2::LightBattery => write!(f, "Light Battery"),
            Column2::ExtendedMag => write!(f, "Extended Mag"),
            Column2::AssaultMag => write!(f, "Assault Mag"),
            Column2::EnhancedBattery => write!(f, "Enhanced Battery"),
            Column2::DisorientingGrenades => write!(f, "Disorienting Grenades"),
            Column2::SwordmastersGuard => write!(f, "Swordmaster's Guard"),
            Column2::FlaredMagwell => write!(f, "Flared Magwell"),
            Column2::AcceleratedCoils => write!(f, "Accelerated Coils"),
        }
    }
}
