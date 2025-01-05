#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
    @location(5) i_angle: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    // 1) 回転行列の構築
    let c = cos(vertex.i_angle);
    let s = sin(vertex.i_angle);
    let rotation_matrix = mat2x2<f32>(
        c, s,
        -s,  c
    );

    // 2) 回転の適用
    let rotated_pos = rotation_matrix * vertex.position.xy;

    // 3) 平行移動の適用
    let translated_pos = vec3<f32>(
        rotated_pos.x + vertex.i_pos_scale.x,
        rotated_pos.y + vertex.i_pos_scale.y,
        vertex.position.z + vertex.i_pos_scale.z
    );

    // 4) スケールの適用 (平行移動後)
    let scaled_pos = translated_pos * vertex.i_pos_scale.w;

    // 5) Bevy の既存関数でクリップ座標系へ変換
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(scaled_pos, 1.0)
    );

    out.color = vertex.i_color; // フラグメントシェーダへ渡す
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
