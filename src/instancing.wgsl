#import mikage::scene_types
#import mikage::math

@group(0) @binding(0) var<uniform> scene: SceneUniform;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) i_pos: vec2<f32>,
    @location(3) i_angle: f32,
    @location(4) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
};

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    // 回転の適用
    let rotated = rotate2d(v.position.xy, v.i_angle);

    // 平行移動の適用
    let world_pos = vec4<f32>(
        rotated.x + v.i_pos.x,
        rotated.y + v.i_pos.y,
        0.0,
        1.0,
    );

    var out: VertexOutput;
    out.clip_position = scene.view_proj * world_pos;
    out.color = v.i_color;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
