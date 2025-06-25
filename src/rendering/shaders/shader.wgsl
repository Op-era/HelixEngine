struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var pos: vec3<f32>;
    if in_vertex_index == 0u {
        pos = vec3<f32>(0.0, 0.5, 0.0);
    } else if in_vertex_index == 1u {
        pos = vec3<f32>(-0.5, -0.5, 0.0);
    } else {
        pos = vec3<f32>(0.5, -0.5, 0.0);
    }
    let world_pos = vec4<f32>(pos, 1.0);
    return camera.view_proj * world_pos;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red triangle
}