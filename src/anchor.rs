/// タイルの接続点を表す
#[derive(Debug, Clone, Copy)]
pub enum Anchor {
    /// 基準となる接続点（インデックス0）
    Anchor1,
    /// 2番目の接続点（インデックス4）
    Anchor2,
    /// 3番目の接続点（インデックス6）
    Anchor3,
    /// 4番目の接続点（インデックス8）
    Anchor4,
}

impl Anchor {
    /// 各アンカーの頂点配列におけるインデックス
    const ANCHOR1_INDEX: usize = 0;
    const ANCHOR2_INDEX: usize = 4;
    const ANCHOR3_INDEX: usize = 6;
    const ANCHOR4_INDEX: usize = 8;

    /// アンカーに対応する頂点配列のインデックスを取得する
    pub fn index(&self) -> usize {
        match self {
            Anchor::Anchor1 => Self::ANCHOR1_INDEX,
            Anchor::Anchor2 => Self::ANCHOR2_INDEX,
            Anchor::Anchor3 => Self::ANCHOR3_INDEX,
            Anchor::Anchor4 => Self::ANCHOR4_INDEX,
        }
    }
}
