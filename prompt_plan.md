## Phase 0 - Core Mechanics Foundation
### Leptos/Trunk Project Setup
- Create new Leptos project with Trunk setup
- Add CI pipeline for Rust/WASM compatibility

### Base UI Framework
- Create main game scene
  - Implement resource display panel
  - Implement Map
- Add clickable Rock

### Rock Generation System
- Create RockFactory that generates rocks with random attributes
- Add inventory 

## Phase 1 - Automation

### Time System
- Game time delta system
- Game event scheduler

### Save System
- Implement JSON serialization
- Add versioned save format
- Create auto-save trigger
- Add load validation

### Initial Upgrade System
- Implmement automated clout and rock upgrades

## Phase 2 - Core Gameplay

### Other Locations
  - Implement other locations
  - Fill out map
  - Add requirements for each location
  - Add location specific events

## Phase 3 - Social Media

### Social Media Skeleton
- Create GeodeCities Page
- Add ability to post on GeodeCities
- Add email
- Add GeodeCities upgrade
- Add skeleton for next social media sites

### Viral Mechanics Core
- Implement ViralPost struct
- Create viral chance calculator
- Add cooldown mechanic
- Add mega-viral mechanic

### Heat and Infamy

### Add Infamy
- Add Infamy mechanic
- Add Infamy raising events
- Add effect of Infamy to other systems

### Heat System Implementation
- Add Heat mechanic
- Add effect of heat to other systems
- Create prestige UI skeleton
- Create prestige system
