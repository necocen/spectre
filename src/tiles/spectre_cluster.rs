use crate::utils::{Aabb, Angle, HexVec};

use super::{
    Anchor, EDGE_CHAIN, MAX_CLUSTER_LEVEL, MIN_PARTIAL_CLUSTER_LEVEL, MysticCluster, MysticLike,
    Skeleton, SpectreIter, SpectreLike,
};

pub struct SpectreCluster {
    pub(super) a: SpectreLike,
    pub(super) b: SpectreLike,
    pub(super) c: SpectreLike,
    pub(super) d: SpectreLike,
    pub(super) e: SpectreLike,
    pub(super) f: SpectreLike,
    pub(super) g: SpectreLike,
    pub(super) h: MysticLike,
    level: usize,
    bbox: Aabb,
}

impl SpectreCluster {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: impl Into<SpectreLike>,
        b: impl Into<SpectreLike>,
        c: impl Into<SpectreLike>,
        d: impl Into<SpectreLike>,
        e: impl Into<SpectreLike>,
        f: impl Into<SpectreLike>,
        g: impl Into<SpectreLike>,
        h: impl Into<MysticLike>,
        level: usize,
    ) -> Self {
        let a = a.into();
        let b = b.into();
        let c = c.into();
        let d = d.into();
        let e = e.into();
        let f = f.into();
        let g = g.into();
        let h = h.into();
        assert_eq!(h.coordinate(Anchor::Anchor1), a.coordinate(Anchor::Anchor1));
        assert_eq!(a.coordinate(Anchor::Anchor3), b.coordinate(Anchor::Anchor1));
        assert_eq!(b.coordinate(Anchor::Anchor4), c.coordinate(Anchor::Anchor2));
        assert_eq!(c.coordinate(Anchor::Anchor3), d.coordinate(Anchor::Anchor1));
        assert_eq!(d.coordinate(Anchor::Anchor3), e.coordinate(Anchor::Anchor1));
        assert_eq!(e.coordinate(Anchor::Anchor4), f.coordinate(Anchor::Anchor2));
        assert_eq!(f.coordinate(Anchor::Anchor3), g.coordinate(Anchor::Anchor1));
        assert_eq!(g.coordinate(Anchor::Anchor4), h.coordinate(Anchor::Anchor4));

        // Bounds include both materialized children and unloaded skeletons.
        let mut bbox = Aabb::NULL;
        bbox = bbox.union(&a.bbox());
        bbox = bbox.union(&b.bbox());
        bbox = bbox.union(&c.bbox());
        bbox = bbox.union(&d.bbox());
        bbox = bbox.union(&e.bbox());
        bbox = bbox.union(&f.bbox());
        bbox = bbox.union(&g.bbox());
        bbox = bbox.union(&h.bbox());

        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            level,
            bbox,
        }
    }

    pub fn with_anchor(
        anchor: Anchor,
        coordinate: impl Into<HexVec>,
        edge_direction: impl Into<Angle>,
        level: usize,
    ) -> Self {
        assert!(
            (1..=MAX_CLUSTER_LEVEL).contains(&level),
            "unsupported cluster level: {level}"
        );

        let (start_idx, start_anchor) = match anchor {
            Anchor::Anchor1 => (6, Anchor::Anchor3), // g から開始
            Anchor::Anchor2 => (3, Anchor::Anchor2), // d から開始
            Anchor::Anchor3 => (1, Anchor::Anchor3), // b から開始
            Anchor::Anchor4 => (0, Anchor::Anchor2), // a から開始
        };

        let edge_direction: Angle = edge_direction.into();
        let coordinate: HexVec = coordinate.into();

        let first = SpectreLike::with_anchor(start_anchor, coordinate, edge_direction, level - 1);
        Self::from_children(Self::connected_children(first, start_idx), level)
    }

    fn connected_children(first: SpectreLike, start: usize) -> [SpectreLike; 8] {
        let mut children: [Option<SpectreLike>; 8] = std::array::from_fn(|_| None);
        children[start] = Some(first);
        for step in 0..7 {
            let index = (start + step) % 8;
            let (from, to) = EDGE_CHAIN[index];
            let next = children[index]
                .as_ref()
                .unwrap()
                .connected_spectre_like(from, to);
            children[(index + 1) % 8] = Some(next);
        }
        children.map(Option::unwrap)
    }

    fn from_children([a, b, c, d, e, f, g, h]: [SpectreLike; 8], level: usize) -> Self {
        let h = h.into_mystic_like();
        Self::new(a, b, c, d, e, f, g, h, level)
    }

    pub fn with_child_a(a: SpectreCluster) -> Self {
        Self::with_child_at(a, 0)
    }

    pub fn with_child_f(f: SpectreCluster) -> Self {
        Self::with_child_at(f, 5)
    }

    fn with_child_at(child: SpectreCluster, index: usize) -> Self {
        let level = child.level() + 1;
        assert!(
            level <= MAX_CLUSTER_LEVEL,
            "unsupported cluster level: {level}"
        );
        let mut children = Self::connected_children(child.to_skeleton().into(), index);
        children[index] = child.into();
        Self::from_children(children, level)
    }

    pub fn connected_cluster(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreCluster {
        // 新しいSpectreの角度を計算
        // levelによって頂点を合わせる場合に接合する辺の選びかたが変わる
        let angle = if self.level % 2 == 1 {
            // 奇数番目のlevelでは新しいClusterを辺が密着するまで時計回りに回転させる
            self.edge_direction_into(from_anchor).opposite()
        } else {
            // 偶数番目のlevelでは反時計回りに回転させる
            let rotation = self.edge_direction_from(to_anchor)
                - self.edge_direction_into(to_anchor).opposite();
            self.edge_direction_from(from_anchor) + rotation
        };

        SpectreCluster::with_anchor(to_anchor, self.coordinate(from_anchor), angle, self.level)
    }

    pub fn into_mystic_cluster(self) -> MysticCluster {
        MysticCluster::new(
            self.a, self.b, self.c, self.d, self.f, self.g, self.h, self.level,
        )
    }

    pub fn to_skeleton(&self) -> Skeleton {
        Skeleton::with_anchor(
            Anchor::Anchor1,
            self.coordinate(Anchor::Anchor1),
            self.edge_direction_from(Anchor::Anchor1),
            self.level,
            Some(self.bbox()),
        )
    }

    pub fn update(&mut self, bbox: &Aabb) {
        if self.level < MIN_PARTIAL_CLUSTER_LEVEL {
            return;
        }
        self.a.update(bbox);
        self.b.update(bbox);
        self.c.update(bbox);
        self.d.update(bbox);
        self.e.update(bbox);
        self.f.update(bbox);
        self.g.update(bbox);
        self.h.update(bbox);
        let mut bbox = Aabb::NULL;
        bbox = bbox.union(&self.a.bbox());
        bbox = bbox.union(&self.b.bbox());
        bbox = bbox.union(&self.c.bbox());
        bbox = bbox.union(&self.d.bbox());
        bbox = bbox.union(&self.e.bbox());
        bbox = bbox.union(&self.f.bbox());
        bbox = bbox.union(&self.g.bbox());
        bbox = bbox.union(&self.h.bbox());
        self.bbox = bbox;
    }

    pub fn spectres_in(&self, bbox: Aabb) -> SpectreIter<'_> {
        SpectreIter::new(self, bbox)
    }

    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.g.coordinate(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.coordinate(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.coordinate(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.coordinate(Anchor::Anchor2),
        }
    }

    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.edge_direction_from(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.edge_direction_from(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.edge_direction_from(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.edge_direction_from(Anchor::Anchor2),
        }
    }

    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.edge_direction_into(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.edge_direction_into(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.edge_direction_into(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.edge_direction_into(Anchor::Anchor2),
        }
    }

    pub fn bbox(&self) -> Aabb {
        self.bbox
    }

    pub fn level(&self) -> usize {
        self.level
    }
}
