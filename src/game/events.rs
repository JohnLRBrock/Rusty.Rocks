use std::ops::RangeInclusive;
use rand::Rng;
use serde::{Deserialize, Serialize};
use super::time::GameTime;

/// Represents a scheduled game event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    /// Type of the event
    event_type: String,
    /// When the event should trigger (Unix timestamp)
    trigger_time: u64,
    /// Whether the event has been triggered
    triggered: bool,
}

impl GameEvent {
    /// Creates a new event scheduled to trigger after the specified number of seconds
    pub fn new(event_type: &str, seconds_from_now: u64) -> Self {
        let current_time = GameTime::current_timestamp();
        Self {
            event_type: event_type.to_string(),
            trigger_time: current_time + seconds_from_now,
            triggered: false,
        }
    }

    /// Get the event type
    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    /// Get the trigger time
    pub fn trigger_time(&self) -> u64 {
        self.trigger_time
    }

    /// Check if the event has been triggered
    pub fn is_triggered(&self) -> bool {
        self.triggered
    }

    /// Mark the event as triggered
    fn trigger(&mut self) {
        self.triggered = true;
    }

    /// Check if the event should trigger based on the current game time
    fn should_trigger(&self, game_time: &GameTime) -> bool {
        !self.triggered && game_time.current_timestamp() >= self.trigger_time
    }
}

/// Manages and processes scheduled events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventScheduler {
    pending_events: Vec<GameEvent>,
}

impl EventScheduler {
    /// Creates a new event scheduler
    pub fn new() -> Self {
        Self {
            pending_events: Vec::new(),
        }
    }

    /// Schedule a new event to occur at a random time within the specified range (in seconds)
    pub fn schedule_event(&mut self, event_type: &str, time_range: RangeInclusive<u64>) {
        let mut rng = rand::thread_rng();
        let seconds = rng.gen_range(time_range);
        let event = GameEvent::new(event_type, seconds);
        
        // Insert maintaining sort order by trigger time
        let insert_pos = self.pending_events
            .binary_search_by_key(&event.trigger_time, |e| e.trigger_time)
            .unwrap_or_else(|pos| pos);
        
        self.pending_events.insert(insert_pos, event);
    }

    /// Process all events that should trigger based on the current game time
    /// Returns a vector of triggered events
    pub fn process_events(&mut self, game_time: &GameTime) -> Vec<GameEvent> {
        let mut triggered = Vec::new();
        let mut i = 0;
        
        while i < self.pending_events.len() {
            if self.pending_events[i].should_trigger(game_time) {
                self.pending_events[i].trigger();
                triggered.push(self.pending_events.remove(i));
            } else {
                i += 1;
            }
        }
        
        triggered
    }

    /// Get a reference to all pending events
    pub fn pending_events(&self) -> &[GameEvent] {
        &self.pending_events
    }
}

impl Default for EventScheduler {
    fn default() -> Self {
        Self::new()
    }
}
