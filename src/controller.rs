use glam::Vec2;
use lyon_tessellation::{
    geom::Point, geometry_builder::simple_builder, path::Path, FillOptions, FillTessellator,
    VertexBuffers,
};
use mikage::InstanceVertex;

use crate::{
    tiles::{Anchor, Skeleton, Spectre, SpectreCluster, SpectreIter},
    utils::{Aabb, Angle, HexVec},
};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpectreInstance {
    pub position: [f32; 3],
    pub angle: f32,
}

impl InstanceVertex for SpectreInstance {
    fn vertex_attributes() -> Vec<mikage::wgpu::VertexAttribute> {
        vec![mikage::wgpu::VertexAttribute {
            format: mikage::wgpu::VertexFormat::Float32x4,
            offset: 0,
            shader_location: 2,
        }]
    }
}

/// Spectreタイルのメッシュを生成する
pub fn create_spectre_mesh() -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
    let mut path_builder = Path::builder();
    let points = Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO).vertices();
    let points_vec2: Vec<Vec2> = points.iter().map(|p| p.to_vec2()).collect();
    path_builder.begin(Point::new(points_vec2[0].x, points_vec2[0].y));
    for point in points_vec2.iter().skip(1) {
        path_builder.line_to(Point::new(point.x, point.y));
    }
    path_builder.close();
    let path = path_builder.build();

    let mut buffers: VertexBuffers<Point<f32>, u16> = VertexBuffers::new();
    {
        let mut vertex_builder = simple_builder(&mut buffers).with_inverted_winding(); // 反時計回りにする
        let mut tessellator = FillTessellator::new();
        let result =
            tessellator.tessellate_path(&path, &FillOptions::default(), &mut vertex_builder);
        assert!(result.is_ok());
    }

    let positions: Vec<[f32; 3]> = buffers.vertices.iter().map(|p| [p.x, p.y, 0.0]).collect();
    let normals: Vec<[f32; 3]> = buffers.vertices.iter().map(|_| [0.0, 0.0, 1.0]).collect();
    let indices: Vec<u32> = buffers.indices.iter().map(|&i| i as u32).collect();

    (positions, normals, indices)
}

#[inline]
fn to_instance(spectre: &Spectre) -> SpectreInstance {
    let anchor_pos = spectre.coordinate(Anchor::Anchor1).to_vec2();
    SpectreInstance {
        position: [anchor_pos.x, anchor_pos.y, 0.0],
        angle: spectre.rotation().to_radians(),
    }
}

pub struct TilesController {
    spectres: Box<SpectreCluster>,
}

impl TilesController {
    /// クラスターの最大レベル。これ以上拡張しようとすると座標がi32の範囲を超えるため。
    const MAX_CLUSTER_LEVEL: usize = 18;

    pub fn new() -> Self {
        let skeleton = Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 5, None)
            .to_spectre_cluster(&Aabb::NULL);
        let spectres = Box::new(skeleton);
        Self { spectres }
    }

    pub fn expand(&mut self) {
        if self.spectres.level() > Self::MAX_CLUSTER_LEVEL {
            tracing::warn!("Cannot expand more");
            return;
        }

        // 現在のSpectreClusterをAまたはFとして上位のSpectreClusterを生成する
        let mut spectres = Box::new(
            Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 1, None)
                .to_spectre_cluster(&Aabb::NULL),
        );
        std::mem::swap(&mut self.spectres, &mut spectres);
        if spectres.level().is_multiple_of(2) {
            tracing::info!("Expand from A");
            self.spectres = Box::new(SpectreCluster::with_child_a(*spectres));
        } else {
            tracing::info!("Expand from F");
            self.spectres = Box::new(SpectreCluster::with_child_f(*spectres));
        }
    }

    pub fn update(&mut self, bbox: &Aabb) {
        self.spectres.update(bbox);
    }

    pub fn spectres_in(&self, bbox: &Aabb) -> SpectreIter<'_> {
        self.spectres.spectres_in(*bbox)
    }
}

#[derive(Default)]
pub struct LastViewState {
    /// カメラの表示範囲
    pub bbox: Option<Aabb>,
    /// 前のフレームでタイルを拡大したかどうか
    pub expanded: bool,
}

/// カメラのビューに基づいてタイルの表示を更新する。
/// bboxに変更がない場合はNoneを返す。
pub fn update_tiles(
    controller: &mut TilesController,
    last_view: &mut LastViewState,
    bbox: &Aabb,
) -> Option<Vec<SpectreInstance>> {
    // 前フレームと同じbboxの場合は早期リターン
    if let Some(last_bbox) = last_view.bbox
        && last_bbox == *bbox
        && !last_view.expanded
    {
        return None;
    }
    last_view.bbox = Some(*bbox);

    // bboxに含まれるタイルを取得してインスタンスデータを生成
    controller.update(bbox);
    let spectres = controller.spectres_in(bbox);
    let instance_data: Vec<SpectreInstance> = spectres.map(to_instance).collect();

    // 描画対象タイルの重心の偏りによってタイル生成の要否を判定する
    last_view.expanded = false;
    if !instance_data.is_empty() {
        let center = (bbox.min + bbox.max) * 0.5;
        let barycenter = instance_data.iter().fold(Vec2::ZERO, |acc, data| {
            acc + (Vec2::new(data.position[0], data.position[1]) - center)
        }) / instance_data.len() as f32;
        if barycenter.length() > 5.0 {
            controller.expand();
            last_view.expanded = true;
        }
    }

    Some(instance_data)
}
