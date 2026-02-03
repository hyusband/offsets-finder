use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameVariant {
    FreeFire,
    FreeFireMax,
    FreeFireTela,
}

impl GameVariant {
    pub fn name(&self) -> &str {
        match self {
            GameVariant::FreeFire => "Free Fire",
            GameVariant::FreeFireMax => "Free Fire MAX",
            GameVariant::FreeFireTela => "Free Fire TELA",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Target {
    Fixed { name: String, hex: String },
    Pattern { name: String, pattern: String },
    Separator,
}

impl Target {
    pub fn new_fixed(name: &str, hex: &str) -> Self {
        Target::Fixed {
            name: name.to_string(),
            hex: hex.to_string(),
        }
    }

    pub fn new_pattern(name: &str, pattern: &str) -> Self {
        Target::Pattern {
            name: name.to_string(),
            pattern: pattern.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> Option<&str> {
        match self {
            Target::Fixed { name, .. } => Some(name),
            Target::Pattern { name, .. } => Some(name),
            Target::Separator => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetResult {
    pub name: String,
    pub offset: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetCategory {
    Core,
    Player,
    Camera,
    Weapon,
    Silent,
    Collision,
    Attributes,
    Bot,
    Skeleton,
}

impl OffsetCategory {
    pub fn name(&self) -> &str {
        match self {
            OffsetCategory::Core => "Core",
            OffsetCategory::Player => "Player",
            OffsetCategory::Camera => "Camera",
            OffsetCategory::Weapon => "Weapon",
            OffsetCategory::Silent => "Silent Aim",
            OffsetCategory::Collision => "Collision",
            OffsetCategory::Attributes => "Attributes",
            OffsetCategory::Bot => "Bot Detection",
            OffsetCategory::Skeleton => "Skeleton/Bones",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    CppHeader,
    RustModule,
    PlainText,
}

impl ExportFormat {
    pub fn extension(&self) -> &str {
        match self {
            ExportFormat::Json => "json",
            ExportFormat::CppHeader => "hpp",
            ExportFormat::RustModule => "rs",
            ExportFormat::PlainText => "txt",
        }
    }
}
