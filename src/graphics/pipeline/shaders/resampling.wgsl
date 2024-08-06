@group(0) @binding(0) var source_texture: texture_2d<f32>;
@group(0) @binding(1) var linear_sampler: sampler;

struct VertexInput {
    @location(0) canon: vec2f,
    @location(1) uv: vec2f,
};

struct FragmentInput {
    @builtin(position) clip_position: vec4f,
    @location(0) uv: vec2f,
};

@vertex
fn vertex(in: VertexInput) -> FragmentInput {
    return FragmentInput(vec4f(in.canon, 0., 1.), in.uv);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4f {
    let value = textureSample(source_texture, linear_sampler, in.uv).r;
    return vec4f(vec3f(value), 1.);
}
