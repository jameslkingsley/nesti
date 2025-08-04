# Nesti Refactoring Plan: From Trait Objects to Enum + ECS

## Overview

This document outlines the plan to refactor Nesti from using trait objects (`Element` trait) to a more idiomatic Bevy ECS approach using an enum for the public API and component-based architecture internally.

## Goals

1. Keep the public API minimal and ergonomic
2. Enable `Element::tick` to be called during `Nesti::flush`
3. Remove trait objects in favor of enum dispatch
4. Leverage Bevy ECS patterns for better performance and maintainability

## Current Architecture Issues

- `Element` trait objects are only available during `put()` calls
- No way to call `tick()` on elements during `flush()`
- Timer elements need access to their configuration (e.g., `Truncate`) during tick

## Proposed Solution

### 1. Public API: Element Enum

Replace the `Element` trait with an enum that users import:

```rust
// In lib.rs or elements/mod.rs
#[derive(Debug, Clone)]
pub enum Element {
    // Text elements
    Text(String),
    
    // Number elements
    Integer(i64),
    IntegerUnit(i64, String),
    Decimal(f64),
    DecimalUnit(f64, String),
    
    // Timer element (needs ticking)
    Timer(Truncate),
    
    // Progress bar (might need ticking for rate)
    Progress {
        current: u64,
        maximum: u64,
        show_percent: bool,
        show_values: bool,
        show_rate: bool,
    },
    
    // Boolean
    Boolean(bool),
    
    // Size
    Size(u64),
    
    // Color
    Color { r: u8, g: u8, b: u8 },
    
    // Container types
    Vector(Vec<(String, Element)>),
    Map(Vec<(String, Element)>),
}
```

Usage example:
```rust
use nesti::{nesti, Element::*};

nesti("path/to/element", Timer(Seconds));
nesti("path/to/number", IntegerUnit(123, "trees".to_string()));
```

### 2. Internal Architecture: Component-Based

#### Marker Components
Create marker components for each element type that needs special behavior:

```rust
#[derive(Component)]
struct TimerElement;

#[derive(Component)]
struct ProgressElement;

// etc. for other elements that need special handling
```

#### Configuration Components
Store element-specific configuration as components:

```rust
#[derive(Component)]
struct TimerConfig {
    truncate: Truncate,
}

#[derive(Component)]
struct ProgressConfig {
    maximum: u64,
    show_percent: bool,
    show_values: bool,
    show_rate: bool,
}

// The existing TimeComponent for Timer
#[derive(Component)]
struct TimeComponent(Instant);
```

### 3. System-Based Updates

Create systems for elements that need ticking:

```rust
fn tick_timers(
    mut query: Query<(&mut Content, &TimeComponent, &TimerConfig), With<TimerElement>>
) {
    for (mut content, time, config) in &mut query {
        let uptime = time.elapsed();
        content.0 = uptime.human_with_format(config.truncate, TimerFormatter).to_string();
    }
}

fn tick_progress(
    mut query: Query<(&mut Content, &ProgressConfig, /* other components */), With<ProgressElement>>
) {
    // Update progress bars that show rate
}
```

### 4. Refactored put() Method

```rust
impl Nesti {
    pub fn put<P>(&self, path: P, element: Element)
    where
        P: Into<String>,
    {
        let path = path.into();
        let mut world = self.world.write();

        // Check if entity exists at path
        let entity = /* query for existing entity */;

        match element {
            Element::Timer(truncate) => {
                if let Some(entity) = entity {
                    // Update existing timer
                    let mut ent = world.entity_mut(entity);
                    // Update content based on current time
                } else {
                    // Spawn new timer
                    world.spawn((
                        Path(path),
                        InsertionOrder(order),
                        Content("0s".to_string()),
                        TimerElement,
                        TimerConfig { truncate },
                        TimeComponent(Instant::now()),
                    ));
                }
            }
            Element::IntegerUnit(value, unit) => {
                let content = format!("{} {}", value.to_formatted_string(&Locale::en), unit);
                if let Some(entity) = entity {
                    world.entity_mut(entity).insert(Content(content));
                } else {
                    world.spawn((
                        Path(path),
                        InsertionOrder(order),
                        Content(content),
                    ));
                }
            }
            // ... handle other variants
        }
    }
}
```

### 5. Refactored flush() Method

```rust
impl Nesti {
    pub fn flush(&self) -> Result<(), std::io::Error> {
        let mut world = self.world.write();
        world.flush();

        // Run tick systems
        tick_timers(&mut world);
        tick_progress(&mut world);
        // ... other tick systems

        let content = self.render(&mut world);
        // ... rest of flush logic
    }
}
```

#### Alternative: One-Shot Systems

Instead of calling functions directly, register systems with the World:

```rust
// During initialization
let timer_system_id = world.register_system(tick_timers);
let progress_system_id = world.register_system(tick_progress);

// During flush
world.run_system(timer_system_id);
world.run_system(progress_system_id);
```

### 6. Benefits

1. **No trait objects** - Everything is statically dispatched
2. **Idiomatic ECS** - Uses Bevy's component and system patterns
3. **Efficient queries** - Bevy can optimize component access
4. **Extensible** - Easy to add new element types
5. **Clean API** - Users just import and use enum variants

### 7. Migration Steps

1. Create the `Element` enum with all variants
2. Add marker and config components
3. Implement the match-based `put()` method
4. Create tick systems for dynamic elements
5. Update `flush()` to run tick systems
6. Remove the old `Element` trait and implementations
7. Update examples and tests

### 8. Considerations

- For complex types like `Vector` and `Map`, we might need recursive spawning
- Style overrides can be handled by checking for an optional style in the enum or as a separate parameter
- Generic number types (IntegerLike, FloatLike) might need to be simplified to concrete types in the enum