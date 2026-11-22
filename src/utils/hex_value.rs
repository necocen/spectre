use std::cmp::Ordering;
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
            } else if self.irrational < 0 {
                write!(f, "-")?;
            }
            if self.irrational % 2 == 0 {
                write!(f, "{}", (self.irrational / 2).unsigned_abs())?;
            } else {
                write!(f, "{}/2", self.irrational.unsigned_abs())?;
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

impl Ord for HexValue {
    fn cmp(&self, other: &Self) -> Ordering {
        let rational = i128::from(self.rational) - i128::from(other.rational);
        let irrational = i128::from(self.irrational) - i128::from(other.irrational);
        match (rational.cmp(&0), irrational.cmp(&0)) {
            (Ordering::Equal, order) | (order, Ordering::Equal) => order,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
            (Ordering::Less, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, Ordering::Less) => {
                (rational * rational).cmp(&(3 * irrational * irrational))
            }
            (Ordering::Less, Ordering::Greater) => {
                (3 * irrational * irrational).cmp(&(rational * rational))
            }
        }
    }
}

impl PartialOrd for HexValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const SQRT_3_F32: f32 = 1.7320508;
const SQRT_3_F64: f64 = 1.7320508075688772;

impl HexValue {
    /// 新しいHexValueを生成
    #[inline]
    pub const fn new(rational: i32, irrational: i32) -> Self {
        Self {
            rational,
            irrational,
        }
    }

    /// ゼロ値
    pub const ZERO: Self = Self::new(0, 0);

    // cos(30° * n) = rational/2 + irrational*√3/2
    const COS_TABLE: [Self; 12] = [
        Self::new(2, 0),  // 0°
        Self::new(0, 1),  // 30°
        Self::new(1, 0),  // 60°
        Self::new(0, 0),  // 90°
        Self::new(-1, 0), // 120°
        Self::new(0, -1), // 150°
        Self::new(-2, 0), // 180°
        Self::new(0, -1), // 210°
        Self::new(-1, 0), // 240°
        Self::new(0, 0),  // 270°
        Self::new(1, 0),  // 300°
        Self::new(0, 1),  // 330°
    ];

    // sin(30° * n) = rational/2 + irrational*√3/2
    const SIN_TABLE: [Self; 12] = [
        Self::new(0, 0),  // 0°
        Self::new(1, 0),  // 30°
        Self::new(0, 1),  // 60°
        Self::new(2, 0),  // 90°
        Self::new(0, 1),  // 120°
        Self::new(1, 0),  // 150°
        Self::new(0, 0),  // 180°
        Self::new(-1, 0), // 210°
        Self::new(0, -1), // 240°
        Self::new(-2, 0), // 270°
        Self::new(0, -1), // 300°
        Self::new(-1, 0), // 330°
    ];

    /// 正六角形の頂点に対応する角度のcos値を取得
    #[inline]
    pub const fn cos(angle: Angle) -> Self {
        Self::COS_TABLE[angle.value() as usize]
    }

    /// 正六角形の頂点に対応する角度のsin値を取得
    #[inline]
    pub const fn sin(angle: Angle) -> Self {
        Self::SIN_TABLE[angle.value() as usize]
    }

    /// f32に変換
    #[inline]
    pub fn to_f32(self) -> f32 {
        self.rational as f32 * 0.5 + self.irrational as f32 * (SQRT_3_F32 * 0.5)
    }

    #[inline]
    pub fn to_f64(self) -> f64 {
        (f64::from(self.rational) + f64::from(self.irrational) * SQRT_3_F64) * 0.5
    }
}

impl Add for HexValue {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            rational: self.rational + rhs.rational,
            irrational: self.irrational + rhs.irrational,
        }
    }
}

impl AddAssign for HexValue {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for HexValue {
    type Output = Self;

    #[inline]
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
    fn ordering_is_exact_even_when_f32_values_are_indistinguishable() {
        let a = HexValue::new(100_000_000, 0);
        let b = HexValue::new(100_000_001, 0);
        assert_eq!(a.to_f32(), b.to_f32());
        assert!(a < b);
        assert!(HexValue::new(3, -2) < HexValue::ZERO);
        assert!(HexValue::new(-3, 2) > HexValue::ZERO);
        assert!(HexValue::new(i32::MIN, i32::MIN) < HexValue::new(i32::MAX, i32::MAX));
        assert!(HexValue::new(i32::MAX, i32::MIN) < HexValue::new(i32::MIN, i32::MAX));
        assert_eq!(a.partial_cmp(&a), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn ordering_matches_real_values_for_small_coefficients() {
        for rational in -8..=8 {
            for irrational in -8..=8 {
                let value = HexValue::new(rational, irrational);
                for other in [HexValue::ZERO, HexValue::new(1, 1), HexValue::new(-1, -1)] {
                    assert_eq!(
                        value.cmp(&other),
                        value.to_f64().partial_cmp(&other.to_f64()).unwrap()
                    );
                    assert_eq!(value.cmp(&other), other.cmp(&value).reverse());
                }
            }
        }
    }

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
        assert_eq!(HexValue::new(0, 2).to_string(), "1 * √3");
        assert_eq!(HexValue::new(0, -2).to_string(), "-1 * √3");
        assert_eq!(HexValue::new(0, -1).to_string(), "-1/2 * √3");
        assert_eq!(HexValue::ZERO.to_string(), "0");
    }

    #[test]
    fn test_to_f32() {
        let value = HexValue::new(1, 1);
        let expected = 0.5 + 3.0_f32.sqrt() / 2.0;
        assert!((value.to_f32() - expected).abs() < 1e-6);
    }
}
