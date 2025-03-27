# Rock Influencer Idle Game - Development Checklist

## Phase 1 - Core Mechanics
### Project Setup
- [ ] Leptos/Trunk project
  - [ ] Trunk.toml configuration
  - [ ] Deploy to GitHub Pages
  - [ ] Component architecture skeleton
  - [ ] Basic page layout and routing

### Resource System
- [ ] `GameResources` struct implementation
  - [ ] WASM-compatible state management
  - [ ] Serialization for localStorage
- [ ] `ResourceMultipliers` system
  - [ ] Leptos store integration
  - [ ] Multiplier application logic
- [ ] Unit tests:
  - [ ] Default values initialization
  - [ ] Browser storage persistence
  - [ ] Overflow protection

### Click Mechanics
- [ ] Rock generation system
  - [ ] Attribute randomization (size/quality/luster)
  - [ ] Browser-based inventory management
- [ ] Click cooldown system
  - [ ] Progressive cooldown scaling
  - [ ] CSS/SVG visual feedback
- [ ] Integration tests:
  - [ ] Click->Resource flow validation
  - [ ] Inventory capacity limits

### Initial Upgrades
- [ ] SocialMediaUpgrade struct
  - [ ] Cost scaling formula (4.2^x)
  - [ ] Purchase validation logic
- [ ] GeodeCities implementation
  - [ ] Base viral chance
  - [ ] Follower growth curve
- [ ] Tests:
  - [ ] Upgrade cost progression
  - [ ] Purchase denial on insufficient funds

### Base UI
- [ ] Main game scene
  - [ ] Resource display panel
  - [ ] Click target button
  - [ ] Upgrade menu skeleton
- [ ] Data binding system
  - [ ] Real-time resource updates
  - [ ] Upgrade availability indicators
- [ ] UI tests:
  - [ ] Layout responsiveness
  - [ ] Input validation

## Phase 2 - Progression Systems
### Platform System
- [ ] Platform unlock requirements
- [ ] Platform-specific viral mechanics
- [ ] Cross-platform progression tracking

### Viral System
- [ ] Viral type implementations
- [ ] Cooldown management
- [ ] Mega-viral requirements

## Phase 3 - Advanced Systems
### Heat System
- [ ] Infamy conversion logic
- [ ] Logarithmic bonus calculations
- [ ] Prestige UI integration

### Automation
- [ ] Timed generation system
- [ ] Upgrade scaling models
- [ ] Background task manager

## Phase 4 - Polish
### Optimization
- [ ] Memory usage profiling
- [ ] Calculation caching system
- [ ] BigNumber implementation

### Save System
- [ ] JSON serialization
- [ ] Versioned save format
- [ ] Auto-save triggers

## Ancillary Systems
- [ ] Error handling framework
- [ ] Analytics tracking
- [ ] Localization setup
- [ ] Accessibility features
  - [ ] Colorblind mode
  - [ ] UI scaling

## Documentation
- [ ] Technical design doc
- [ ] Player-facing tutorial
- [ ] Modding API skeleton

## Infrastructure
- [ ] Build pipeline
- [ ] Crash reporting
- [ ] Update distribution system

## Asset Management
- [ ] Rock texture atlas
- [ ] UI sound effects
- [ ] Achievement icons