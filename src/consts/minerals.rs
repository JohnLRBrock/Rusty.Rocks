#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum OpticalPhenomena {
    Asterism,
    Chatoyance,
    ColorChange,
    Iridescence,
    Schiller
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Transparency {
    Opaque,
    Transparent,
    Translucent,
}

#[derive(Debug, Clone)]
pub enum Cleavage {
    Basal,
    Prismatic,
    Cubic,
    Rhombohedral,
    Octahedral,
    Dodecahedral
}

pub const MIN_SPECIFIC_GRAVITY: f32 = 2.0;  // g/cm³, typical minimum for common minerals
pub const MAX_SPECIFIC_GRAVITY: f32 = 7.5;  // g/cm³, typical maximum for common minerals

pub const MIN_MOHS_HARDNESS: f32 = 1.0;     // Talc
pub const MAX_MOHS_HARDNESS: f32 = 10.0;    // Diamond
