struct VertexInput {
    @location(0) canon: vec2f,
    @location(1) uv: vec2f,
};

struct FragmentInput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vertex(in: VertexInput) -> FragmentInput {
    return FragmentInput(
        vec4f(in.canon, 0., 1.),
        vec4f((in.canon + 1.) / 2, 1., 1.)
    );
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4f {
    return in.color;
}
