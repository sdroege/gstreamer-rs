
struct VertexInput {
  @location(0) pos: vec3<f32>,
  @location(1) uv: vec2<f32>,
}
struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
}

@vertex
fn main(vertex: VertexInput) -> VertexOutput {
    return VertexOutput(vec4(vertex.pos, 1.), vertex.uv);
}
