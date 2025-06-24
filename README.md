# Helix Engine and Controlled Burn Game

## Project Overview

Welcome to the Helix Engine project, the foundation for a next-generation 3D game engine and its debut game, Controlled Burn. This project combines advanced procedural generation techniques with unique voxel and mesh attribute systems under the umbrella of MeshDNA/VoxelDNA technology.

### Key Features and Unique Aspects

- **Procedurally Generated Maps:** The engine uses procedural algorithms to create vast, dynamic environments ensuring every playthrough is unique.

- **VoxelDNA & MeshDNA Technology:** Each voxel and mesh in the engine carries a DNA-like attribute set. This system enables different behaviors, interactions, and properties at the voxel level, allowing for complex, emergent gameplay that is highly customizable.

- **Hybrid 2D-3D Top-Down Gameplay:** While the game Controlled Burn presents a 2D top-down perspective, it leverages fully 3D elements to enrich visual fidelity and interaction complexity.

- **Multi-Platform Ambitions:** The engine and Controlled Burn game will target PC, VR, Console, and Mobile platforms to reach a broad audience and demonstrate the engine’s versatility.

- **Future VR and AR Support:** Helix Engine is designed with extensibility in mind, preparing the groundwork for eventual VR and AR integrations.

- **Modular, Expandable Architecture:** Every subsystem — input, rendering, camera, procedural generation, physics — is modular, allowing for easier maintenance, upgrading, and customization.

---

## Project Structure

- **Core Engine (Helix Engine)**
  - Rendering pipeline using `wgpu` with shader-based GPU acceleration.
  - Input handling supporting keyboard, mouse, and planned VR/AR controllers.
  - Procedural generation framework adaptable for terrain, objects, and AI placement.
  - Voxel and Mesh DNA system managing attributes and behaviors.
  - Camera and scene management for 2D and 3D hybrid views.
  - Multi-platform backend for PC, consoles, mobile devices, and VR.

- **Controlled Burn Game**
  - First game built on Helix Engine.
  - Top-down voxel-based gameplay with dynamic procedural maps.
  - Focus on performance and immersive interaction.
  - Planned support for multiplayer and modding.

---

## Development Roadmap

1. **Core Engine Development**
   - Complete input and camera systems.
   - Develop procedural map generator.
   - Implement VoxelDNA/MeshDNA attribute framework.
   - Build rendering pipeline with shaders.
   - Develop platform abstraction layer.

2. **Controlled Burn Game Buildout**
   - Create game logic and mechanics.
   - Integrate procedural maps.
   - Design voxel and mesh attributes for gameplay.
   - Test across target platforms.

3. **VR/AR Extensions**
   - Add VR input and rendering support.
   - Develop AR overlays and interaction.

4. **Tools and Editors**
   - Build map editors, attribute editors.
   - Create asset pipelines.
   - Develop debugging and profiling tools.

5. **Licensing and Distribution**
   - Finalize commercial licensing for Helix Engine.
   - Launch Controlled Burn on platforms.
   - Open selected engine modules to community.

---

## How to Use This Repository

This repository currently contains the foundational source code of the Helix Engine and initial Controlled Burn assets. It is private and under active development.

- To build the project, ensure you have Rust and related dependencies installed.
- Use `cargo build` and `cargo run` for local compilation and testing.
- The `src/` directory contains engine subsystems and game-specific code.
- The `rendering/shaders/` directory holds WGSL shader files.
- Input and camera modules are designed to be extended for VR and AR.

---

## Contribution and Licensing

- At this stage, this project is closed-source and private.
- Licensing will be modeled similar to Unity and Unreal Engine, with commercial and personal use options.
- Contributions will be accepted after a formal open-source or licensing announcement.
- For business inquiries or licensing information, please contact [Your Contact Info].

---

## Contact

Project Lead: Shane Foster  
Email: [havencommandllc@gmail.com]  
GitHub: [https://github.com/Op-era/HelixEngine](https://github.com/Op-era/HelixEngine)

---

Thank you for your interest in the Helix Engine and Controlled Burn project. Together we will create a powerful, flexible engine and a compelling game experience.
