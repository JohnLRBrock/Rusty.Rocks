# Rock Influencer Idle Game - Technical Specification

## 1. Technical Stack
- **Engine:** Godot
- **Language:** Rust
- **Architecture:** Entity Component System (ECS) recommended for managing numerous game entities and states

## 2. Core Systems Architecture

### 2.1 Resource Management
```rust
struct GameResources {
    clout: f64,
    followers: f64,
    infamy: f64,
    heat: i32,
    money: f64
}

struct ResourceMultipliers {
    viral_chance: f64,
    mega_viral_chance: f64,
    clout_gain: f64,
    follower_gain: f64
}
```

### 2.2 Social Media Platform System
```rust
enum Platform {
    GeodeCities,    // Starting platform
    Rockster,
    MyStone,
    RockBook,
    Stonr,
    Instagranite,
    RokTok
}

struct PlatformStats {
    unlocked: bool,
    followers: i64,
    viral_cooldown: Duration,
    viral_ceiling: f64,
    base_viral_chance: f64
}
```

## 3. Core Game Loops

### 3.1 Click Collection System
- Base click handler with upgrade modifiers
- Random rock generation on click
- Inventory management system
- Click cooldown management

### 3.2 Viral System
```rust
struct ViralPost {
    platform: Platform,
    viral_type: ViralType,
    cooldown: Duration,
    spread_platforms: Vec<Platform>,
    rewards: ResourceRewards
}

enum ViralType {
    Normal,
    Mega
}
```

## 4. Progression Systems

### 4.1 Upgrade Categories
Each category should have ~12-13 upgrades before first progression wall:

- Social Media
- Rock Preparation
- Rock Gathering
- Automation

### 4.2 Cost Scaling
- Base Formula: `new_cost = base_cost * (4.2^upgrade_level)`
- Implement with logarithmic calculations for large numbers

### 4.3 Heat System
```rust
struct HeatCalculator {
    base_conversion_rate: f64,    // 1 per 10k infamy
    viral_chance_bonus: f64,      // Logarithmic scaling
    mega_viral_chance_bonus: f64, // Logarithmic scaling
    ceiling_multiplier: f64,      // Linear scaling
    base_clout_bonus: f64,        // Linear scaling
    base_follower_bonus: f64      // Linear scaling
}
```

## 5. Data Management

### 5.1 Save System
- Implement auto-save every 5 minutes
- Save format: JSON
- Include version control for save compatibility
- Store in user's local directory

### 5.2 State Management
- Use Redux-style state management
- Implement undo/redo system for purchases
- Cache frequently accessed values

## 6. Error Handling

### 6.1 Critical Errors
- Implement crash recovery
- Save state backup system
- Error logging system

### 6.2 Game Balance Errors
- Implement sanity checks for resource generation
- Monitor for exploitation vectors
- Add anti-cheat mechanisms

## 7. Testing Strategy

### 7.1 Unit Tests
- Resource calculations
- Upgrade scaling
- Viral chance calculations
- Heat conversion logic

### 7.2 Integration Tests
- Platform unlocking sequence
- Resource flow between systems
- Save/load functionality
- Prestige mechanics

### 7.3 Performance Tests
- Large number handling
- Multiple viral posts
- Resource calculation efficiency
- Memory usage optimization

## 8. UI/UX Requirements

### 8.1 Core Screens
#### Main Game View
- Central clicking area
- Resource displays
- Upgrade menu access

#### Social Media Interface
- Platform-specific layouts
- Post creation interface
- Viral status indicators

#### Upgrade Menu
- Category organization
- Cost/benefit display
- Purchase confirmation

### 8.2 Notification System
- Email notification queue
- Viral post alerts
- Achievement notifications
- Upgrade availability alerts

## 9. Performance Considerations

### 9.1 Optimization Targets
- Target 60 FPS on mid-range hardware
- Maximum memory usage: 500MB
- Save file size limit: 5MB
- Background processing for viral calculations

### 9.2 Number Handling
- Implement BigNumber system for large values
- Scientific notation for display
- Efficient calculation caching

## 10. Development Phases

### Phase 1: Core Mechanics
- Basic clicking mechanism
- Resource generation
- Initial upgrade system
- GeodeCities platform

### Phase 2: Progression Systems
- Additional platforms
- Viral mechanics
- Upgrade categories
- Basic automation

### Phase 3: Advanced Features
- Heat system
- PR management
- Cross-platform viral spread
- Advanced automation

### Phase 4: Polish
- UI refinement
- Balance testing
- Performance optimization
- Save system implementation

## 11. Maintenance Plan

### 11.1 Post-Launch Monitoring
- Error tracking system
- Performance metrics
- Player progression analytics
- Balance adjustment capabilities

### 11.2 Update Strategy
- Regular balance patches
- Content updates
- Bug fix priority system
- Version control strategy