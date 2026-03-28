#import mikage::scene_types
#import mikage::math
#import mikage::color_utils

@group(0) @binding(0) var<uniform> scene: SceneUniform;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) i_pos_angle: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    let angle = v.i_pos_angle.w;

    // 回転の適用
    let rotated = rotate2d(v.position.xy, angle);

    // 平行移動の適用
    let world_pos = vec4<f32>(
        rotated.x + v.i_pos_angle.x,
        rotated.y + v.i_pos_angle.y,
        0.0,
        1.0,
    );

    var out: VertexOutput;
    out.clip_position = scene.view_proj * world_pos;

    // HSV coloring（hueはラジアン [0,TAU)、bevy/mikage共通）
    let hue = 3.84 + sin(angle) * 0.333;
    let saturation = sin(1.666 * v.i_pos_angle.x) * 0.166 + 0.666;
    let value = sin(v.i_pos_angle.y) * 0.166 + 0.833;
    out.color = vec4<f32>(hsv2rgb(hue, saturation, value), 1.0);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
