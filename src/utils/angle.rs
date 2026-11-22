use std::f32::consts::PI;

/// 角度を表す型（0〜11）
///
/// # Details
/// 12方向の角度を表現し、加減算は自動的にmod 12で正規化されます。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle(u8);

impl Angle {
    /// 角度0度
    pub const ZERO: Self = Self(0);

    const RADIANS: [f32; 12] = [
        0.0 * PI / 6.0,
        1.0 * PI / 6.0,
        2.0 * PI / 6.0,
        3.0 * PI / 6.0,
        4.0 * PI / 6.0,
        5.0 * PI / 6.0,
        6.0 * PI / 6.0,
        7.0 * PI / 6.0,
        8.0 * PI / 6.0,
        9.0 * PI / 6.0,
        10.0 * PI / 6.0,
        11.0 * PI / 6.0,
    ];

    /// 角度を正規化して新しいAngleを生成
    #[inline]
    pub const fn new(value: i32) -> Self {
        Self(value.rem_euclid(12) as u8)
    }

    /// 内部値を取得（0-11）
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// ラジアンに変換
    #[inline]
    pub const fn to_radians(self) -> f32 {
        Self::RADIANS[self.0 as usize]
    }

    #[inline]
    pub const fn opposite(self) -> Self {
        if self.0 >= 6 {
            Self(self.0 - 6)
        } else {
            Self(self.0 + 6)
        }
    }
}

// 角度の加算（自動的にmod 12）
impl std::ops::Add for Angle {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let sum = self.0 + rhs.0;
        Self(if sum >= 12 { sum - 12 } else { sum })
    }
}

// 角度の減算（自動的にmod 12）
impl std::ops::Sub for Angle {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        let diff = self.0 + 12 - rhs.0;
        Self(if diff >= 12 { diff - 12 } else { diff })
    }
}

impl std::ops::Neg for Angle {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(if self.0 == 0 { 0 } else { 12 - self.0 })
    }
}

// 角度の加算代入
impl std::ops::AddAssign for Angle {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// 角度の減算代入
impl std::ops::SubAssign for Angle {
    #[inline]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_constructors() {
        assert_eq!(Angle::ZERO.value(), 0);
        assert_eq!(Angle::new(0).value(), 0);
        assert_eq!(Angle::new(12).value(), 0);
        assert_eq!(Angle::new(-1).value(), 11);
        assert_eq!(Angle::new(13).value(), 1);
    }

    #[test]
    fn test_value_and_radians() {
        let angle = Angle::new(3);
        assert_eq!(angle.value(), 3);
        assert_eq!(angle.to_radians(), PI / 2.0); // 3 * PI/6 = PI/2

        let angle = Angle::new(6);
        assert_eq!(angle.value(), 6);
        assert_eq!(angle.to_radians(), PI); // 6 * PI/6 = PI
    }

    #[test]
    fn test_opposite() {
        let angle = Angle::new(1);
        assert_eq!(angle.opposite().value(), 7);

        let angle = Angle::new(7);
        assert_eq!(angle.opposite().value(), 1);

        let angle = Angle::new(0);
        assert_eq!(angle.opposite().value(), 6);
    }

    #[test]
    fn test_arithmetic() {
        // 加算
        assert_eq!((Angle::new(3) + Angle::new(4)).value(), 7);
        assert_eq!((Angle::new(8) + Angle::new(5)).value(), 1);

        // 減算
        assert_eq!((Angle::new(7) - Angle::new(4)).value(), 3);
        assert_eq!((Angle::new(2) - Angle::new(5)).value(), 9);

        // 否定
        assert_eq!((-Angle::new(3)).value(), 9);
        assert_eq!((-Angle::new(0)).value(), 0);
    }

    #[test]
    fn test_assign_operators() {
        // 加算代入
        let mut angle = Angle::new(3);
        angle += Angle::new(4);
        assert_eq!(angle.value(), 7);

        // 減算代入
        let mut angle = Angle::new(7);
        angle -= Angle::new(4);
        assert_eq!(angle.value(), 3);
    }

    #[test]
    fn test_from() {
        // From<u8>
        let angle: Angle = 3u8.into();
        assert_eq!(angle.value(), 3);
        let angle: Angle = 13u8.into();
        assert_eq!(angle.value(), 1);

        // From<i32>
        let angle: Angle = 3i32.into();
        assert_eq!(angle.value(), 3);
        let angle: Angle = (-1i32).into();
        assert_eq!(angle.value(), 11);
    }
}
