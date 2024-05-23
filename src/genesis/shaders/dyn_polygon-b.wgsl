struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_cord: vec2<f32>,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_cord: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOut {
    var out: VertexOut;
    out.tex_cord = model.tex_cord;
    out.clip_position = vec4<f32>(model.position.x + 0.3, model.position.y + 0.3, model.position.z, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.5, 0.4, 1.0);
}
