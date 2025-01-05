use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::angle::Angle;

/// 正六角形のタイリングに適した実数値を表現する型
/// i/2 + j*√3/2 の形で値を保持する
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexValue {
    /// 有理数部分の分子（分母は2で固定）
    rational: i32,
    /// √3の係数の分子（分母は2で固定）
    irrational: i32,
}

impl std::fmt::Display for HexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/2 + 2 * {}/√3", self.rational, self.irrational)
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
