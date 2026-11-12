import bevy_enoki::shaders::particle_vertex_out::VertexOutput;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = length(in.uv - vec2(0.5)) * 2.0;
    let falloff = smoothstep(1.0, 0.2, d);          // wide, soft puff
    return vec4(in.color.rgb, in.color.a * falloff);
}