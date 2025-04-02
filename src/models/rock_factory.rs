use crate::consts::minerals::{LusterType, Colors, Transparency, Cleavage, OpticalPhenomena};
use crate::consts::minerals::{MIN_SPECIFIC_GRAVITY, MAX_SPECIFIC_GRAVITY, MIN_MOHS_HARDNESS, MAX_MOHS_HARDNESS};
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
use crate::consts::minerals::RockType;

#[derive(Debug, Clone)]
pub struct Rock {
    pub rock_type: RockType,    // Geological classification
    pub size: f32,        // MIN_ROCK_SIZE to MAX_ROCK_SIZE
    pub quality: f32,     // MIN_QUALITY to MAX_QUALITY
    pub lusters: LusterType,
    pub color: Colors,
    pub transparency: Transparency,
    pub cleavage: Cleavage,
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
        let rock_type = RockType::all().collect().choose(&mut rand::rng()).unwrap().clone();
        let size = self.rng.gen_range(0.1..=10.0);
        let grade = self.rng.gen_range(0.0..=1.0);
        let lusters = LusterType::all().collect().choose(&mut rand::rng()).unwrap().clone();
        let color = Colors::all().collect().choose(&mut rand::rng()).unwrap().clone();
        let transparency = Transparency::all().collect().choose(&mut rand::rng()).unwrap().clone();
        let cleavage = Cleavage::all().collect().choose(&mut rand::rng()).unwrap().clone();
        let hardness = self.rng.gen_range(MIN_MOHS_HARDNESS..=MAX_MOHS_HARDNESS);
        let specific_gravity = self.rng.gen_range(MIN_SPECIFIC_GRAVITY..=MAX_SPECIFIC_GRAVITY);
        let optical_phenomena = self.optical_phenomena();
        let impurities = self.rng.gen_range(0.0..=1.0);

        let rock = Rock {
            rock_type,
            size,
            grade,
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

    fn optical_phenomena(&mut self) -> Option<OpticalPhenomena> {
        if self.rng.gen_ratio(1, 100) {
            Some(OpticalPhenomena::all().collect().choose(&mut self.rng).unwrap().clone())
        } else {
            None
        }
    }

    fn calculate_value(&self, rock: &Rock) -> f32 {
        let base_value = 1.0;

        let quality_multiplier = 1.0 + rock.quality;
        let size_multiplier = (rock.size / 2.0).min(3.0);
        let impurity_penalty = 1.0 - (rock.impurities * 0.3);

        base_value * quality_multiplier * size_multiplier * (1.0 + luster_bonus) 
            * (1.0 + hardness_bonus) * phenomena_bonus * impurity_penalty
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
        assert!(rock.grade >= 0.0 && rock.grade <= 1.0);
        assert!(!rock.lusters.is_empty() && rock.lusters.len() <= 3);
        assert!(rock.hardness >= MIN_MOHS_HARDNESS && rock.hardness <= MAX_MOHS_HARDNESS);
        assert!(rock.specific_gravity >= MIN_SPECIFIC_GRAVITY && rock.specific_gravity <= MAX_SPECIFIC_GRAVITY);
        assert!(rock.impurities >= 0.0 && rock.impurities <= 1.0);
        assert!(rock.value > 0.0);
        
        // Test rock type assignment
        match rock.mineral_type {
            MineralType::Obsidian | MineralType::Granite => assert_eq!(rock.rock_type, RockType::Igneous),
            MineralType::Marble | MineralType::Slate => assert_eq!(rock.rock_type, RockType::Metamorphic),
            _ => {} // Quartz and Amethyst can be any type
        }
    }

    #[test]
    fn test_value_calculation() {
        let factory = RockFactory::new();
        let rock = Rock {
            mineral_type: MineralType::Amethyst,
            rock_type: RockType::Igneous,
            size: 1.0,
            quality: 1.0,
            lusters: vec![LusterType::Vitreous, LusterType::Adamantine],
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
        assert!(value > 25.0); // Base value should be increased by perfect attributes
    }
}
