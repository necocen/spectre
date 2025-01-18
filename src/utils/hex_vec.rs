use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use super::{Angle, HexValue};

/// 正六角形のタイリングに適した2次元ベクトル
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexVec {
    pub x: HexValue,
    pub y: HexValue,
}

impl HexVec {
    /// 新しいHexVecを生成
    pub const fn new(x: HexValue, y: HexValue) -> Self {
        Self { x, y }
    }

    /// ゼロベクトル
    pub const ZERO: Self = Self::new(HexValue::ZERO, HexValue::ZERO);

    /// Vec2に変換
    pub fn to_vec2(self) -> bevy::math::Vec2 {
        bevy::math::Vec2::new(self.x.to_f32(), self.y.to_f32())
    }

    /// 点を指定された角度だけ回転する
    /// すべてのベクトルが回転できるわけではないので注意
    ///
    /// # Arguments
    /// * `center` - 回転の中心点
    /// * `angle` - 回転角度
    pub fn rotate(self, center: Self, angle: Angle) -> Self {
        // 点を原点に移動
        let relative = self - center;

        // 回転行列を使って回転
        let cos = HexValue::cos(angle);
        let sin = HexValue::sin(angle);

        let x = HexValue::new(
            (3 * cos.irrational * relative.x.irrational + cos.rational * relative.x.rational
                - 3 * sin.irrational * relative.y.irrational
                - sin.rational * relative.y.rational)
                / 2,
            (cos.rational * relative.x.irrational + cos.irrational * relative.x.rational
                - sin.rational * relative.y.irrational
                - sin.irrational * relative.y.rational)
                / 2,
        );
        let y = HexValue::new(
            (3 * sin.irrational * relative.x.irrational
                + sin.rational * relative.x.rational
                + 3 * cos.irrational * relative.y.irrational
                + cos.rational * relative.y.rational)
                / 2,
            (sin.rational * relative.x.irrational
                + sin.irrational * relative.x.rational
                + cos.rational * relative.y.irrational
                + cos.irrational * relative.y.rational)
                / 2,
        );

        let rotated = Self::new(x, y);

        // 中心点を加算して元の座標系に戻す
        center + rotated
    }
}

impl Add for HexVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for HexVec {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for HexVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for HexVec {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for HexVec {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i32> for HexVec {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<bevy::math::Vec2> for HexVec {
    fn from(v: bevy::math::Vec2) -> Self {
        // 注意: これは近似的な変換です
        // 実際の使用時は直接HexVecを構築することを推奨
        Self {
            x: HexValue::new((v.x * 2.0) as i32, 0),
            y: HexValue::new((v.y * 2.0) as i32, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_unit_vector() {
        // 基準となる点 (1, 0)
        let point = HexVec::new(HexValue::new(2, 0), HexValue::ZERO);
        let center = HexVec::ZERO;

        // 30度回転のテスト
        let angle = Angle::new(1);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(0, 1)); // √3/2
        assert_eq!(rotated.y, HexValue::new(1, 0)); // 1/2

        // 60度回転のテスト
        let angle = Angle::new(2);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(1, 0)); // 1/2
        assert_eq!(rotated.y, HexValue::new(0, 1)); // √3/2

        // 90度回転のテスト
        let angle = Angle::new(3);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::ZERO); // 0
        assert_eq!(rotated.y, HexValue::new(2, 0)); // 1

        // 120度回転のテスト
        let angle = Angle::new(4);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(-1, 0)); // -1/2
        assert_eq!(rotated.y, HexValue::new(0, 1)); // √3/2

        // 150度回転のテスト
        let angle = Angle::new(5);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(0, -1)); // -√3/2
        assert_eq!(rotated.y, HexValue::new(1, 0)); // 1/2

        // 180度回転のテスト
        let angle = Angle::new(6);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(-2, 0)); // -1
        assert_eq!(rotated.y, HexValue::ZERO); // 0

        // 270度回転のテスト
        let angle = Angle::new(9);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::ZERO); // 0
        assert_eq!(rotated.y, HexValue::new(-2, 0)); // -1

        // 330度回転のテスト
        let angle = Angle::new(11);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(0, 1)); // √3/2
        assert_eq!(rotated.y, HexValue::new(-1, 0)); // -1/2
    }

    #[test]
    fn test_rotate_special_vectors() {
        let center = HexVec::ZERO;

        // (1/2, √3/2)の60度回転 -> (-1/2, √3/2)
        let point = HexVec::new(HexValue::new(1, 0), HexValue::new(0, 1));
        let angle = Angle::new(2);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(-1, 0)); // -1/2
        assert_eq!(rotated.y, HexValue::new(0, 1)); // √3/2

        // (√3/2, 1/2)の90度回転 -> (-1/2, √3/2)
        let point = HexVec::new(HexValue::new(0, 1), HexValue::new(1, 0));
        let angle = Angle::new(3);
        let rotated = point.rotate(center, angle);
        assert_eq!(rotated.x, HexValue::new(-1, 0)); // -1/2
        assert_eq!(rotated.y, HexValue::new(0, 1)); // √3/2
    }
}
