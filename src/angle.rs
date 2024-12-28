use std::f32::consts::PI;

/// 角度を表す型（0〜11）
///
/// # Details
/// 12方向の角度を表現し、加減算は自動的にmod 12で正規化されます。
#[derive(Debug, Clone, Copy)]
pub struct Angle(u8);

impl Angle {
    /// 角度0度
    pub const ZERO: Self = Self(0);
    /// 反対方向（180度）
    pub const OPPOSITE: Self = Self(6);

    /// 角度を正規化して新しいAngleを生成
    pub const fn new(value: i32) -> Self {
        Self(value.rem_euclid(12) as u8)
    }

    /// 内部値を取得（0-11）
    pub fn value(self) -> u8 {
        self.0
    }

    /// ラジアンに変換
    pub fn to_radians(self) -> f32 {
        self.0 as f32 * PI / 6.0
    }
}

// 角度の加算（自動的にmod 12）
impl std::ops::Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 as i32 + rhs.0 as i32)
    }
}

// 角度の減算（自動的にmod 12）
impl std::ops::Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 as i32 - rhs.0 as i32)
    }
}

// 角度の加算代入
impl std::ops::AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// 角度の減算代入
impl std::ops::SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// u8からの変換
impl From<u8> for Angle {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

// i32からの変換
impl From<i32> for Angle {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}
