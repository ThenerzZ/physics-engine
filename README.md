# Physics Engine with Interactive Editor

A 3D physics engine built with Rust, using Bevy for rendering and Rapier for physics simulation.

## Features

- 3D physics simulation
- Interactive editor window
- Real-time object manipulation
- Collision detection and response
- Dynamic object creation

## Controls

- WASD: Move camera horizontally
- Q/E: Move camera up/down
- Space: Spawn new physics object
- Mouse: Look around (coming soon)

## Building and Running

Make sure you have Rust installed, then:

```bash
cargo run --release
```

## Dependencies

- bevy: Game engine and rendering
- bevy_rapier3d: Physics simulation

## Project Structure

- `src/main.rs`: Main application entry point and setup
- `src/editor.rs`: Editor controls and camera management
- `src/physics.rs`: Physics simulation and object behavior
