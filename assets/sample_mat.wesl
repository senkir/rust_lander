import bevy_enoki::shaders::particle_vertex_out::VertexOutput;

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var out = in.color
    // go wild
    return out;
}

