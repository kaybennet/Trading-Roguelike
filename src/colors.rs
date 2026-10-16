use bevy::prelude::Color;

use crate::rarity::Rarity;
use crate::stocks::Ticker;

pub const DEBUG_BG_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const SCORE_BOX_COLOR: Color = Color::srgb(0.1, 0.12, 0.17);
pub const PANEL_COLOR: Color = Color::srgb(0.16, 0.2, 0.28);
pub const BG_TOP_COLOR: Color = Color::srgb(0.07, 0.09, 0.14);
pub const BG_BOTTOM_COLOR: Color = Color::srgb(0.03, 0.04, 0.07);
pub const BORDER_COLOR: Color = Color::srgb(0.24, 0.28, 0.37);
pub const ACCENT_COLOR: Color = Color::srgb(0.3, 0.82, 0.65);
pub const MUTED_COLOR: Color = Color::srgb(0.6, 0.65, 0.74);
pub const STOCK_UP_COLOR: Color = Color::srgb(0.4, 0.9, 0.5);
pub const STOCK_DOWN_COLOR: Color = Color::srgb(0.95, 0.4, 0.4);
pub const BUTTON_COLOR: Color = Color::srgb(0.17, 0.5, 0.42);
pub const BUTTON_HOVER_COLOR: Color = Color::srgb(0.22, 0.62, 0.52);
pub const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.12, 0.4, 0.33);
pub const GAME_OVER_BG_COLOR: Color = Color::srgba(0.1, 0.02, 0.03, 0.92);
pub const TRIGGER_FLASH_COLOR: Color = Color::srgb(0.6, 0.62, 0.66);
pub const MODIFIER_COLOR: Color = Color::srgb(0.4, 0.28, 0.5);

/// Every stock has its own colour, used for its name and price chart.
const STOCK_COLORS: [(&str, Color); 12] = [
    ("ACME", Color::srgb(1.0, 0.55, 0.2)),
    ("BLNK", Color::srgb(0.35, 0.8, 1.0)),
    ("CRPT", Color::srgb(0.98, 0.88, 0.3)),
    ("DYNO", Color::srgb(0.95, 0.45, 0.7)),
    ("ECHO", Color::srgb(0.7, 0.55, 1.0)),
    ("FLUX", Color::srgb(0.7, 0.92, 0.3)),
    ("GRID", Color::srgb(0.4, 0.5, 1.0)),
    ("HELX", Color::srgb(1.0, 0.42, 0.42)),
    ("IONX", Color::srgb(0.2, 0.8, 0.65)),
    ("JOLT", Color::srgb(0.85, 0.35, 0.95)),
    ("KRNL", Color::srgb(0.85, 0.62, 0.45)),
    ("LUMA", Color::srgb(0.8, 0.85, 1.0)),
];

pub fn stock_color(ticker: Ticker) -> Color {
    STOCK_COLORS.iter().find(|(s, _)| *s == ticker.name()).map_or(MUTED_COLOR, |&(_, color)| color)
}

pub fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Common => Color::srgb(0.7, 0.72, 0.76),
        Rarity::Uncommon => Color::srgb(0.35, 0.8, 0.45),
        Rarity::Rare => Color::srgb(0.3, 0.6, 1.0),
        Rarity::Epic => Color::srgb(0.7, 0.4, 0.95),
        Rarity::Legendary => Color::srgb(1.0, 0.65, 0.2),
        Rarity::Mythic => Color::srgb(1.0, 0.3, 0.45),
    }
}
