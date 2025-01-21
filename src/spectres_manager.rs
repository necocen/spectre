use bevy::prelude::Resource;

use crate::{
    geometry::{Anchor, Skeleton, SpectreIter, SuperSpectre},
    utils::{Aabb, Angle, HexVec},
};

#[derive(Resource)]
pub struct SpectresManager {
    spectres: Box<SuperSpectre>,
}

impl SpectresManager {
    pub fn new() -> Self {
        let skeleton = Skeleton::new_with_anchor(5, HexVec::ZERO, Anchor::Anchor1, Angle::ZERO)
            .to_super_spectre(&Aabb::NULL);
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
            Skeleton::new_with_anchor(1, HexVec::ZERO, Anchor::Anchor1, Angle::ZERO)
                .to_super_spectre(&Aabb::NULL),
        );
        std::mem::swap(&mut self.spectres, &mut spectres);
        if spectres.level % 2 == 0 {
            tracing::info!("Expand from A");
            self.spectres = Box::new(SuperSpectre::from_child_a(*spectres));
        } else {
            tracing::info!("Expand from F");
            self.spectres = Box::new(SuperSpectre::from_child_f(*spectres));
        }
    }

    pub fn update(&mut self, bbox: &Aabb) {
        self.spectres.update_children(bbox);
    }

    pub fn spectres_in(&self, bbox: &Aabb) -> SpectreIter {
        self.spectres.iter(*bbox)
    }
}
