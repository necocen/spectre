#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}
#import bevy_render::color_operations::hsv_to_rgb

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_pos_angle: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let angle = vertex.i_pos_angle.w;
    // 1) 回転行列の構築
    let c = cos(angle);
    let s = sin(angle);
    let rotation_matrix = mat2x2<f32>(
        c, s,
        -s, c
    );

    // 2) 回転の適用
    let rotated_pos = rotation_matrix * vertex.position.xy;

    // 3) 平行移動の適用
    let translated_pos = vec3<f32>(
        rotated_pos.x + vertex.i_pos_angle.x,
        rotated_pos.y + vertex.i_pos_angle.y,
        vertex.position.z + vertex.i_pos_angle.z
    );

    // 4) Bevy の既存関数でクリップ座標系へ変換
    var out: VertexOutput;
    out.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(translated_pos, 1.0)
    );

    // 5) 色の適用
    let hue = 3.84 + sin(angle) * 0.333; // 青を中心に色相を変化
    let saturation = sin(1.666 * vertex.i_pos_angle.x) * 0.166 + 0.666;
    let value = sin(vertex.i_pos_angle.y) * 0.166 + 0.833;
    out.color = vec4<f32>(hsv_to_rgb(vec3<f32>(hue, saturation, value)), 1.0);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
