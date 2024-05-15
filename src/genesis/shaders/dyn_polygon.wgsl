struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput,) -> VertexOut {
    var out: VertexOut;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
  }

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
  }
