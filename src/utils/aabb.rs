use bevy::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub const NULL: Aabb = Aabb {
        min: Vec2::splat(f32::INFINITY),
        max: Vec2::splat(f32::NEG_INFINITY),
    };

    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        if (self == &Self::NULL) || (other == &Self::NULL) {
            return Self::NULL;
        }
        let min = Vec2::new(self.min.x.max(other.min.x), self.min.y.max(other.min.y));
        let max = Vec2::new(self.max.x.min(other.max.x), self.max.y.min(other.max.y));
        Aabb::from_min_max(min, max)
    }

    pub fn union(&self, other: &Self) -> Self {
        if self == &Self::NULL {
            return *other;
        } else if other == &Self::NULL {
            return *self;
        }
        let min = Vec2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y));
        let max = Vec2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y));
        Aabb::from_min_max(min, max)
    }

    pub fn has_intersection(&self, other: &Self) -> bool {
        !self.intersection(other).is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.min.x <= point.x
            && point.x <= self.max.x
            && self.min.y <= point.y
            && point.y <= self.max.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let bbox1 = Aabb::from_min_max(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
        assert_eq!(bbox1.min, Vec2::new(1.0, 2.0));
        assert_eq!(bbox1.max, Vec2::new(3.0, 4.0));

        let bbox2 = Aabb::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(bbox2.min, Vec2::new(1.0, 2.0));
        assert_eq!(bbox2.max, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_intersection() {
        let bbox1 = Aabb::new(0.0, 0.0, 2.0, 2.0);
        let bbox2 = Aabb::new(1.0, 1.0, 3.0, 3.0);
        let intersection = bbox1.intersection(&bbox2);
        assert_eq!(intersection.min, Vec2::new(1.0, 1.0));
        assert_eq!(intersection.max, Vec2::new(2.0, 2.0));

        // 交差しない場合
        let bbox3 = Aabb::new(3.0, 3.0, 4.0, 4.0);
        let no_intersection = bbox1.intersection(&bbox3);
        assert!(no_intersection.is_empty());
    }

    #[test]
    fn test_union() {
        let bbox1 = Aabb::new(0.0, 0.0, 2.0, 2.0);
        let bbox2 = Aabb::new(1.0, 1.0, 3.0, 3.0);
        let union = bbox1.union(&bbox2);
        assert_eq!(union.min, Vec2::new(0.0, 0.0));
        assert_eq!(union.max, Vec2::new(3.0, 3.0));
    }

    #[test]
    fn test_is_empty() {
        // 正常なAABB
        let bbox1 = Aabb::new(0.0, 0.0, 2.0, 2.0);
        assert!(!bbox1.is_empty());

        // 空のAABB（x方向）
        let bbox2 = Aabb::new(2.0, 0.0, 1.0, 2.0);
        assert!(bbox2.is_empty());

        // 空のAABB（y方向）
        let bbox3 = Aabb::new(0.0, 2.0, 2.0, 1.0);
        assert!(bbox3.is_empty());

        // 点
        let bbox4 = Aabb::new(1.0, 1.0, 1.0, 1.0);
        assert!(bbox4.is_empty());
    }

    #[test]
    fn test_null() {
        // NULLの値が正しく定義されているか
        assert_eq!(Aabb::NULL.min, Vec2::splat(f32::INFINITY));
        assert_eq!(Aabb::NULL.max, Vec2::splat(f32::NEG_INFINITY));
        assert!(Aabb::NULL.is_empty());

        // NULLとの交差演算
        let bbox = Aabb::new(0.0, 0.0, 2.0, 2.0);
        assert_eq!(bbox.intersection(&Aabb::NULL), Aabb::NULL);
        assert_eq!(Aabb::NULL.intersection(&bbox), Aabb::NULL);
        assert_eq!(Aabb::NULL.intersection(&Aabb::NULL), Aabb::NULL);

        // NULLとの合併演算
        assert_eq!(bbox.union(&Aabb::NULL), bbox);
        assert_eq!(Aabb::NULL.union(&bbox), bbox);
        assert_eq!(Aabb::NULL.union(&Aabb::NULL), Aabb::NULL);
    }
}
