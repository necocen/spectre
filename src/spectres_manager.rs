use bevy::prelude::Resource;

use crate::{
    geometry::{Anchor, Skeleton, SpectreCluster, SpectreIter},
    utils::{Aabb, Angle, HexVec},
};

#[derive(Resource)]
pub struct SpectresManager {
    spectres: Box<SpectreCluster>,
}

impl SpectresManager {
    pub fn new() -> Self {
        let skeleton = Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 5)
            .to_spectre_cluster(&Aabb::NULL);
        let spectres = Box::new(skeleton);
        Self { spectres }
    }

    pub fn expand(&mut self) {
        if self.spectres.level > 18 {
            tracing::warn!("Cannot expand more");
            return;
        }

        // 現在のSuperSpectreをAまたはFとして上位のSuperSpectreを生成する
        let mut spectres = Box::new(
            Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 1)
                .to_spectre_cluster(&Aabb::NULL),
        );
        std::mem::swap(&mut self.spectres, &mut spectres);
        if spectres.level % 2 == 0 {
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

    pub fn spectres_in(&self, bbox: &Aabb) -> SpectreIter {
        self.spectres.spectres_in(*bbox)
    }
}
