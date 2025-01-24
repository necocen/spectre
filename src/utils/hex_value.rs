use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use super::Angle;

/// 正六角形のタイリングに適した実数値を表現する型
/// i/2 + j*√3/2 の形で値を保持する
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct HexValue {
    /// 有理数部分の分子（分母は2で固定）
    pub rational: i32,
    /// √3の係数の分子（分母は2で固定）
    pub irrational: i32,
}

impl std::fmt::Debug for HexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rational == 0 && self.irrational == 0 {
            return write!(f, "0");
        }
        if self.rational != 0 {
            if self.rational % 2 == 0 {
                write!(f, "{}", self.rational / 2)?;
            } else {
                write!(f, "{}/2", self.rational)?;
            }
        }
        if self.irrational != 0 {
            if self.rational != 0 {
                if self.irrational > 0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            }
            if self.irrational % 2 == 0 {
                write!(f, "{}", i32::abs(self.irrational / 2))?;
            } else {
                write!(f, "{}/2", i32::abs(self.irrational))?;
            }
            write!(f, " * √3")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for HexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl HexValue {
    /// 新しいHexValueを生成
    pub const fn new(rational: i32, irrational: i32) -> Self {
        Self {
            rational,
            irrational,
        }
    }

    /// ゼロ値
    pub const ZERO: Self = Self::new(0, 0);

    /// 正六角形の頂点に対応する角度のcos値を取得
    pub fn cos(angle: Angle) -> Self {
        // cos(30° * n) = rational/2 + irrational*√3/2
        match angle.value() % 12 {
            0 => Self::new(2, 0),  // cos(0°) = 1
            1 => Self::new(0, 1),  // cos(30°) = √3/2
            2 => Self::new(1, 0),  // cos(60°) = 1/2
            3 => Self::new(0, 0),  // cos(90°) = 0
            4 => Self::new(-1, 0), // cos(120°) = -1/2
            5 => Self::new(0, -1), // cos(150°) = -√3/2
            6 => Self::new(-2, 0), // cos(180°) = -1
            7 => Self::new(0, -1), // cos(210°) = -√3/2
            8 => Self::new(-1, 0), // cos(240°) = -1/2
            9 => Self::new(0, 0),  // cos(270°) = 0
            10 => Self::new(1, 0), // cos(300°) = 1/2
            11 => Self::new(0, 1), // cos(330°) = √3/2
            _ => unreachable!(),
        }
    }

    /// 正六角形の頂点に対応する角度のsin値を取得
    pub fn sin(angle: Angle) -> Self {
        // sin(30° * n) = rational/2 + irrational*√3/2
        match angle.value() % 12 {
            0 => Self::new(0, 0),   // sin(0°) = 0
            1 => Self::new(1, 0),   // sin(30°) = 1/2
            2 => Self::new(0, 1),   // sin(60°) = √3/2
            3 => Self::new(2, 0),   // sin(90°) = 1
            4 => Self::new(0, 1),   // sin(120°) = √3/2
            5 => Self::new(1, 0),   // sin(150°) = 1/2
            6 => Self::new(0, 0),   // sin(180°) = 0
            7 => Self::new(-1, 0),  // sin(210°) = -1/2
            8 => Self::new(0, -1),  // sin(240°) = -√3/2
            9 => Self::new(-2, 0),  // sin(270°) = -1
            10 => Self::new(0, -1), // sin(300°) = -√3/2
            11 => Self::new(-1, 0), // sin(330°) = -1/2
            _ => unreachable!(),
        }
    }

    /// f32に変換
    pub fn to_f32(self) -> f32 {
        self.rational as f32 / 2.0 + self.irrational as f32 * 3.0_f32.sqrt() / 2.0
    }
}

impl Add for HexValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            rational: self.rational + rhs.rational,
            irrational: self.irrational + rhs.irrational,
        }
    }
}

impl AddAssign for HexValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for HexValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            rational: self.rational - rhs.rational,
            irrational: self.irrational - rhs.irrational,
        }
    }
}

impl SubAssign for HexValue {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for HexValue {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            rational: -self.rational,
            irrational: -self.irrational,
        }
    }
}

impl Mul<i32> for HexValue {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            rational: self.rational * rhs,
            irrational: self.irrational * rhs,
        }
    }
}

impl Div<i32> for HexValue {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self {
            rational: self.rational / rhs,
            irrational: self.irrational / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let value = HexValue::new(1, 2);
        assert_eq!(value.rational, 1);
        assert_eq!(value.irrational, 2);

        assert_eq!(HexValue::ZERO.rational, 0);
        assert_eq!(HexValue::ZERO.irrational, 0);
    }

    #[test]
    fn test_cos() {
        // 0度 = cos(0°) = 1
        let cos0 = HexValue::cos(Angle::new(0));
        assert_eq!(cos0, HexValue::new(2, 0));
        assert!((cos0.to_f32() - 1.0).abs() < 1e-6);

        // 60度 = cos(60°) = 1/2
        let cos60 = HexValue::cos(Angle::new(2));
        assert_eq!(cos60, HexValue::new(1, 0));
        assert!((cos60.to_f32() - 0.5).abs() < 1e-6);

        // 90度 = cos(90°) = 0
        let cos90 = HexValue::cos(Angle::new(3));
        assert_eq!(cos90, HexValue::new(0, 0));
        assert!((cos90.to_f32() - 0.0).abs() < 1e-6);

        // 180度 = cos(180°) = -1
        let cos180 = HexValue::cos(Angle::new(6));
        assert_eq!(cos180, HexValue::new(-2, 0));
        assert!((cos180.to_f32() - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_sin() {
        // 0度 = sin(0°) = 0
        let sin0 = HexValue::sin(Angle::new(0));
        assert_eq!(sin0, HexValue::new(0, 0));
        assert!((sin0.to_f32() - 0.0).abs() < 1e-6);

        // 60度 = sin(60°) = √3/2
        let sin60 = HexValue::sin(Angle::new(2));
        assert_eq!(sin60, HexValue::new(0, 1));
        assert!((sin60.to_f32() - (3.0_f32.sqrt() / 2.0)).abs() < 1e-6);

        // 90度 = sin(90°) = 1
        let sin90 = HexValue::sin(Angle::new(3));
        assert_eq!(sin90, HexValue::new(2, 0));
        assert!((sin90.to_f32() - 1.0).abs() < 1e-6);

        // 180度 = sin(180°) = 0
        let sin180 = HexValue::sin(Angle::new(6));
        assert_eq!(sin180, HexValue::new(0, 0));
        assert!((sin180.to_f32() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_arithmetic() {
        // 加算
        let a = HexValue::new(1, 2);
        let b = HexValue::new(3, 4);
        assert_eq!(a + b, HexValue::new(4, 6));

        // 減算
        assert_eq!(a - b, HexValue::new(-2, -2));

        // 逆
        assert_eq!(-a, HexValue::new(-1, -2));

        // 乗算（スカラー）
        assert_eq!(a * 3, HexValue::new(3, 6));

        // 除算（スカラー）
        assert_eq!(a / 2, HexValue::new(0, 1));
        assert_eq!((a + b) / 2, HexValue::new(2, 3));
    }

    #[test]
    fn test_assign_operators() {
        // 加算代入
        let mut a = HexValue::new(1, 2);
        a += HexValue::new(3, 4);
        assert_eq!(a, HexValue::new(4, 6));

        // 減算代入
        let mut b = HexValue::new(3, 4);
        b -= HexValue::new(1, 2);
        assert_eq!(b, HexValue::new(2, 2));
    }

    #[test]
    fn test_display() {
        assert_eq!(HexValue::new(1, 2).to_string(), "1/2 + 1 * √3");
        assert_eq!(HexValue::new(-1, -2).to_string(), "-1/2 - 1 * √3");
        assert_eq!(HexValue::ZERO.to_string(), "0");
    }

    #[test]
    fn test_to_f32() {
        let value = HexValue::new(1, 1);
        let expected = 0.5 + 3.0_f32.sqrt() / 2.0;
        assert!((value.to_f32() - expected).abs() < 1e-6);
    }
}
