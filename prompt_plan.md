## Phase 1 - Core Mechanics Foundation
### Prompt 1: Leptos/Trunk Project Setup
- Create new Leptos project with Trunk setup
- Implement basic game logic skeleton:
  * Entity registry
  * Component storage
  * System scheduler
- Add CI pipeline for Rust/WASM compatibility
- Test case: Verify empty project launches with Leptos bindings

#### Test Cases
1. Fresh project runs without Rust errors
2. ECS registry can store/retrieve components
3. Basic Leptos-Rust signal passing works

### Prompt 2: Resource System Implementation
- Create ResourceManager singleton
- Implement GameResources struct with atomic floats
- Add ResourceMultipliers with thread-safe access
- Create serialization boilerplate
- Test case: Verify resource calculations with multipliers

#### Test Cases
1. Initial values match default expectations
2. Multipliers affect base values correctly
3. Atomic operations maintain thread safety

### Prompt 3: Click Handler Core
- Implement click event system
- Create RockComponent with random attributes
- Add inventory management skeleton
- Integrate with ResourceManager
- Test case: Verify click->resource flow

#### Test Cases
1. Single click generates rock entity
2. Inventory stores rock attributes
3. Click cooldown enforced properly 

### Prompt 4: Initial Upgrade System
- Create Upgrade struct with cost scaling
- Implement SocialMediaUpgrade category
- Add GeodeCities platform skeleton
- Wire to ResourceManager
- Test case: Verify first purchase flow

#### Test Cases
1. Upgrade costs scale correctly (4.2^x)
2. Purchase applies multiplier changes
3. Insufficient funds prevent purchase

### Prompt 5: Base UI Framework
- Create main game scene
- Implement resource display panel
- Add click target button
- Create upgrade menu skeleton
- Test case: Verify UI-data binding

#### Test Cases
1. UI reflects current resource values
2. Button clicks trigger game actions
3. Menu updates show available upgrades

## Phase 2 - Progression Systems

### Prompt 6: Platform System Foundation
- Implement Platform enum with progression
- Create PlatformUnlockSystem
- Add GeodeCities starter implementation
- Connect to ResourceManager
- Test case: Verify platform locking

#### Test Cases
1. Only GeodeCities starts unlocked
2. Follower thresholds block progression
3. Unlocks trigger UI updates

### Prompt 7: Viral Mechanics Core
- Implement ViralPost struct
- Create viral chance calculator
- Add cooldown timers
- Connect to ResourceManager
- Test case: Verify viral probability

#### Test Cases
1. Base chance matches platform specs
2. Cooldown blocks back-to-back virals
3. Mega-viral requires minimum stats

## Phase 3 - Advanced Systems

### Prompt 8: Heat System Implementation
- Create HeatCalculator
- Implement conversion from infamy
- Add logarithmic bonus calculations
- Connect to prestige UI
- Test case: Verify heat math

#### Test Cases
1. Minimum 10k infamy threshold
2. Bonuses follow log/linear curves
3. Conversion resets infamy properly

### Prompt 9: Automation Foundation
- Implement timed rock generation
- Create AutomationUpgrade struct
- Add background task system
- Connect to click handler
- Test case: Verify idle gains

#### Test Cases
1. Automation rate matches level
2. Upgrades scale production correctly
3. Disabled when resources insufficient

## Phase 4 - Polish & Optimization

### Prompt 10: Save System
- Implement JSON serialization
- Add versioned save format
- Create auto-save trigger
- Add load validation
- Test case: Verify save integrity

#### Test Cases
1. Round-trip save/load preserves state
2. Invalid saves trigger recovery
3. Version mismatch handled safely

## Tech Stack
- **Frontend**: Leptos (Rust/WASM framework)
- **Build System**: Trunk (Rust WASM pipeline)
- **UI**: Modern web components with DaisyUI/Tailwind
- **State Management**: Leptos for global state
- **Animation**: Rust WASM-powered particle effects
- **Audio**: WebAudio API integration

## Project Structure
```
frontend/
  src/
    components/     # Leptos components
    systems/        # Game systems
    utils/          # Helper functions
  index.html        # Trunk entry point
  Cargo.toml

common/
  src/lib.rs       # Shared game logic

```

## Key Dependencies
```toml
[dependencies]
# Frontend
leptos = "0.5"
leptos_reactive = "0.5"
gloo = "0.9"

# Game logic
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }

# Build
trunk = "0.17"