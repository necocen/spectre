use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use mikage::glam::Vec2;

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
    #[inline]
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x.to_f32(), self.y.to_f32())
    }

    /// 点を指定された角度だけ回転する
    /// すべてのベクトルが回転できるわけではないので注意
    ///
    /// # Arguments
    /// * `center` - 回転の中心点
    /// * `angle` - 回転角度
    #[inline]
    pub fn rotate(self, center: Self, angle: Angle) -> Self {
        // 点を原点に移動
        let relative = self - center;
        let xr = i64::from(relative.x.rational);
        let xi = i64::from(relative.x.irrational);
        let yr = i64::from(relative.y.rational);
        let yi = i64::from(relative.y.irrational);

        // 12方向それぞれで cos, sin の非ゼロ項だけを計算し、i64 経由で中間オーバーフローを防ぐ
        let (rxr, rxi, ryr, ryi) = match angle.value() {
            0 => (xr, xi, yr, yi),
            1 => (
                (3 * xi - yr) / 2,
                (xr - yi) / 2,
                (xr + 3 * yi) / 2,
                (xi + yr) / 2,
            ),
            2 => (
                (xr - 3 * yi) / 2,
                (xi - yr) / 2,
                (3 * xi + yr) / 2,
                (xr + yi) / 2,
            ),
            3 => (-yr, -yi, xr, xi),
            4 => (
                (-xr - 3 * yi) / 2,
                (-xi - yr) / 2,
                (3 * xi - yr) / 2,
                (xr - yi) / 2,
            ),
            5 => (
                (-3 * xi - yr) / 2,
                (-xr - yi) / 2,
                (xr - 3 * yi) / 2,
                (xi - yr) / 2,
            ),
            6 => (-xr, -xi, -yr, -yi),
            7 => (
                (-3 * xi + yr) / 2,
                (-xr + yi) / 2,
                (-xr - 3 * yi) / 2,
                (-xi - yr) / 2,
            ),
            8 => (
                (-xr + 3 * yi) / 2,
                (-xi + yr) / 2,
                (-3 * xi - yr) / 2,
                (-xr - yi) / 2,
            ),
            9 => (yr, yi, -xr, -xi),
            10 => (
                (xr + 3 * yi) / 2,
                (xi + yr) / 2,
                (-3 * xi + yr) / 2,
                (-xr + yi) / 2,
            ),
            11 => (
                (3 * xi + yr) / 2,
                (xr + yi) / 2,
                (-xr + 3 * yi) / 2,
                (-xi + yr) / 2,
            ),
            _ => unreachable!(),
        };

        let rotated = Self::new(
            HexValue::new(rxr as i32, rxi as i32),
            HexValue::new(ryr as i32, ryi as i32),
        );

        // 中心点を加算して元の座標系に戻す
        center + rotated
    }
}

impl Add for HexVec {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for HexVec {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for HexVec {
    type Output = Self;

    #[inline]
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

impl Div<i32> for HexVec {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::fmt::Display for HexVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
