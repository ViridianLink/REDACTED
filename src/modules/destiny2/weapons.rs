use futures::{stream, StreamExt};
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use sqlx::prelude::FromRow;
use sqlx::PgPool;

use crate::Result;

use super::column::Column1;
use super::column::Column2;
use super::perk::Perk;
use super::priority::Priority;
use super::role::Role;
use super::source::Source;
use super::weapon_name::WeaponName;

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
//     column_1: Column1::None,
//     column_2: Column2::None,
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

#[derive(Clone)]
pub struct Weapon {
    pub name: WeaponName,
    pub role: Role,
    pub source: Source,
    pub priority: Priority,
    pub column_1: [Column1; 4],
    pub column_2: [Column2; 4],
    pub perk_1: [Perk; 4],
    pub perk_2: [Perk; 4],
    pub alternatives: [WeaponName; 3],
}

impl Weapon {
    pub fn has_column_1(&self) -> bool {
        self.column_1[0] != Column1::None
    }

    pub fn column_1(&self) -> Vec<String> {
        self.column_1
            .iter()
            .filter(|c| **c != Column1::None)
            .map(|c| c.to_string())
            .collect()
    }

    pub fn has_column_2(&self) -> bool {
        self.column_2[0] != Column2::None
    }

    pub fn column_2(&self) -> Vec<String> {
        self.column_2
            .iter()
            .filter(|c| **c != Column2::None)
            .map(|c| c.to_string())
            .collect()
    }

    pub fn has_perks(&self) -> bool {
        self.has_perk_1() || self.has_perk_2()
    }

    pub fn has_perk_1(&self) -> bool {
        self.perk_1[0] != Perk::None
    }

    pub fn perk_1(&self) -> Vec<String> {
        self.perk_1
            .iter()
            .filter(|p| **p != Perk::None)
            .map(|p| p.to_string())
            .collect()
    }

    pub fn has_perk_2(&self) -> bool {
        self.perk_2[0] != Perk::None
    }

    pub fn perk_2(&self) -> Vec<String> {
        self.perk_2
            .iter()
            .filter(|p| **p != Perk::None)
            .map(|p| p.to_string())
            .collect()
    }

    pub fn perks(&self) -> Vec<&Perk> {
        self.perk_1.iter().chain(self.perk_2.iter()).collect()
    }

    pub fn has_alternatives(&self) -> bool {
        self.alternatives[0] != WeaponName::None
    }

    pub fn alternatives(&self) -> Vec<String> {
        self.alternatives
            .iter()
            .filter(|a| **a != WeaponName::None)
            .map(|a| a.to_string())
            .collect()
    }

    pub async fn as_api(&self, pool: &PgPool) -> Result<(ApiWeapon, Option<ApiWeapon>)> {
        let mut weapons = sqlx::query_as!(
            DestinyWeapon,
            "SELECT * FROM destiny_weapons WHERE name LIKE $1 || '%'",
            self.name.to_string()
        )
        .fetch_all(pool)
        .await?
        .into_iter();

        let mut weapon = weapons.next().unwrap();

        let desired_column_1 = stream::iter(self.column_1.iter())
            .filter_map(|perk| async move {
                if !matches!(perk, Column1::None) {
                    Some(perk.as_api(pool).await)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        weapon
            .column_1
            .retain(|p| desired_column_1.contains(&(*p as u32)));

        let desired_column_2 = stream::iter(self.column_2.iter())
            .filter_map(|perk| async move {
                if !matches!(perk, Column2::None) {
                    Some(perk.as_api(pool).await)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        weapon
            .column_2
            .retain(|p| desired_column_2.contains(&(*p as u32)));

        let desired_perk_1 = stream::iter(self.perk_1.iter())
            .filter_map(|perk| async move {
                if !matches!(perk, Perk::None) {
                    Some(perk.as_api(pool).await)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        weapon
            .perk_1
            .retain(|p| desired_perk_1.contains(&(*p as u32)));

        let desired_perk_2 = stream::iter(self.perk_2.iter())
            .filter_map(|perk| async move {
                if !matches!(perk, Perk::None) {
                    Some(perk.as_api(pool).await)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        weapon
            .perk_2
            .retain(|p| desired_perk_2.contains(&(*p as u32)));

        let adept_weapon = weapons.next().map(|mut w| {
            w.column_1 = weapon.column_1.clone();
            w.column_2 = weapon.column_2.clone();
            w.perk_1 = weapon.perk_1.clone();
            w.perk_2 = weapon.perk_2.clone();
            ApiWeapon::from(&w)
        });

        Ok((ApiWeapon::from(&weapon), adept_weapon))
    }
}

impl From<&Weapon> for CreateEmbed {
    fn from(value: &Weapon) -> Self {
        let mut embed = CreateEmbed::new()
            .title(value.name.to_string())
            .colour(&value.priority)
            .description(format!(
                "Role: {}\nSource: {}\nPriority: {}",
                value.role, value.source, value.priority
            ));

        if value.has_column_1() {
            embed = embed.field("Column 1", value.column_1().join("\n"), true);
        }

        if value.has_column_2() {
            embed = embed.field("Column 2", value.column_2().join("\n"), true);
        }

        if value.has_perks() {
            embed = embed.field(" ", " ", false)
        }

        if value.has_perk_1() {
            embed = embed.field("Perk 1", value.perk_1().join("\n"), true);
        }

        if value.has_perk_2() {
            embed = embed.field("Perk 2", value.perk_2().join("\n"), true);
        }

        if value.has_alternatives() {
            embed = embed.footer(CreateEmbedFooter::new(format!(
                "Alternatives:\n{}",
                value.alternatives().join("\n")
            )));
        }

        embed
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

#[derive(Debug)]
pub struct ApiWeapon {
    pub hash: u32,
    pub column_1: Vec<u32>,
    pub column_2: Vec<u32>,
    pub perk_1: Vec<u32>,
    pub perk_2: Vec<u32>,
}

impl ApiWeapon {
    pub fn perks(&self) -> Vec<u32> {
        self.perk_1
            .iter()
            .copied()
            .chain(self.perk_2.iter().copied())
            .collect()
    }
}

impl From<&DestinyWeapon> for ApiWeapon {
    fn from(value: &DestinyWeapon) -> Self {
        Self {
            hash: value.id as u32,
            column_1: value.column_1.iter().map(|i| *i as u32).collect(),
            column_2: value.column_2.iter().map(|i| *i as u32).collect(),
            perk_1: value.perk_1.iter().map(|i| *i as u32).collect(),
            perk_2: value.perk_2.iter().map(|i| *i as u32).collect(),
        }
    }
}

//region High Priority
pub const VS_VELOCITY_BATON: Weapon = Weapon {
    name: WeaponName::VSVelocityBaton,
    role: Role::OrbGenGL,
    source: Source::VespersHost,
    priority: Priority::High,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::HighVelocityRounds,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::Demolitionist,
        Perk::RepulsorBrace,
        Perk::None,
        Perk::None,
    ],
    perk_2: [
        Perk::AttritionOrbs,
        Perk::DestabilizingRounds,
        Perk::BaitAndSwitch,
        Perk::None,
    ],
    alternatives: [WeaponName::None, WeaponName::None, WeaponName::None],
};

pub const TINASHAS_MASTERY: Weapon = Weapon {
    name: WeaponName::TinashasMastery,
    role: Role::KineticRocketSidearm,
    source: Source::IronBanner,
    priority: Priority::High,
    column_1: [
        Column1::VolatileLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::HighExplosiveOrdnance,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::AirTrigger,
        Perk::Reverberation,
        Perk::Deconstruct,
        Perk::None,
    ],
    perk_2: [
        Perk::ChillClip,
        Perk::DesperateMeasures,
        Perk::Surrounded,
        Perk::None,
    ],
    alternatives: [WeaponName::TheCall, WeaponName::None, WeaponName::None],
};

pub const ABERRANT_ACTION: Weapon = Weapon {
    name: WeaponName::AberrantAction,
    role: Role::EnergyRocketSidearm,
    source: Source::Season24,
    priority: Priority::High,
    column_1: [
        Column1::VolatileLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::HighExplosiveOrdnance,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::HealClip, Perk::FieldPrep, Perk::None, Perk::None],
    perk_2: [Perk::Incandescent, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::IndebtedKindness,
        WeaponName::None,
        WeaponName::None,
    ],
};

pub const VS_CHILL_INHIBITOR: Weapon = Weapon {
    name: WeaponName::VSChillInhibitor,
    role: Role::DpsHgl,
    source: Source::VespersHost,
    priority: Priority::High,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::SpikeGrenades,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::EnviousArsenal,
        Perk::CascadePoint,
        Perk::AttritionOrbs,
        Perk::None,
    ],
    perk_2: [
        Perk::BaitAndSwitch,
        Perk::ExplosiveLight,
        Perk::Surrounded,
        Perk::None,
    ],
    alternatives: [
        WeaponName::BitterSweet,
        WeaponName::WickedSister,
        WeaponName::EdgeTransit,
    ],
};

pub const SUNSHOT: Weapon = Weapon {
    name: WeaponName::Sunshot,
    role: Role::ExoticEnergyPrimary,
    source: Source::Quest,
    priority: Priority::High,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::GravitonLance,
        WeaponName::TrinityGhoul,
        WeaponName::None,
    ],
};

pub const LEVIATHANS_BREATH: Weapon = Weapon {
    name: WeaponName::LeviathansBreath,
    role: Role::ExoticConsistentHeavy,
    source: Source::Kiosk,
    priority: Priority::High,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::OneThousandVoices,
        WeaponName::TheProspector,
        WeaponName::Microcosm,
    ],
};

pub const WHISPER_OF_THE_WORM: Weapon = Weapon {
    name: WeaponName::WhisperOfTheWorm,
    role: Role::ExoticTotalHeavy,
    source: Source::TheWhisper,
    priority: Priority::High,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::FieldPrep, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::GrandOverture,
        WeaponName::LegendOfAcrius,
        WeaponName::None,
    ],
};

pub const TRACTOR_CANNON: Weapon = Weapon {
    name: WeaponName::TractorCannon,
    role: Role::ExoticDebuff,
    source: Source::Quest,
    priority: Priority::High,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [WeaponName::Divinity, WeaponName::None, WeaponName::None],
};
//endregion

//region Medium Priority
pub const NO_HESITATION: Weapon = Weapon {
    name: WeaponName::NoHesitation,
    role: Role::SupportAuto,
    source: Source::PaleHeart,
    priority: Priority::Medium,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::Physic, Perk::None, Perk::None, Perk::None],
    perk_2: [
        Perk::Incandescent,
        Perk::CircleOfLife,
        Perk::AttritionOrbs,
        Perk::None,
    ],
    alternatives: [WeaponName::None, WeaponName::None, WeaponName::None],
};

pub const ERGO_SUM: Weapon = Weapon {
    name: WeaponName::ErgoSum,
    role: Role::ExoticDRAddClear,
    source: Source::ExoticQuest,
    priority: Priority::Medium,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::ArcConductor, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::Riskrunner,
        WeaponName::Tarrabah,
        WeaponName::None,
    ],
};

pub const PERFECT_PARADOX: Weapon = Weapon {
    name: WeaponName::PerfectParadox,
    role: Role::Kinetic12pShotgun,
    source: Source::Season24,
    priority: Priority::Medium,
    column_1: [
        Column1::BarrelShroud,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::TacticalMag,
        Column2::LightMag,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::FieldPrep,
        Perk::Pugilist,
        Perk::ThreatDetector,
        Perk::None,
    ],
    perk_2: [
        Perk::OneTwoPunch,
        Perk::TrenchBarrel,
        Perk::VorpalWeapon,
        Perk::None,
    ],
    alternatives: [
        WeaponName::WastelanderM5,
        WeaponName::Swordbreaker,
        WeaponName::OneSmallStep,
    ],
};

pub const VELEDA_F: Weapon = Weapon {
    name: WeaponName::VeledaF,
    role: Role::WeakenOnDemand,
    source: Source::World,
    priority: Priority::Medium,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::TacticalMag,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::AirTrigger, Perk::Slickdraw, Perk::None, Perk::None],
    perk_2: [Perk::WitheringGaze, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::Sovereignty,
        WeaponName::VSGraviticArrest,
        WeaponName::None,
    ],
};

pub const CRITICAL_ANOMALY: Weapon = Weapon {
    name: WeaponName::CriticalAnomaly,
    role: Role::HitscanOverloadStun,
    source: Source::SalvationsEdge,
    priority: Priority::Medium,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::TacticalMag,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::ChillClip, Perk::RewindRounds, Perk::None, Perk::None],
    perk_2: [Perk::ChaosReshaped, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::TinashasMastery,
        WeaponName::RakeAngle,
        WeaponName::Liturgy,
    ],
};

pub const THE_SUPREMACY: Weapon = Weapon {
    name: WeaponName::TheSupremacy,
    role: Role::KineticDamageSniper,
    source: Source::LastWish,
    priority: Priority::Medium,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::AppendedMag,
        Column2::ArmorPiercingRounds,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::RewindRounds,
        Perk::LeadFromGold,
        Perk::Discord,
        Perk::None,
    ],
    perk_2: [
        Perk::KineticTremors,
        Perk::BaitAndSwitch,
        Perk::FourthTimesTheCharm,
        Perk::None,
    ],
    alternatives: [
        WeaponName::Irukandji,
        WeaponName::CriticalAnomaly,
        WeaponName::None,
    ],
};

pub const LOST_SIGNAL: Weapon = Weapon {
    name: WeaponName::LostSignal,
    role: Role::TranscendenceGL,
    source: Source::Season24,
    priority: Priority::Medium,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::HighVelocityRounds,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::AutoLoadingHolster,
        Perk::LeadFromGold,
        Perk::None,
        Perk::None,
    ],
    perk_2: [
        Perk::VorpalWeapon,
        Perk::Demolitionist,
        Perk::OneForAll,
        Perk::None,
    ],
    alternatives: [WeaponName::None, WeaponName::None, WeaponName::None],
};

pub const PRO_MEMORIA: Weapon = Weapon {
    name: WeaponName::ProMemoria,
    role: Role::MachineGun,
    source: Source::PaleHeart,
    priority: Priority::Medium,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::ExtendedMag,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::Demolitionist,
        Perk::Reconstruction,
        Perk::Hatchling,
        Perk::None,
    ],
    perk_2: [
        Perk::DesperateMeasures,
        Perk::BaitAndSwitch,
        Perk::Dragonfly,
        Perk::None,
    ],
    alternatives: [
        WeaponName::Commemoration,
        WeaponName::SongOfIrYut,
        WeaponName::Avalance,
    ],
};

pub const THE_SLAMMER: Weapon = Weapon {
    name: WeaponName::TheSlammer,
    role: Role::MovementSword,
    source: Source::Nightfall,
    priority: Priority::Medium,
    column_1: [
        Column1::TemperedEdge,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::HeavyHuard,
        Column2::EnduringGuard,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::EagerEdge, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::ColdSteel, Perk::Demolitionist, Perk::None, Perk::None],
    alternatives: [
        WeaponName::FallingGuillotine,
        WeaponName::None,
        WeaponName::None,
    ],
};

pub const HELIOCENTRIC_QSC: Weapon = Weapon {
    name: WeaponName::HeliocentricQSc,
    role: Role::EnergyPrimary,
    source: Source::World,
    priority: Priority::Medium,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::HealClip, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::Incandescent, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::AnonymousAutumn,
        WeaponName::Nullify,
        WeaponName::LunasHowl,
    ],
};

pub const PARASITE: Weapon = Weapon {
    name: WeaponName::Parasite,
    role: Role::ExoticBurstHeavy,
    source: Source::ExoticQuest,
    priority: Priority::Medium,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::GrandOverture,
        WeaponName::TheWardcliffCoil,
        WeaponName::None,
    ],
};

pub const GJALLARHORN: Weapon = Weapon {
    name: WeaponName::Gjallarhorn,
    role: Role::ExoticAddClearHeavy,
    source: Source::ExoticQuest,
    priority: Priority::Medium,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::Xenophage,
        WeaponName::Thunderlord,
        WeaponName::None,
    ],
};
//endregion
//region Low Priority
pub const OUTBREAK_PERFECTED: Weapon = Weapon {
    name: WeaponName::OutbreakPerfected,
    role: Role::AmmolessDamage,
    source: Source::ZeroHour,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::RewindRounds, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [WeaponName::FinalWarning, WeaponName::None, WeaponName::None],
};

pub const IKELOS_SG_V1_0_3: Weapon = Weapon {
    name: WeaponName::IKELOSSGV103,
    role: Role::Energy12pShotgun,
    source: Source::OperationSeraphsShield,
    priority: Priority::Low,
    column_1: [
        Column1::BarrelShroud,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::TacticalMag,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::GraveRobber,
        Perk::ThreatDetector,
        Perk::Pugilist,
        Perk::None,
    ],
    perk_2: [
        Perk::OneTwoPunch,
        Perk::Surrounded,
        Perk::CascadePoint,
        Perk::None,
    ],
    alternatives: [
        WeaponName::ProphetOfDoom,
        WeaponName::DeadWeight,
        WeaponName::BassoOstinato,
    ],
};

pub const HERITAGE: Weapon = Weapon {
    name: WeaponName::Heritage,
    role: Role::KineticBurstShotgun,
    source: Source::DeepStoneCrypt,
    priority: Priority::Low,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::AssaultMag,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::Reconstruction,
        Perk::Demolitionist,
        Perk::None,
        Perk::None,
    ],
    perk_2: [Perk::Recombination, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::ImperialDecree,
        WeaponName::UntilItsReturn,
        WeaponName::Someday,
    ],
};

pub const OMNISCIENT_EYE: Weapon = Weapon {
    name: WeaponName::OmniscientEye,
    role: Role::EnergyDamageSniper,
    source: Source::GardenOfSalvation,
    priority: Priority::Low,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::TacticalMag,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::FourthTimesTheCharm,
        Perk::EnviousArsenal,
        Perk::None,
        Perk::None,
    ],
    perk_2: [
        Perk::PrecisionInstrument,
        Perk::VorpalWeapon,
        Perk::None,
        Perk::None,
    ],
    alternatives: [
        WeaponName::IKELOSSRV103,
        WeaponName::TwilightOath,
        WeaponName::DistantTumulus,
    ],
};

pub const SCATTER_SIGNAL: Weapon = Weapon {
    name: WeaponName::ScatterSignal,
    role: Role::KineticDamageFusion,
    source: Source::Season23,
    priority: Priority::Low,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::EnhancedBattery,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::Overflow, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::ControlledBurst, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::Riptide,
        WeaponName::NoxPerennialV,
        WeaponName::None,
    ],
};

pub const ZEALOTS_REWARD: Weapon = Weapon {
    name: WeaponName::ZealotsReward,
    role: Role::EnergyDamageFusion,
    source: Source::GardenOfSalvation,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [
        Perk::AutoLoadingHolster,
        Perk::LeadFromGold,
        Perk::DestablizingRounds,
        Perk::None,
    ],
    perk_2: [
        Perk::ReservoirBurst,
        Perk::ControlledBurst,
        Perk::None,
        Perk::None,
    ],
    alternatives: [
        WeaponName::RoyalExecutioner,
        WeaponName::TecheunForce,
        WeaponName::AxialLacuna,
    ],
};

pub const THE_MOUNTAINTOP: Weapon = Weapon {
    name: WeaponName::TheMountaintop,
    role: Role::MovementGL,
    source: Source::Onslaught,
    priority: Priority::Low,
    column_1: [
        Column1::HardLaunch,
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::ImplosionRounds,
        Column2::StickyGrenades,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::AutoLoadingHolster,
        Perk::Overflow,
        Perk::ImpulseAmplifier,
        Perk::None,
    ],
    perk_2: [
        Perk::Frenzy,
        Perk::Recombination,
        Perk::VorpalWeapon,
        Perk::None,
    ],
    alternatives: [WeaponName::Alethonym, WeaponName::None, WeaponName::None],
};

pub const MARTYRS_RETRIBUTION: Weapon = Weapon {
    name: WeaponName::MartyrsRetribution,
    role: Role::EnergyAddClearWave,
    source: Source::Season24,
    priority: Priority::Low,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::HighVelocityRounds,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::HealClip,
        Perk::Demolitionist,
        Perk::AutoLoadingHolster,
        Perk::None,
    ],
    perk_2: [Perk::Incandescent, Perk::KillClip, Perk::None, Perk::None],
    alternatives: [WeaponName::Forbearance, WeaponName::None, WeaponName::None],
};

pub const TUST_OF_THE_BOAR: Weapon = Weapon {
    name: WeaponName::TuskOfTheBoar,
    role: Role::KineticAddClearWave,
    source: Source::IronBanner,
    priority: Priority::Low,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::HighVelocityRounds,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::Slideways, Perk::Slice, Perk::None, Perk::None],
    perk_2: [
        Perk::ChainReaction,
        Perk::Deconstruct,
        Perk::None,
        Perk::None,
    ],
    alternatives: [
        WeaponName::NewPacificEpitaph,
        WeaponName::None,
        WeaponName::None,
    ],
};

pub const LITURGY: Weapon = Weapon {
    name: WeaponName::Liturgy,
    role: Role::KineticBlindingGL,
    source: Source::Season25,
    priority: Priority::Low,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::SpikeGrenades,
        Column2::DisorientingGrenades,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::Slideways,
        Perk::EnviousArsenal,
        Perk::None,
        Perk::None,
    ],
    perk_2: [
        Perk::ChillClip,
        Perk::LeadFromGold,
        Perk::ChainReaction,
        Perk::None,
    ],
    alternatives: [
        WeaponName::TheMilitiasBirthright,
        WeaponName::IgnitionCode,
        WeaponName::PardonOurDust,
    ],
};

pub const WILDERFLIGHT: Weapon = Weapon {
    name: WeaponName::Wilderflight,
    role: Role::EnergyBlindingGL,
    source: Source::SpireOfTheWatcher,
    priority: Priority::Low,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::DisorientingGrenades,
        Column2::SpikeGrenades,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::AutoLoadingHolster,
        Perk::Demolitionist,
        Perk::None,
        Perk::None,
    ],
    perk_2: [
        Perk::Frenzy,
        Perk::VorpalWeapon,
        Perk::LeadFromGold,
        Perk::None,
    ],
    alternatives: [
        WeaponName::WildStyle,
        WeaponName::EmptyVessel,
        WeaponName::None,
    ],
};

pub const RAKE_ANGLE: Weapon = Weapon {
    name: WeaponName::RakeAngle,
    role: Role::Glaive,
    source: Source::Nightfall,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [
        Perk::ImpulseAmplifier,
        Perk::LeadFromGold,
        Perk::None,
        Perk::None,
    ],
    perk_2: [Perk::ChillClip, Perk::CloseToMelee, Perk::None, Perk::None],
    alternatives: [
        WeaponName::ForthcomingDeviance,
        WeaponName::NezarecsWhisper,
        WeaponName::TheEnigma,
    ],
};

pub const CHRONOPHAGE: Weapon = Weapon {
    name: WeaponName::Chronophage,
    role: Role::SpecialShootToLoot,
    source: Source::Season24,
    priority: Priority::Low,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::LightBattery,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::ShootToLoot,
        Perk::RepulsorBrace,
        Perk::None,
        Perk::None,
    ],
    perk_2: [
        Perk::DestabilizingRounds,
        Perk::Demolitionist,
        Perk::DesperateMeasures,
        Perk::None,
    ],
    alternatives: [
        WeaponName::PathOfLeastResistance,
        WeaponName::RetracedPath,
        WeaponName::None,
    ],
};

pub const SUMMUM_BONUM: Weapon = Weapon {
    name: WeaponName::SummumBonum,
    role: Role::DPSSword,
    source: Source::SalvationsEdge,
    priority: Priority::Low,
    column_1: [
        Column1::JaggedEdge,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::SwordmastersGuard,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [
        Perk::RelentlessStrikes,
        Perk::AttritionOrbs,
        Perk::Deconstruct,
        Perk::None,
    ],
    perk_2: [
        Perk::WhirlwindBlade,
        Perk::ChaosReshaped,
        Perk::Surrounded,
        Perk::None,
    ],
    alternatives: [
        WeaponName::GeodeticHSm,
        WeaponName::Bequest,
        WeaponName::IllOmen,
    ],
};

pub const TOMORROWS_ANSWER_1: Weapon = Weapon {
    name: WeaponName::TomorrowsAnswer,
    role: Role::DPSRocket,
    source: Source::TrialsOfOsiris,
    priority: Priority::Low,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::ImpactCasing,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::EnviousArsenal, Perk::None, Perk::None, Perk::None],
    perk_2: [
        Perk::BaitAndSwitch,
        Perk::ExplosiveLight,
        Perk::None,
        Perk::None,
    ],
    alternatives: [
        WeaponName::ApexPredator,
        WeaponName::CruxTerminationIV,
        WeaponName::ColdComfort,
    ],
};

pub const TOMORROWS_ANSWER_2: Weapon = Weapon {
    name: WeaponName::TomorrowsAnswer,
    role: Role::AddClearRocket,
    source: Source::TrialsOfOsiris,
    priority: Priority::Low,
    column_1: [
        Column1::QuickLaunch,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::ImpactCasing,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::AirTrigger, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::Bipod, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::BraytechOsprey,
        WeaponName::FaithKeeper,
        WeaponName::Semiotician,
    ],
};

pub const SCINTILLATION: Weapon = Weapon {
    name: WeaponName::Scintillation,
    role: Role::Linear,
    source: Source::Nightfall,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::RewindRounds, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::BaitAndSwitch, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::DoomedPetitioner,
        WeaponName::Cataclysmic,
        WeaponName::BriarsContempt,
    ],
};

pub const MULTIMACH_CCX: Weapon = Weapon {
    name: WeaponName::MultimachCCX,
    role: Role::KineticPrimary,
    source: Source::IronBanner,
    priority: Priority::Low,
    column_1: [
        Column1::FlutedBarrel,
        Column1::None,
        Column1::None,
        Column1::None,
    ],
    column_2: [
        Column2::FlaredMagwell,
        Column2::None,
        Column2::None,
        Column2::None,
    ],
    perk_1: [Perk::AttritionOrbs, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::KineticTremors, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::Imminence,
        WeaponName::MidnightCoup,
        WeaponName::AccruedRedemption,
    ],
};

pub const WARDENS_LAW: Weapon = Weapon {
    name: WeaponName::WardensLaw,
    role: Role::KineticLPHC,
    source: Source::Nightfall,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [
        Perk::Demolitionst,
        Perk::EnlightenedAction,
        Perk::None,
        Perk::None,
    ],
    perk_2: [Perk::VorpalWeapon, Perk::None, Perk::None, Perk::None],
    alternatives: [WeaponName::None, WeaponName::None, WeaponName::None],
};

pub const YESTERDAYS_QUESTION: Weapon = Weapon {
    name: WeaponName::YesterdaysQuestion,
    role: Role::EnergyLPHC,
    source: Source::TrialsOfOsiris,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::RapidHit, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::VorpalWeapon, Perk::None, Perk::None, Perk::None],
    alternatives: [WeaponName::MaahesHC4, WeaponName::None, WeaponName::None],
};

pub const KHVOSTOV_7G_0X: Weapon = Weapon {
    name: WeaponName::Khvostov7G0X,
    role: Role::ExoticKineticPrimary,
    source: Source::ExoticQuest,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::Necrochasm,
        WeaponName::BadJuju,
        WeaponName::TheHuckleberry,
    ],
};

pub const THE_FOURTH_HORSEMAN: Weapon = Weapon {
    name: WeaponName::TheFourthHorseman,
    role: Role::ExoticBurstSpecial,
    source: Source::Kiosk,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::ChoirOfOne,
        WeaponName::StillHunt,
        WeaponName::IzanagisBurden,
    ],
};

pub const BURIED_BLOODLINE: Weapon = Weapon {
    name: WeaponName::BuriedBloodline,
    role: Role::ExoticSurvivability,
    source: Source::WarlordsRuin,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [
        WeaponName::RedDeathReformed,
        WeaponName::None,
        WeaponName::None,
    ],
};

pub const EUPHONY: Weapon = Weapon {
    name: WeaponName::Euphony,
    role: Role::ExoticTotalSpecial,
    source: Source::SalvationsEdge,
    priority: Priority::Low,
    column_1: [Column1::None, Column1::None, Column1::None, Column1::None],
    column_2: [Column2::None, Column2::None, Column2::None, Column2::None],
    perk_1: [Perk::None, Perk::None, Perk::None, Perk::None],
    perk_2: [Perk::None, Perk::None, Perk::None, Perk::None],
    alternatives: [WeaponName::Cloudstrike, WeaponName::None, WeaponName::None],
};
//endregion
