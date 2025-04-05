use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Represents a game time manager that handles offline progress calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTime {
    /// Last time the game state was saved/updated
    last_update: u64,
    /// When the player first started playing
    created_at: u64,
}

impl GameTime {
    /// Creates a new GameTime instance
    pub fn new() -> Self {
        let now = Self::current_timestamp();
        Self {
            last_update: now,
            created_at: now,
        }
    }

    /// Get the current Unix timestamp in seconds
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }

    /// Updates the last_update timestamp to now
    pub fn update(&mut self) {
        self.last_update = Self::current_timestamp();
    }

    /// Calculate how many seconds have passed since the last update
    pub fn get_offline_time(&self) -> u64 {
        let now = Self::current_timestamp();
        now.saturating_sub(self.last_update)
    }

    /// Calculate resource accumulation based on a rate per second
    pub fn calculate_offline_progress(&self, rate_per_second: f64) -> f64 {
        let offline_seconds = self.get_offline_time() as f64;
        rate_per_second * offline_seconds
    }

    /// Get the total time played (including offline time) in seconds
    pub fn total_time_played(&self) -> u64 {
        Self::current_timestamp().saturating_sub(self.created_at)
    }

    /// Get the last update timestamp
    pub fn last_update(&self) -> u64 {
        self.last_update
    }

    /// Get the creation timestamp
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
}

/// Represents a resource that accumulates over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBasedResource {
    /// Current amount of the resource
    amount: f64,
    /// Rate of accumulation per second
    rate_per_second: f64,
}

impl TimeBasedResource {
    /// Creates a new TimeBasedResource with an initial amount and rate
    pub fn new(initial_amount: f64, rate_per_second: f64) -> Self {
        Self {
            amount: initial_amount,
            rate_per_second,
        }
    }

    /// Update the resource amount based on elapsed time
    pub fn update(&mut self, game_time: &GameTime) {
        let accumulated = game_time.calculate_offline_progress(self.rate_per_second);
        self.amount += accumulated;
    }

    /// Get the current amount of the resource
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// Get the rate of accumulation per second
    pub fn rate_per_second(&self) -> f64 {
        self.rate_per_second
    }

    /// Set a new rate of accumulation per second
    pub fn set_rate(&mut self, new_rate: f64) {
        self.rate_per_second = new_rate;
    }

    /// Add or subtract from the current amount
    pub fn adjust_amount(&mut self, delta: f64) {
        self.amount += delta;
    }
} 