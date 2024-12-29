# Physics Engine by ThenerzZ

A demonstration project showcasing how to build a physics-based engine using Rust. This project serves as an educational example of implementing physics simulation, 3D rendering, and interactive UI in Rust, while also being a foundation for future game development projects.

## About

This engine is primarily a proof-of-concept demonstrating:
- How to integrate physics simulation with a modern game engine (Bevy)
- Ways to implement interactive 3D object manipulation
- Techniques for building responsive UI for 3D applications
- Basic architecture for a physics-based game engine

While functional, this is meant to be an educational resource and starting point rather than a production-ready engine.

## Features

- **Interactive 3D Environment**
  - Orbit camera controls
  - Object selection and manipulation
  - Real-time physics simulation

- **Multiple Shape Types**
  - Cubes
  - Spheres
  - Cylinders
  - Cones
  - Capsules

- **Physics Properties**
  - Dynamic rigid bodies
  - Collision detection
  - Mass properties
  - Friction and restitution
  - Damping controls

- **User Interface**
  - Inspector panel for object properties
  - Transform tools (Move, Rotate, Scale)
  - Easy object creation via dropdown menu
  - Real-time property editing

## Learning Points

This project demonstrates several key concepts in game engine development:
- Physics integration with rendering systems
- Event handling and user input
- UI state management
- 3D object manipulation
- Component-based architecture using Bevy ECS

## Technologies Used

- **Bevy** (v0.12.0) - Game engine and rendering
- **Bevy Rapier 3D** (v0.23.0) - Physics simulation
- **Bevy Egui** (v0.24.0) - User interface

These choices showcase modern Rust game development practices and tools.

## Getting Started

1. Install Rust and Cargo
2. Clone the repository
3. Run the engine:
```bash
cargo run
```

## Controls

- **Left Click**: Select objects
- **Right Click + Drag**: Orbit camera
- **Mouse Wheel**: Zoom in/out
- **UI Tools**:
  - Select: Choose objects
  - Move: Translate objects
  - Rotate: Rotate objects
  - Scale: Resize objects

## Future Development

This engine is being developed as a foundation for future games by ThenerzZ. Planned features include:
- More primitive shapes
- Custom mesh support
- Advanced physics constraints
- Scene saving/loading
- Material system
- Particle effects

## Author

Created by ThenerzZ

## License

This project is intended for personal use in future game development projects by ThenerzZ.
