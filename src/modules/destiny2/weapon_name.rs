use std::{collections::HashMap, fmt};

use bungie_api::types::{
    common::DestinyDisplayPropertiesDefinition, definitions::DestinyInventoryItemDefinition,
    destiny::DamageType,
};

#[derive(Clone, PartialEq)]
pub enum WeaponName {
    None,
    VSVelocityBaton,
    TinashasMastery,
    TheCall,
    AberrantAction,
    IndebtedKindness,
    VSChillInhibitor,
    BitterSweet,
    WickedSister,
    EdgeTransit,
    Sunshot,
    GravitonLance,
    TrinityGhoul,
    LeviathansBreath,
    OneThousandVoices,
    TheProspector,
    Microcosm,
    WhisperOfTheWorm,
    GrandOverture,
    LegendOfAcrius,
    TractorCannon,
    Divinity,
    NoHesitation,
    ErgoSum,
    Riskrunner,
    Tarrabah,
    PerfectParadox,
    WastelanderM5,
    Swordbreaker,
    OneSmallStep,
    VeledaF,
    Sovereignty,
    VSGraviticArrest,
    CriticalAnomaly,
    RakeAngle,
    Liturgy,
    TheSupremacy,
    Irukandji,
    LostSignal,
    ProMemoria,
    Commemoration,
    SongOfIrYut,
    Avalance,
    TheSlammer,
    FallingGuillotine,
    HeliocentricQSc,
    AnonymousAutumn,
    Nullify,
    LunasHowl,
    Parasite,
    TheWardcliffCoil,
    Gjallarhorn,
    Xenophage,
    Thunderlord,
    OutbreakPerfected,
    FinalWarning,
    IKELOSSGV103,
    ProphetOfDoom,
    DeadWeight,
    BassoOstinato,
    Heritage,
    ImperialDecree,
    UntilItsReturn,
    Someday,
    OmniscientEye,
    IKELOSSRV103,
    TwilightOath,
    DistantTumulus,
    ScatterSignal,
    Riptide,
    NoxPerennialV,
    ZealotsReward,
    RoyalExecutioner,
    TecheunForce,
    AxialLacuna,
    TheMountaintop,
    Alethonym,
    MartyrsRetribution,
    Forbearance,
    TuskOfTheBoar,
    NewPacificEpitaph,
    TheMilitiasBirthright,
    IgnitionCode,
    PardonOurDust,
    Wilderflight,
    WildStyle,
    EmptyVessel,
    ForthcomingDeviance,
    NezarecsWhisper,
    TheEnigma,
    Chronophage,
    PathOfLeastResistance,
    RetracedPath,
    SummumBonum,
    GeodeticHSm,
    Bequest,
    IllOmen,
    TomorrowsAnswer,
    ApexPredator,
    CruxTerminationIV,
    ColdComfort,
    BraytechOsprey,
    FaithKeeper,
    Semiotician,
    Scintillation,
    DoomedPetitioner,
    Cataclysmic,
    BriarsContempt,
    MultimachCCX,
    Imminence,
    MidnightCoup,
    AccruedRedemption,
    WardensLaw,
    YesterdaysQuestion,
    MaahesHC4,
    Khvostov7G0X,
    Necrochasm,
    BadJuju,
    TheHuckleberry,
    TheFourthHorseman,
    ChoirOfOne,
    StillHunt,
    IzanagisBurden,
    BuriedBloodline,
    RedDeathReformed,
    Euphony,
    Cloudstrike,
}

impl WeaponName {
    pub async fn to_api<'a>(
        &self,
        manifest: &'a HashMap<String, DestinyInventoryItemDefinition>,
    ) -> &'a DestinyInventoryItemDefinition {
        let name_str = self.to_string();
        let items = manifest.values().collect::<Vec<_>>();

        let mut valid_items_iter = items.into_iter().filter(|item| match item {
            DestinyInventoryItemDefinition {
                default_damage_type: DamageType::None,
                ..
            } => false,
            DestinyInventoryItemDefinition {
                display_properties: DestinyDisplayPropertiesDefinition { name, .. },
                ..
            } if name == &name_str => true,
            _ => false,
        });

        let valid_items = valid_items_iter.next().unwrap();

        if valid_items_iter.next().is_some() {
            panic!("Multiple items with the same name: {}", name_str);
        }

        valid_items
    }
}

impl fmt::Display for WeaponName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponName::None => write!(f, "None"),
            WeaponName::VSVelocityBaton => write!(f, "VS Velocity Baton"),
            WeaponName::TinashasMastery => write!(f, "Tinasha's Mastery"),
            WeaponName::TheCall => write!(f, "The Call"),
            WeaponName::AberrantAction => write!(f, "Aberrant Action"),
            WeaponName::IndebtedKindness => write!(f, "Indebted Kindness"),
            WeaponName::VSChillInhibitor => write!(f, "VS Chill Inhibitor"),
            WeaponName::BitterSweet => write!(f, "Bitter/Sweet"),
            WeaponName::WickedSister => write!(f, "Wicked Sister"),
            WeaponName::EdgeTransit => write!(f, "Edge Transit"),
            WeaponName::Sunshot => write!(f, "Sunshot"),
            WeaponName::GravitonLance => write!(f, "Graviton Lance"),
            WeaponName::TrinityGhoul => write!(f, "Trinity Ghoul"),
            WeaponName::LeviathansBreath => write!(f, "Leviathan's Breath"),
            WeaponName::OneThousandVoices => write!(f, "One Thousand Voices"),
            WeaponName::TheProspector => write!(f, "The Prospector"),
            WeaponName::Microcosm => write!(f, "Microcosm"),
            WeaponName::WhisperOfTheWorm => write!(f, "Whisper of the Worm"),
            WeaponName::GrandOverture => write!(f, "Grand Overture"),
            WeaponName::LegendOfAcrius => write!(f, "Legend of Acrius"),
            WeaponName::TractorCannon => write!(f, "Tractor Cannon"),
            WeaponName::Divinity => write!(f, "Divinity"),
            WeaponName::NoHesitation => write!(f, "No Hesitation"),
            WeaponName::ErgoSum => write!(f, "Ergo Sum"),
            WeaponName::Riskrunner => write!(f, "Riskrunner"),
            WeaponName::Tarrabah => write!(f, "Tarrabah"),
            WeaponName::PerfectParadox => write!(f, "Perfect Paradox"),
            WeaponName::WastelanderM5 => write!(f, "Wastelander M5"),
            WeaponName::Swordbreaker => write!(f, "Swordbreaker"),
            WeaponName::OneSmallStep => write!(f, "One Small Step"),
            WeaponName::VeledaF => write!(f, "Veleda-F"),
            WeaponName::Sovereignty => write!(f, "Sovereignty"),
            WeaponName::VSGraviticArrest => write!(f, "VS Gravitic Arrest"),
            WeaponName::CriticalAnomaly => write!(f, "Critical Anomaly"),
            WeaponName::RakeAngle => write!(f, "Rake Angle"),
            WeaponName::Liturgy => write!(f, "Liturgy"),
            WeaponName::TheSupremacy => write!(f, "The Supremacy"),
            WeaponName::Irukandji => write!(f, "Irukandji"),
            WeaponName::LostSignal => write!(f, "Lost Signal"),
            WeaponName::ProMemoria => write!(f, "Pro Memoria"),
            WeaponName::Commemoration => write!(f, "Commemoration"),
            WeaponName::SongOfIrYut => write!(f, "Song of Ir Yut"),
            WeaponName::Avalance => write!(f, "Avalanche"),
            WeaponName::TheSlammer => write!(f, "The Slammer"),
            WeaponName::FallingGuillotine => write!(f, "Falling Guillotine"),
            WeaponName::HeliocentricQSc => write!(f, "Heliocentric QSc"),
            WeaponName::AnonymousAutumn => write!(f, "Anonymous Autumn"),
            WeaponName::Nullify => write!(f, "Nullify"),
            WeaponName::LunasHowl => write!(f, "Luna's Howl"),
            WeaponName::Parasite => write!(f, "Parasite"),
            WeaponName::TheWardcliffCoil => write!(f, "The Wardcliff Coil"),
            WeaponName::Gjallarhorn => write!(f, "Gjallarhorn"),
            WeaponName::Xenophage => write!(f, "Xenophage"),
            WeaponName::Thunderlord => write!(f, "Thunderlord"),
            WeaponName::OutbreakPerfected => write!(f, "Outbreak Perfected"),
            WeaponName::FinalWarning => write!(f, "Final Warning"),
            WeaponName::IKELOSSGV103 => write!(f, "IKELOS_SG_v1.0.3"),
            WeaponName::ProphetOfDoom => write!(f, "Prophet of Doom"),
            WeaponName::DeadWeight => write!(f, "Dead Weight"),
            WeaponName::BassoOstinato => write!(f, "Basso Ostinato"),
            WeaponName::Heritage => write!(f, "Heritage"),
            WeaponName::ImperialDecree => write!(f, "Imperial Decree"),
            WeaponName::UntilItsReturn => write!(f, "Until It's Return"),
            WeaponName::Someday => write!(f, "Someday"),
            WeaponName::OmniscientEye => write!(f, "Omniscient Eye"),
            WeaponName::IKELOSSRV103 => write!(f, "IKELOS_SR_v1.0.3"),
            WeaponName::TwilightOath => write!(f, "Twilight Oath"),
            WeaponName::DistantTumulus => write!(f, "Distant Tumulus"),
            WeaponName::ScatterSignal => write!(f, "Scatter Signal"),
            WeaponName::Riptide => write!(f, "Riptide"),
            WeaponName::NoxPerennialV => write!(f, "Nox Perennial V"),
            WeaponName::ZealotsReward => write!(f, "Zealot's Reward"),
            WeaponName::RoyalExecutioner => write!(f, "Royal Executioner"),
            WeaponName::TecheunForce => write!(f, "Techeun Force"),
            WeaponName::AxialLacuna => write!(f, "Axial Lacuna"),
            WeaponName::TheMountaintop => write!(f, "The Mountaintop"),
            WeaponName::Alethonym => write!(f, "Alethonym"),
            WeaponName::MartyrsRetribution => write!(f, "Martyr's Retribution"),
            WeaponName::Forbearance => write!(f, "Forbearance"),
            WeaponName::TuskOfTheBoar => write!(f, "Tusk of the Boar"),
            WeaponName::NewPacificEpitaph => write!(f, "New Pacific Epitaph"),
            WeaponName::TheMilitiasBirthright => write!(f, "The Militia's Birthright"),
            WeaponName::IgnitionCode => write!(f, "Ignition Code"),
            WeaponName::PardonOurDust => write!(f, "Pardon Our Dust"),
            WeaponName::Wilderflight => write!(f, "Wilderflight"),
            WeaponName::WildStyle => write!(f, "Wild Style"),
            WeaponName::EmptyVessel => write!(f, "Empty Vessel"),
            WeaponName::ForthcomingDeviance => write!(f, "Forthcoming Deviance"),
            WeaponName::NezarecsWhisper => write!(f, "Nezarec's Whisper"),
            WeaponName::TheEnigma => write!(f, "The Enigma"),
            WeaponName::Chronophage => write!(f, "Chronophage"),
            WeaponName::PathOfLeastResistance => write!(f, "Path of Least Resistance"),
            WeaponName::RetracedPath => write!(f, "Retraced Path"),
            WeaponName::SummumBonum => write!(f, "Summum Bonum"),
            WeaponName::GeodeticHSm => write!(f, "Geodetic HSm"),
            WeaponName::Bequest => write!(f, "Bequest"),
            WeaponName::IllOmen => write!(f, "Ill Omen"),
            WeaponName::TomorrowsAnswer => write!(f, "Tomorrow's Answer"),
            WeaponName::ApexPredator => write!(f, "Apex Predator"),
            WeaponName::CruxTerminationIV => write!(f, "Crux Termination IV"),
            WeaponName::ColdComfort => write!(f, "Cold Comfort"),
            WeaponName::BraytechOsprey => write!(f, "Braytech Osprey"),
            WeaponName::FaithKeeper => write!(f, "Faithkeeper"),
            WeaponName::Semiotician => write!(f, "Semiotician"),
            WeaponName::Scintillation => write!(f, "Scintillation"),
            WeaponName::DoomedPetitioner => write!(f, "Doomed Petitioner"),
            WeaponName::Cataclysmic => write!(f, "Cataclysmic"),
            WeaponName::BriarsContempt => write!(f, "Briar's Contempt"),
            WeaponName::MultimachCCX => write!(f, "Multimach CCX"),
            WeaponName::Imminence => write!(f, "Imminence"),
            WeaponName::MidnightCoup => write!(f, "Midnight Coup"),
            WeaponName::AccruedRedemption => write!(f, "Accrued Redemption"),
            WeaponName::WardensLaw => write!(f, "Warden's Law"),
            WeaponName::YesterdaysQuestion => write!(f, "Yesterday's Question"),
            WeaponName::MaahesHC4 => write!(f, "Maahes HC4"),
            WeaponName::Khvostov7G0X => write!(f, "Khvostov 7G-0X"),
            WeaponName::Necrochasm => write!(f, "Necrochasm"),
            WeaponName::BadJuju => write!(f, "Bad Juju"),
            WeaponName::TheHuckleberry => write!(f, "The Huckleberry"),
            WeaponName::TheFourthHorseman => write!(f, "The Fourth Horseman"),
            WeaponName::ChoirOfOne => write!(f, "Choir of One"),
            WeaponName::StillHunt => write!(f, "Still Hunt"),
            WeaponName::IzanagisBurden => write!(f, "Izanagi's Burden"),
            WeaponName::BuriedBloodline => write!(f, "Buried Bloodline"),
            WeaponName::RedDeathReformed => write!(f, "Red Death Reformed"),
            WeaponName::Euphony => write!(f, "Euphony"),
            WeaponName::Cloudstrike => write!(f, "Cloudstrike"),
        }
    }
}
