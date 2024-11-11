use std::fmt;

use serenity::all::Colour;

#[derive(Clone)]

pub enum Priority {
    High,
    Medium,
    Low,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Medium => write!(f, "Medium"),
            Priority::Low => write!(f, "Low"),
        }
    }
}

impl From<&Priority> for Colour {
    fn from(priority: &Priority) -> Self {
        match priority {
            Priority::High => Colour(0xff0000),
            Priority::Medium => Colour(0xff8000),
            Priority::Low => Colour(0xffff00),
        }
    }
}
