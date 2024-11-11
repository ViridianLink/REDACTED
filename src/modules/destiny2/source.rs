use std::fmt;

#[derive(Clone)]
pub enum Source {
    VespersHost,
    IronBanner,
    Season24,
    Quest,
    Kiosk,
    TheWhisper,
    PaleHeart,
    ExoticQuest,
    World,
    SalvationsEdge,
    LastWish,
    Nightfall,
    ZeroHour,
    OperationSeraphsShield,
    DeepStoneCrypt,
    GardenOfSalvation,
    Season23,
    Onslaught,
    Season25,
    SpireOfTheWatcher,
    TrialsOfOsiris,
    WarlordsRuin,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::VespersHost => write!(f, "Vesper's Host"),
            Source::IronBanner => write!(f, "Iron Banner"),
            Source::Season24 => write!(f, "Season 24"),
            Source::Quest => write!(f, "Quest"),
            Source::Kiosk => write!(f, "Kiosk"),
            Source::TheWhisper => write!(f, "The Whisper"),
            Source::PaleHeart => write!(f, "Pale Heart"),
            Source::ExoticQuest => write!(f, "Exotic Quest"),
            Source::World => write!(f, "World"),
            Source::SalvationsEdge => write!(f, "Salvation's Edge"),
            Source::LastWish => write!(f, "Last Wish"),
            Source::Nightfall => write!(f, "Nightfall"),
            Source::ZeroHour => write!(f, "Zero Hour"),
            Source::OperationSeraphsShield => write!(f, "Operation: Seraph's Shield"),
            Source::DeepStoneCrypt => write!(f, "Deep Stone Crypt"),
            Source::GardenOfSalvation => write!(f, "Garden of Salvation"),
            Source::Season23 => write!(f, "Season 23"),
            Source::Onslaught => write!(f, "Onslaught"),
            Source::Season25 => write!(f, "Season 25"),
            Source::SpireOfTheWatcher => write!(f, "Spire of the Watcher"),
            Source::TrialsOfOsiris => write!(f, "Trials of Osiris"),
            Source::WarlordsRuin => write!(f, "Warlord's Ruin"),
        }
    }
}
