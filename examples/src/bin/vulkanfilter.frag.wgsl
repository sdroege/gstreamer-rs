
@group(0) @binding(0) var samp: sampler;
@group(0) @binding(1) var tex: texture_2d<f32>;

struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
}

@fragment
fn main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    var uv = vertex.uv;
    uv.y = 1. - uv.y; // convert to vulkan coordinates
    if uv.x > 0.5 { // sample from left half if in right half
        uv.x = 1. - uv.x;
    }
    return textureSample(tex, samp, uv);
}
