import bevy_enoki::shaders::particle_vertex_out::VertexOutput;

@group(1) @binding(0)
var<uniform> tint: vec4<f32>;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f); // smooth interpolation
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    return 0.5 * value_noise(p) + 0.3 * value_noise(p * 2.0) + 0.2 * value_noise(p * 4.0);
}
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = length(in.uv - vec2<f32>(0.5)) * 2.0;

    // bright smooth center falling off gradually, so no visible edge
    let glow = pow(1.0 - smoothstep(0.0, 1.0, d), 1.6);

    // same per-particle drift through a shared screen-space field as before,
    // but now it only modulates the light, with no threshold cutting shapes out
    let a = in.lifetime_total * 61.0;
    let off = vec2<f32>(cos(a), sin(a)) * in.lifetime_frac * 1.2;
    let p = in.clip_position.xy * 0.008;                       // bigger features than the fog
    let n = 0.5 * (fbm(p + off) + fbm(p * 1.15 - off * 0.7 + vec2<f32>(3.1, 7.7)));

    let shimmer = 0.7 + 0.6 * n;                               // about 1.0 on average, ranges 0.7 to 1.3

    // alien color drift: the base green slides toward violet where the field is high
    let accent = vec3<f32>(0.55, 0.4, 1.0);
    let hue = smoothstep(0.4, 0.7, n) * 0.45;
    let rgb = mix(in.color.rgb, accent, hue) * tint.rgb * (1.0 + glow * 0.6);

    return vec4<f32>(rgb, in.color.a * tint.a * glow * shimmer);
}