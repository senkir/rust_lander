import bevy_enoki::shaders::particle_vertex_out::VertexOutput;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // distance from quad center, 0 at center, 1 at the edge
    let d = length(in.uv - vec2(0.5)) * 2.0;

    // soft round falloff; squaring makes the edge fade faster
    let falloff = pow(clamp(1.0 - d, 0.0, 1.0), 2.0);

    // hot core: brighter near the center and early in life
    let core = falloff * (1.0 - in.lifetime_frac);
    let rgb = in.color.rgb * (1.0 + core * 1.5);

    return vec4(rgb, in.color.a * falloff);
// return vec4(1.0, 0.0, 1.0, 1.0); //TEST: magenta
}