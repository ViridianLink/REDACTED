use std::fmt;

#[derive(Clone)]
pub enum Role {
    OrbGenGL,
    KineticRocketSidearm,
    EnergyRocketSidearm,
    DpsHgl,
    ExoticEnergyPrimary,
    ExoticConsistentHeavy,
    ExoticTotalHeavy,
    ExoticDebuff,
    SupportAuto,
    ExoticDRAddClear,
    Kinetic12pShotgun,
    WeakenOnDemand,
    HitscanOverloadStun,
    KineticDamageSniper,
    TranscendenceGL,
    MachineGun,
    MovementSword,
    EnergyPrimary,
    ExoticBurstHeavy,
    ExoticAddClearHeavy,
    AmmolessDamage,
    Energy12pShotgun,
    KineticBurstShotgun,
    EnergyDamageSniper,
    KineticDamageFusion,
    EnergyDamageFusion,
    MovementGL,
    EnergyAddClearWave,
    KineticAddClearWave,
    KineticBlindingGL,
    EnergyBlindingGL,
    Glaive,
    SpecialShootToLoot,
    DPSSword,
    DPSRocket,
    AddClearRocket,
    Linear,
    KineticPrimary,
    KineticLPHC,
    EnergyLPHC,
    ExoticKineticPrimary,
    ExoticBurstSpecial,
    ExoticSurvivability,
    ExoticTotalSpecial,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::OrbGenGL => write!(f, "Orb Gen Grenade Launcher"),
            Role::KineticRocketSidearm => write!(f, "Kinetic Rocket Sidearm"),
            Role::EnergyRocketSidearm => write!(f, "Energy Rocket Sidearm"),
            Role::DpsHgl => write!(f, "DPS Heavy Grenade Launcher"),
            Role::ExoticEnergyPrimary => write!(f, "Exotic Energy Primary"),
            Role::ExoticConsistentHeavy => write!(f, "Exotic Consistent Heavy"),
            Role::ExoticTotalHeavy => write!(f, "Exotic Total Heavy"),
            Role::ExoticDebuff => write!(f, "Exotic Debuff"),
            Role::SupportAuto => write!(f, "Support Auto"),
            Role::ExoticDRAddClear => write!(f, "Exotic Damage Resistant Add Clear"),
            Role::Kinetic12pShotgun => write!(f, "Kinetic One-Two Punch Shotgun"),
            Role::WeakenOnDemand => write!(f, "Weaken on Demand"),
            Role::HitscanOverloadStun => write!(f, "Hitscan Overload Stun"),
            Role::KineticDamageSniper => write!(f, "Kinetic Damage Sniper"),
            Role::TranscendenceGL => write!(f, "Transcendence Grenade Launcher"),
            Role::MachineGun => write!(f, "Machine Gun"),
            Role::MovementSword => write!(f, "Movement Sword"),
            Role::EnergyPrimary => write!(f, "Energy Primary"),
            Role::ExoticBurstHeavy => write!(f, "Exotic Burst Heavy"),
            Role::ExoticAddClearHeavy => write!(f, "Exotic Add Clear Heavy"),
            Role::AmmolessDamage => write!(f, "Ammoless Damage"),
            Role::Energy12pShotgun => write!(f, "Energy One-Two Punch Shotgun"),
            Role::KineticBurstShotgun => write!(f, "Kinetic Burst Shotgun"),
            Role::EnergyDamageSniper => write!(f, "Energy Damage Sniper"),
            Role::KineticDamageFusion => write!(f, "Kinetic Damage Fusion"),
            Role::EnergyDamageFusion => write!(f, "Energy Damage Fusion"),
            Role::MovementGL => write!(f, "Movement Grenade Launcher"),
            Role::EnergyAddClearWave => write!(f, "Energy Add Clear Wave"),
            Role::KineticAddClearWave => write!(f, "Kinetic Add Clear Wave"),
            Role::KineticBlindingGL => write!(f, "Kinetic Blinding Grenade Launcher"),
            Role::EnergyBlindingGL => write!(f, "Energy Blinding Grenade Launcher"),
            Role::Glaive => write!(f, "Glaive"),
            Role::SpecialShootToLoot => write!(f, "Special Shoot to Loot"),
            Role::DPSSword => write!(f, "DPS Sword"),
            Role::DPSRocket => write!(f, "DPS Rocket"),
            Role::AddClearRocket => write!(f, "Add Clear Rocket"),
            Role::Linear => write!(f, "Linear"),
            Role::KineticPrimary => write!(f, "Kinetic Primary"),
            Role::KineticLPHC => write!(f, "Kinetic Linear Fusion Heavy"),
            Role::EnergyLPHC => write!(f, "Energy Linear Fusion Heavy"),
            Role::ExoticKineticPrimary => write!(f, "Exotic Kinetic Primary"),
            Role::ExoticBurstSpecial => write!(f, "Exotic Burst Special"),
            Role::ExoticSurvivability => write!(f, "Exotic Survivability"),
            Role::ExoticTotalSpecial => write!(f, "Exotic Total Special"),
        }
    }
}
