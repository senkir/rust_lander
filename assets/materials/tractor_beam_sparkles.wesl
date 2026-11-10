import bevy_enoki::shaders::particle_vertex_out::VertexOutput;

@group(1) @binding(0) var<uniform> tint: vec4<f32>;
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = (in.uv - vec2<f32>(0.5)) * 2.0;
    let a = abs(p);

    // thin rays that fade out along their length
    let ray_v = max(1.0 - a.x * 12.0, 0.0) * pow(max(1.0 - a.y, 0.0), 2.0);
    let ray_h = max(1.0 - a.y * 12.0, 0.0) * pow(max(1.0 - a.x, 0.0), 2.0);

    // small faint core instead of a hard dot
    let core = pow(max(1.0 - length(p), 0.0), 6.0) * 0.5;

    let star = clamp(0.8 * (ray_v + ray_h) + core, 0.0, 1.0);

    let twinkle = 0.55 + 0.45 * sin(in.lifetime_frac * 40.0 + in.lifetime_total * 50.0);

    let rgb = in.color.rgb * tint.rgb * 1.6;
    return vec4<f32>(rgb, in.color.a * tint.a * star * twinkle);
}