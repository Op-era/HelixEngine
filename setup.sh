#!/bin/bash

set -e

# Root src folder
mkdir -p src

# Create main.rs
cat > src/main.rs << 'EOF'
fn main() {
    println!("Volumetric Engine: Starting...");
}
EOF

# Create lib.rs with all module declarations
cat > src/lib.rs << 'EOF'
pub mod state;
pub mod mesh;
pub mod physics;
pub mod rendering;
pub mod animation;
pub mod scripting;
pub mod tools;
pub mod input;
pub mod camera;
pub mod config;
pub mod utils;
EOF

# Create src/state.rs
touch src/state.rs

# Create flat modules in src/
touch src/input.rs
touch src/camera.rs
touch src/config.rs
touch src/utils.rs

# Create mesh folder and files
mkdir -p src/mesh
touch src/mesh/mod.rs
touch src/mesh/geometry.rs
touch src/mesh/dna.rs
touch src/mesh/skeleton.rs

# Create physics folder and files
mkdir -p src/physics
touch src/physics/mod.rs
touch src/physics/simulation.rs
touch src/physics/fluids.rs
touch src/physics/combustion.rs
touch src/physics/materials.rs

# Create rendering folder and files
mkdir -p src/rendering/shaders
touch src/rendering/mod.rs
touch src/rendering/pipeline.rs
touch src/rendering/mesh_renderer.rs

# Create animation folder and files
mkdir -p src/animation
touch src/animation/mod.rs
touch src/animation/skeleton_animation.rs
touch src/animation/mesh_deformation.rs

# Create scripting folder and files
mkdir -p src/scripting
touch src/scripting/mod.rs
touch src/scripting/lua_bindings.rs
touch src/scripting/script_manager.rs

# Create tools folder and files
mkdir -p src/tools
touch src/tools/mod.rs
touch src/tools/editor.rs
touch src/tools/exporter.rs

echo "Folder structure and files created."

# Optional: create minimal Cargo.toml if missing
if [ ! -f Cargo.toml ]; then
cat > Cargo.toml << 'EOF'
[package]
name = "volumetric_engine"
version = "0.1.0"
edition = "2021"

[dependencies]
wgpu = "0.17.2"
winit = "0.28.7"
glam = "0.24"
pollster = "0.3"
gltf = "1.3"
bytemuck = { version = "1.9", features = ["derive"] }
env_logger = "0.10"
log = "0.4"
EOF
echo "Minimal Cargo.toml created."
fi
