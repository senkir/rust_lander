import bevy_enoki::shaders::particle_vertex_out::VertexOutput;

@group(1) @binding(0) var<uniform> tint: vec4<f32>;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);          // smooth interpolation
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    return 0.5 * value_noise(p)
         + 0.3 * value_noise(p * 2.0)
         + 0.2 * value_noise(p * 4.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = length(in.uv - vec2<f32>(0.5)) * 2.0;
    let soft = 1.0 - smoothstep(0.0, 1.0, d);            // soft round envelope, no hard edge

    // lifetime_total is random per particle, so it works as a seed
    // each particle gets its own wisp pattern
    let seed = vec2<f32>(in.lifetime_total * 37.0, in.lifetime_total * 19.0);
    let n = fbm(in.uv * 3.0 + seed + vec2<f32>(0.0, in.lifetime_frac * 0.6));

    // keep only the denser parts of the noise, which makes thin tendrils
    let wisp = soft * smoothstep(0.3, 0.8, n);

    let rgb = in.color.rgb * tint.rgb * (0.8 + n * 0.5);
    return vec4<f32>(rgb, in.color.a * wisp);
}