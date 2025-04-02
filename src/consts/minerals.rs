use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum RockType {
    Sedimentary,
    Igneous,
    Metamorphic,
}

pub const MIN_MOHS_HARDNESS: f32 = 1.0;
pub const MAX_MOHS_HARDNESS: f32 = 10.0;
pub const MIN_SPECIFIC_GRAVITY: f32 = 2.0;
pub const MAX_SPECIFIC_GRAVITY: f32 = 7.0;

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum LusterType {
    Adamantine,
    Dull,
    Greasy,
    Metallic,
    Pearly,
    Resinous,
    Silky,
    Submetallic,
    Vitreous,
    Waxy,
}

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum Colors {
    Black,
    Blue,
    Brown,
    Green,
    Red,
    White,
    Yellow,
    Orange,
    Purple,
    Pink,
    Gray,
    Gold,
    Silver,
    Violet,
}

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum Transparency {
    Opaque,
    Transparent,
    Translucent,
}

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum Cleavage {
    Basal,
    Prismatic,
    Cubic,
    Rhombohedral,
    Octahedral,
    Dodecahedral,
}

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum OpticalPhenomena {
    Asterism,
    Chatoyance,
    ColorChange,
    Iridescence,
    Schiller,
}
