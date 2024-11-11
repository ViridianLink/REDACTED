use std::fmt;

use sqlx::{prelude::FromRow, PgPool};

#[derive(PartialEq, Copy, Clone)]
pub enum Perk {
    None,
    Demolitionist,
    RepulsorBrace,
    AirTrigger,
    Reverberation,
    Deconstruct,
    HealClip,
    EnviousArsenal,
    CascadePoint,
    AttritionOrbs,
    FieldPrep,
    Physic,
    ArcConductor,
    Pugilist,
    ThreatDetector,
    ChillClip,
    Slickdraw,
    RewindRounds,
    LeadFromGold,
    Discord,
    AutoLoadingHolster,
    Reconstruction,
    Hatchling,
    EagerEdge,
    GraveRobber,
    DestabilizingRounds,
    BaitAndSwitch,
    DesperateMeasures,
    Surrounded,
    Incandescent,
    ExplosiveLight,
    CircleOfLife,
    OneTwoPunch,
    TrenchBarrel,
    VorpalWeapon,
    WitheringGaze,
    ChaosReshaped,
    KineticTremors,
    FourthTimesTheCharm,
    Dragonfly,
    Recombination,
    OneForAll,
    ColdSteel,
    PrecisionInstrument,
    Overflow,
    ControlledBurst,
    DestablizingRounds,
    ReservoirBurst,
    ImpulseAmplifier,
    Frenzy,
    KillClip,
    Slice,
    Slideways,
    ChainReaction,
    CloseToMelee,
    ShootToLoot,
    RelentlessStrikes,
    WhirlwindBlade,
    Bipod,
    Demolitionst,
    EnlightenedAction,
    RapidHit,
}

impl Perk {
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

impl fmt::Display for Perk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Perk::None => write!(f, "None"),
            Perk::Demolitionist => write!(f, "Demolitionist"),
            Perk::RepulsorBrace => write!(f, "Repulsor Brace"),
            Perk::AirTrigger => write!(f, "Air Trigger"),
            Perk::Reverberation => write!(f, "Reverberation"),
            Perk::Deconstruct => write!(f, "Deconstruct"),
            Perk::HealClip => write!(f, "Heal Clip"),
            Perk::EnviousArsenal => write!(f, "Envious Arsenal"),
            Perk::CascadePoint => write!(f, "Cascade Point"),
            Perk::AttritionOrbs => write!(f, "Attrition Orbs"),
            Perk::FieldPrep => write!(f, "Field Prep"),
            Perk::Physic => write!(f, "Physic"),
            Perk::ArcConductor => write!(f, "Arc Conductor"),
            Perk::Pugilist => write!(f, "Pugilist"),
            Perk::ThreatDetector => write!(f, "Threat Detector"),
            Perk::ChillClip => write!(f, "Chill Clip"),
            Perk::Slickdraw => write!(f, "Slickdraw"),
            Perk::RewindRounds => write!(f, "Rewind Rounds"),
            Perk::LeadFromGold => write!(f, "Lead from Gold"),
            Perk::Discord => write!(f, "Discord"),
            Perk::AutoLoadingHolster => write!(f, "Auto-Loading Holster"),
            Perk::Reconstruction => write!(f, "Reconstruction"),
            Perk::Hatchling => write!(f, "Hatchling"),
            Perk::EagerEdge => write!(f, "Eager Edge"),
            Perk::DestabilizingRounds => write!(f, "Destabilizing Rounds"),
            Perk::BaitAndSwitch => write!(f, "Bait and Switch"),
            Perk::DesperateMeasures => write!(f, "Desperate Measures"),
            Perk::Surrounded => write!(f, "Surrounded"),
            Perk::Incandescent => write!(f, "Incandescent"),
            Perk::ExplosiveLight => write!(f, "Explosive Light"),
            Perk::CircleOfLife => write!(f, "Circle of Life"),
            Perk::OneTwoPunch => write!(f, "One-Two Punch"),
            Perk::TrenchBarrel => write!(f, "Trench Barrel"),
            Perk::VorpalWeapon => write!(f, "Vorpal Weapon"),
            Perk::WitheringGaze => write!(f, "Withering Gaze"),
            Perk::ChaosReshaped => write!(f, "Chaos Reshaped"),
            Perk::KineticTremors => write!(f, "Kinetic Tremors"),
            Perk::FourthTimesTheCharm => write!(f, "Fourth Time's the Charm"),
            Perk::OneForAll => write!(f, "One for All"),
            Perk::Dragonfly => write!(f, "Dragonfly"),
            Perk::ColdSteel => write!(f, "Cold Steel"),
            Perk::PrecisionInstrument => write!(f, "Precision Instrument"),
            Perk::Overflow => write!(f, "Overflow"),
            Perk::ControlledBurst => write!(f, "Controlled Burst"),
            Perk::DestablizingRounds => write!(f, "Destablizing Rounds"),
            Perk::ReservoirBurst => write!(f, "Reservoir Burst"),
            Perk::ImpulseAmplifier => write!(f, "Impulse Amplifier"),
            Perk::Frenzy => write!(f, "Frenzy"),
            Perk::KillClip => write!(f, "Kill Clip"),
            Perk::Slice => write!(f, "Slice"),
            Perk::Slideways => write!(f, "Slideways"),
            Perk::ChainReaction => write!(f, "Chain Reaction"),
            Perk::CloseToMelee => write!(f, "Close to Melee"),
            Perk::ShootToLoot => write!(f, "Shoot to Loot"),
            Perk::RelentlessStrikes => write!(f, "Relentless Strikes"),
            Perk::WhirlwindBlade => write!(f, "Whirlwind Blade"),
            Perk::Bipod => write!(f, "Bipod"),
            Perk::Demolitionst => write!(f, "Demolitionist"),
            Perk::EnlightenedAction => write!(f, "Enlightened Action"),
            Perk::RapidHit => write!(f, "Rapid Hit"),
            Perk::GraveRobber => write!(f, "Grave Robber"),
            Perk::Recombination => write!(f, "Recombination"),
        }
    }
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct DestinyPerk {
    pub id: i64,
    pub name: String,
}
