## Phase 0 - Core Mechanics Foundation
### Project Setup
- [x] Leptos/Trunk project
  - [x] Deploy to GitHub Pages

### Base UI Framework
- [x] Main game scene
  - [x] Resource display panel
  - [x] Implement Map
  - [x] Clickable Rock target

### Rock Generation System
- [ ] Create RockFactory that generates rocks with random attributes
  - [x] Define basic mineral types and attributes
    - [x] Create mineral type enums
    - [x] Implement attribute structs
  - [x] Implement RockFactory core
    - [x] Random attribute generation
    - [x] Quality calculations
    - [x] Factory configuration
  - [x] Integrate with GameState
    - [x] Add rock inventory
    - [x] Implement capacity system
  - [x] Update UI components
    - [x] Display rock attributes
- [x] Inventory capacity limits

## Phase 1 - Automation

### Time System
- [ ] Game time delta system
- [ ] Game event scheduler

### Save System
- [ ] JSON serialization
- [ ] Versioned save format
- [ ] Auto-save triggers
- [ ] Load validation

### Initial Upgrade System
- [ ] Implement automated clout and rock upgrades

## Phase 2 - Core Gameplay

### Location System
- [ ] Implement other locations
  - [ ] Fill out map
  - [ ] Add requirements for each location
  - [ ] Add location specific events
  - [ ] Location-specific rock distributions

## Phase 3 - Social Media

### Social Media Skeleton
- [ ] Create GeodeCities Page
- [ ] Add ability to post on GeodeCities
- [ ] Add email system
- [ ] Add GeodeCities upgrades

### Viral Mechanics Core
- [ ] Implement ViralPost struct
- [ ] Create viral chance calculator
- [ ] Add cooldown mechanic
- [ ] Add mega-viral mechanic

### Add monetization
- [ ] Add monetization system
- [ ] Add upgrades that cost money

## Phase 4 - Heat and Infamy

### Infamy System
- [ ] Add Infamy mechanic
- [ ] Add Infamy raising events
- [ ] Add effect of Infamy to other systems
- [ ] PR Manager implementation

### Heat System
- [ ] Add Heat mechanic
- [ ] Add effect of heat to other systems
- [ ] Create prestige UI skeleton
- [ ] Create prestige system