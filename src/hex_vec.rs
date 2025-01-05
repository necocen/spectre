use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::hex_value::HexValue;

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
