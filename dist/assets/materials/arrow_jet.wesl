import bevy_enoki::shaders::particle_vertex_out::VertexOutput;

@group(1) @binding(0) var<uniform> tint: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = length(in.uv - vec2(0.5)) * 2.0;

    // tight bright core plus a faint halo
    let core = pow(clamp(1.0 - d, 0.0, 1.0), 4.0);
    let halo = pow(clamp(1.0 - d, 0.0, 1.0), 1.5) * 0.3;
    let shape = core + halo;

    // overdrive the color so it reads as hot (more with bloom on)
    let rgb = in.color.rgb * tint.rgb * (1.0 + core * 2.0);
    return vec4(rgb, in.color.a * clamp(shape, 0.0, 1.0));
}