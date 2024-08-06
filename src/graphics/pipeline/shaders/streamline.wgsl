@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) direction: vec3<f32>,
};

struct FragmentInput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vertex(in: VertexInput) -> FragmentInput {
    return FragmentInput(
        transform * vec4<f32>(in.position, 1.),
        abs(in.direction)
    );
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4f {
    return vec4f(in.color, 1.);
}
