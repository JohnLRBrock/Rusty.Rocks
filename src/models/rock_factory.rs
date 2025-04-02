use crate::consts::minerals::{LusterType, Colors, Transparency, Cleavage, OpticalPhenomena};
use crate::consts::minerals::{MIN_SPECIFIC_GRAVITY, MAX_SPECIFIC_GRAVITY, MIN_MOHS_HARDNESS, MAX_MOHS_HARDNESS};
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub enum RockType {
    Sedimentary,
    Igneous,
    Metamorphic,
}

#[derive(Debug, Clone)]
pub struct Rock {
    pub rock_type: RockType,
    pub size: f32,        // 0.1 to 10.0
    pub quality: f32,     // 0.0 to 1.0
    pub lusters: LusterType,
    pub color: Colors,
    pub transparency: Transparency,
    pub cleavage: Option<Cleavage>,
    pub hardness: f32,    // MIN_MOHS_HARDNESS to MAX_MOHS_HARDNESS
    pub specific_gravity: f32,  // MIN_SPECIFIC_GRAVITY to MAX_SPECIFIC_GRAVITY
    pub optical_phenomena: Option<OpticalPhenomena>,
    pub impurities: f32,  // 0.0 to 1.0 (lower is better)
    pub value: f32,
}

pub struct RockFactory {
    rng: rand::rngs::ThreadRng,
}

impl RockFactory {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_rock(&mut self) -> Rock {
        let rock_type = self.random_rock_type();
        let size = self.rng.gen_range(0.1..=10.0);
        let quality = self.rng.gen_range(0.0..=1.0);
        let lusters = self.random_luster();
        let color = self.random_color();
        let transparency = self.random_transparency();
        let cleavage = self.random_cleavage();
        let hardness = self.rng.gen_range(MIN_MOHS_HARDNESS..=MAX_MOHS_HARDNESS);
        let specific_gravity = self.rng.gen_range(MIN_SPECIFIC_GRAVITY..=MAX_SPECIFIC_GRAVITY);
        let optical_phenomena = self.random_optical_phenomena();
        let impurities = self.rng.gen_range(0.0..=1.0);

        let rock = Rock {
            rock_type,
            size,
            quality,
            lusters,
            color,
            transparency,
            cleavage,
            hardness,
            specific_gravity,
            optical_phenomena,
            impurities,
            value: 0.0, // Temporary value, will be calculated below
        };

        let value = self.calculate_value(&rock);
        Rock { value, ..rock }
    }

    fn random_rock_type(&mut self) -> RockType {
        let types = [RockType::Sedimentary, RockType::Igneous, RockType::Metamorphic];
        types.choose(&mut self.rng).unwrap().clone()
    }

    fn random_luster(&mut self) -> LusterType {
        let lusters = [
            LusterType::Adamantine,
            LusterType::Dull,
            LusterType::Greasy,
            LusterType::Metallic,
            LusterType::Pearly,
            LusterType::Resinous,
            LusterType::Silky,
            LusterType::Submetallic,
            LusterType::Vitreous,
            LusterType::Waxy,
        ];
        lusters.choose(&mut self.rng).unwrap().clone()
    }

    fn random_color(&mut self) -> Colors {
        let colors = [
            Colors::Black, Colors::Blue, Colors::Brown, Colors::Green,
            Colors::Red, Colors::White, Colors::Yellow, Colors::Orange,
            Colors::Purple, Colors::Pink, Colors::Gray, Colors::Gold,
            Colors::Silver, Colors::Violet
        ];
        colors.choose(&mut self.rng).unwrap().clone()
    }

    fn random_transparency(&mut self) -> Transparency {
        let transparencies = [
            Transparency::Opaque,
            Transparency::Transparent,
            Transparency::Translucent,
        ];
        transparencies.choose(&mut self.rng).unwrap().clone()
    }

    fn random_cleavage(&mut self) -> Option<Cleavage> {
        if self.rng.gen_bool(0.7) {  // 70% chance to have cleavage
            let cleavages = [
                Cleavage::Basal,
                Cleavage::Prismatic,
                Cleavage::Cubic,
                Cleavage::Rhombohedral,
                Cleavage::Octahedral,
                Cleavage::Dodecahedral,
            ];
            Some(cleavages.choose(&mut self.rng).unwrap().clone())
        } else {
            None
        }
    }

    fn random_optical_phenomena(&mut self) -> Option<OpticalPhenomena> {
        if self.rng.gen_bool(0.3) {  // 30% chance to have optical phenomena
            let phenomena = [
                OpticalPhenomena::Asterism,
                OpticalPhenomena::Chatoyance,
                OpticalPhenomena::ColorChange,
                OpticalPhenomena::Iridescence,
                OpticalPhenomena::Schiller,
            ];
            Some(phenomena.choose(&mut self.rng).unwrap().clone())
        } else {
            None
        }
    }

    fn calculate_value(&self, rock: &Rock) -> f32 {
        let base_value = 10.0;

        let quality_multiplier = 1.0 + rock.quality;
        let size_multiplier = (rock.size / 2.0).min(3.0);
        let hardness_bonus = (rock.hardness / MAX_MOHS_HARDNESS) * 0.5;
        let phenomena_bonus = if rock.optical_phenomena.is_some() { 1.5 } else { 1.0 };
        let impurity_penalty = 1.0 - (rock.impurities * 0.3);

        base_value * quality_multiplier * size_multiplier * (1.0 + hardness_bonus) 
            * phenomena_bonus * impurity_penalty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_generation() {
        let mut factory = RockFactory::new();
        let rock = factory.generate_rock();

        assert!(rock.size >= 0.1 && rock.size <= 10.0);
        assert!(rock.quality >= 0.0 && rock.quality <= 1.0);
        assert!(rock.hardness >= MIN_MOHS_HARDNESS && rock.hardness <= MAX_MOHS_HARDNESS);
        assert!(rock.specific_gravity >= MIN_SPECIFIC_GRAVITY && rock.specific_gravity <= MAX_SPECIFIC_GRAVITY);
        assert!(rock.impurities >= 0.0 && rock.impurities <= 1.0);
        assert!(rock.value > 0.0);
    }

    #[test]
    fn test_value_calculation() {
        let factory = RockFactory::new();
        let rock = Rock {
            rock_type: RockType::Igneous,
            size: 1.0,
            quality: 1.0,
            lusters: LusterType::Vitreous,
            color: Colors::Purple,
            transparency: Transparency::Transparent,
            cleavage: Some(Cleavage::Rhombohedral),
            hardness: MAX_MOHS_HARDNESS,
            specific_gravity: (MIN_SPECIFIC_GRAVITY + MAX_SPECIFIC_GRAVITY) / 2.0,
            optical_phenomena: Some(OpticalPhenomena::Asterism),
            impurities: 0.0,
            value: 0.0,
        };

        let value = factory.calculate_value(&rock);
        assert!(value > 10.0); // Base value should be increased by perfect attributes
    }
}
