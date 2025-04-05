use crate::consts::minerals::{LusterType, Colors, Transparency, Cleavage, OpticalPhenomena, RockType};
use crate::consts::minerals::{MIN_SPECIFIC_GRAVITY, MAX_SPECIFIC_GRAVITY, MIN_MOHS_HARDNESS, MAX_MOHS_HARDNESS};
use rand::Rng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use strum::IntoEnumIterator;

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
    rng: Box<dyn rand::RngCore>,
}

impl RockFactory {
    pub fn new() -> Self {
        Self {
            rng: Box::new(rand::thread_rng()),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: Box::new(StdRng::seed_from_u64(seed)),
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
        RockType::iter().choose(&mut *self.rng).unwrap()
    }

    fn random_luster(&mut self) -> LusterType {
        LusterType::iter().choose(&mut *self.rng).unwrap()
    }

    fn random_color(&mut self) -> Colors {
        Colors::iter().choose(&mut *self.rng).unwrap()
    }

    fn random_transparency(&mut self) -> Transparency {
        Transparency::iter().choose(&mut *self.rng).unwrap()
    }

    fn random_cleavage(&mut self) -> Option<Cleavage> {
        if self.rng.gen_bool(0.7) {  // 70% chance to have cleavage
            Some(Cleavage::iter().choose(&mut *self.rng).unwrap())
        } else {
            None
        }
    }

    fn random_optical_phenomena(&mut self) -> Option<OpticalPhenomena> {
        if self.rng.gen_bool(0.3) {  // 30% chance to have optical phenomena
            Some(OpticalPhenomena::iter().choose(&mut *self.rng).unwrap())
        } else {
            None
        }
    }

    pub fn calculate_value(&self, rock: &Rock) -> f32 {
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
    fn test_rock_generation_with_seed() {
        let mut factory1 = RockFactory::with_seed(42);
        let mut factory2 = RockFactory::with_seed(42);
        
        let rock1 = factory1.generate_rock();
        let rock2 = factory2.generate_rock();
        
        assert_eq!(rock1.rock_type, rock2.rock_type);
        assert_eq!(rock1.size, rock2.size);
        assert_eq!(rock1.quality, rock2.quality);
        assert_eq!(rock1.lusters, rock2.lusters);
        assert_eq!(rock1.color, rock2.color);
        assert_eq!(rock1.transparency, rock2.transparency);
        assert_eq!(rock1.cleavage, rock2.cleavage);
        assert_eq!(rock1.hardness, rock2.hardness);
        assert_eq!(rock1.specific_gravity, rock2.specific_gravity);
        assert_eq!(rock1.optical_phenomena, rock2.optical_phenomena);
        assert_eq!(rock1.impurities, rock2.impurities);
        assert_eq!(rock1.value, rock2.value);
    }
}
