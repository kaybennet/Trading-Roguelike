use strum::{EnumIter, IntoStaticStr};

/// How rare a joker or modifier is; decides how often the shop offers it.
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, IntoStaticStr)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl Rarity {
    pub fn name(self) -> &'static str {
        self.into()
    }

    /// Relative chance of being drawn for the shop.
    pub fn draw_weight(self) -> u32 {
        match self {
            Self::Common => 50,
            Self::Uncommon => 30,
            Self::Rare => 15,
            Self::Epic => 8,
            Self::Legendary => 3,
            Self::Mythic => 1,
        }
    }
}
