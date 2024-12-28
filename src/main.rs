use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, (setup_camera, setup_tiles))
        .add_systems(Update, camera_movement_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn((Camera2d, Msaa::Sample4))
        .insert(CameraController::default());
}

/// タイルを描画する
///
/// # Arguments
/// * `commands` - Bevyのコマンドバッファ
/// * `spectre` - 描画するタイル
/// * `index` - タイルのインデックス（未使用）
///
/// # Details
/// タイルの頂点から多角形を生成し、angleに応じた色で描画します。
/// 色はHSVカラーモデルを使用し、以下のように決定されます：
/// - 色相（H）：angleを30度ごとに割り当て（0-360度）
/// - 彩度（S）：80%で固定
/// - 明度（V）：80%で固定
fn spawn_tile(commands: &mut Commands, spectre: &Spectre) {
    // タイルの頂点から多角形を生成
    let polygon = shapes::Polygon {
        points: spectre.points.to_vec(),
        closed: true,
    };

    // angleから色相を計算（30度ごと）
    let hue = spectre.angle.value() as f32 * 30.0;
    let saturation =
        (spectre.anchor(Anchor::Anchor1).length() / spectre.size * 2.0).sin() * 0.25 + 0.75;
    println!(
        "angle: {}, length: {}, hue: {}, saturation: {}",
        spectre.angle.value(),
        spectre.anchor(Anchor::Anchor1).length(),
        hue,
        saturation
    );
    // HSVからRGBに変換（彩度80%、明度80%）
    let color = Color::hsl(hue, saturation, 0.8);

    // タイルのエンティティを生成
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&polygon),
            ..default()
        },
        Fill::color(color),
    ));
}

/// タイルを生成して配置する
///
/// # Arguments
/// * `commands` - Bevyのコマンドバッファ
/// * `tiles` - 生成済みのタイル配列
/// * `connections` - タイルの接続情報：(接続元タイルのインデックス, 接続元アンカー, 接続先アンカー)
/// * `start_index` - 生成するタイルの開始インデックス（色分け用）
///
/// # Details
/// 指定された接続情報に基づいて新しいタイルを生成し、既存のタイルに接続します。
/// 生成したタイルは`tiles`に追加され、同時に描画用のエンティティも生成されます。
/// `start_index`は色分けのために使用され、タイルの階層を視覚的に表現します。
fn place_connected_tiles(
    commands: &mut Commands,
    tiles: &mut Vec<Spectre>,
    connections: &[(usize, Anchor, Anchor)],
) {
    for (from_idx, from_anchor, to_anchor) in connections {
        let tile = tiles[*from_idx].adjacent_anchor(*from_anchor, *to_anchor);
        let tile_clone = tile.clone();
        tiles.push(tile);
        spawn_tile(commands, &tile_clone);
    }
}

/// タイルを配置するシステム
///
/// # Details
/// タイルを以下の手順で配置します：
/// 1. 中心タイル：原点に配置（角度0度）
/// 2. 1段目のタイル：中心タイルから4方向に接続
/// 3. 2段目のタイル：1段目のタイルから外側に接続
/// 4. 中心タイルを最後に再描画して最前面に表示
fn setup_tiles(mut commands: Commands) {
    // タイルのサイズを設定
    const TILE_SIZE: f32 = 40.0;

    // 中心となるタイルを生成（原点に配置、角度0度）
    let center =
        Spectre::new_with_anchor(Vec2::new(0.0, 0.0), Anchor::Anchor1, TILE_SIZE, Angle::ZERO);

    spawn_tile(&mut commands, &center);
    let mut tiles = vec![center.clone()];

    // 1段目：中心タイルから4方向に接続
    let inner_connections = [
        // (接続元インデックス, 接続元アンカー, 接続先アンカー)
        (0, Anchor::Anchor1, Anchor::Anchor1), // A
        (1, Anchor::Anchor3, Anchor::Anchor1), // F
        (2, Anchor::Anchor4, Anchor::Anchor2), // B
        (3, Anchor::Anchor3, Anchor::Anchor1), // C
        (4, Anchor::Anchor3, Anchor::Anchor1), // G
        (5, Anchor::Anchor4, Anchor::Anchor2), // D
        (6, Anchor::Anchor3, Anchor::Anchor1), // E
    ];
    place_connected_tiles(&mut commands, &mut tiles, &inner_connections);
}

/// カメラの移動を制御するコンポーネント
///
/// # Details
/// マウスのドラッグ操作によってカメラを移動させる機能を提供します。
/// 左クリックを押下している間、マウスの移動に合わせてカメラが移動します。
#[derive(Component)]
struct CameraController {
    /// カメラの移動速度（ピクセル単位）
    pub speed: f32,
    /// ドラッグ中かどうかのフラグ
    pub dragging: bool,
    /// 前フレームでのマウス座標
    pub last_position: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 1.0,                // 1ピクセルのマウス移動で1ピクセルのカメラ移動
            dragging: false,           // 初期状態ではドラッグしていない
            last_position: Vec2::ZERO, // 初期位置は原点
        }
    }
}

/// カメラの移動を制御するシステム
///
/// # Arguments
/// * `windows` - ウィンドウの情報を取得するためのクエリ
/// * `mouse_input` - マウスの入力状態
/// * `query` - カメラのTransformとControllerを取得するクエリ
///
/// # Details
/// マウスの左クリックドラッグでカメラを移動させます：
/// 1. 左クリック押下でドラッグ開始
/// 2. ドラッグ中はマウスの移動量に応じてカメラを移動
/// 3. 左クリック解放でドラッグ終了
fn camera_movement_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    // プライマリウィンドウを取得
    let window = windows.single();

    // カメラエンティティを処理
    for (mut transform, mut controller) in query.iter_mut() {
        // 1. ドラッグ開始：左クリック押下時
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Some(pos) = window.cursor_position() {
                controller.dragging = true;
                controller.last_position = pos;
            }
        }

        // 2. ドラッグ終了：左クリック解放時
        if mouse_input.just_released(MouseButton::Left) {
            controller.dragging = false;
        }

        // 3. カメラ移動：ドラッグ中
        if controller.dragging {
            if let Some(pos) = window.cursor_position() {
                // マウスの移動量を計算
                let delta = pos - controller.last_position;

                // カメラの位置を更新
                // X軸：マウスの移動と逆方向に移動
                transform.translation.x -= delta.x * controller.speed;
                // Y軸：マウスの移動と逆方向に移動（座標系の違いを調整）
                transform.translation.y += delta.y * controller.speed;

                // 次のフレームのために現在位置を保存
                controller.last_position = pos;
            }
        }
    }
}

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
    fn index(&self) -> usize {
        match self {
            Anchor::Anchor1 => Self::ANCHOR1_INDEX,
            Anchor::Anchor2 => Self::ANCHOR2_INDEX,
            Anchor::Anchor3 => Self::ANCHOR3_INDEX,
            Anchor::Anchor4 => Self::ANCHOR4_INDEX,
        }
    }
}

/// タイルの形状を表す
#[derive(Clone)]
struct Spectre {
    /// 辺の長さ
    size: f32,
    /// タイルの回転角度
    angle: Angle,
    /// タイルを構成する頂点の座標
    points: [Vec2; Spectre::VERTEX_COUNT],
}

/// 角度を表す型（0〜11）
///
/// # Details
/// 12方向の角度を表現し、加減算は自動的にmod 12で正規化されます。
#[derive(Debug, Clone, Copy)]
struct Angle(u8);

impl Angle {
    /// 角度0度
    const ZERO: Self = Self(0);
    /// 反対方向（180度）
    const OPPOSITE: Self = Self(6);

    /// 角度を正規化して新しいAngleを生成
    const fn new(value: i32) -> Self {
        Self(value.rem_euclid(12) as u8)
    }

    /// 内部値を取得（0-11）
    pub fn value(self) -> u8 {
        self.0
    }

    /// ラジアンに変換
    fn to_radians(self) -> f32 {
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

impl Spectre {
    /// 頂点数
    const VERTEX_COUNT: usize = 14;
    /// 各頂点から反時計回りに進む辺の角度（0〜ANGLE_COUNT-1）
    const DIRECTIONS: [Angle; Self::VERTEX_COUNT] = [
        Angle::new(0),
        Angle::new(0),
        Angle::new(2),
        Angle::new(11),
        Angle::new(1),
        Angle::new(4),
        Angle::new(6),
        Angle::new(3),
        Angle::new(5),
        Angle::new(8),
        Angle::new(6),
        Angle::new(9),
        Angle::new(7),
        Angle::new(10),
    ];

    /// 指定されたアンカーを基準点としてタイルを生成する
    pub fn new_with_anchor(
        anchor_point: Vec2,
        anchor: Anchor,
        size: f32,
        angle: impl Into<Angle>,
    ) -> Self {
        Self::new_with_anchor_at(anchor_point, anchor.index(), size, angle.into())
    }

    /// 指定された角度の方向ベクトルを計算する
    fn direction_vector(angle: Angle, direction: Angle) -> Vec2 {
        let total_angle = angle + direction;
        let rad = total_angle.to_radians();
        Vec2::new(rad.cos(), rad.sin())
    }

    /// 指定されたアンカーを基準に点を配置する
    ///
    /// # Arguments
    /// * `anchor_point` - アンカーの座標
    /// * `anchor_index` - アンカーのインデックス
    /// * `size` - 辺の長さ
    /// * `angle` - 基準角度（0〜ANGLE_COUNT-1）
    fn new_with_anchor_at(
        anchor_point: Vec2,
        anchor_index: usize,
        size: f32,
        angle: Angle,
    ) -> Self {
        let mut points = [Vec2::ZERO; Self::VERTEX_COUNT];
        points[anchor_index] = anchor_point;

        // アンカーから前方の点を配置
        Self::place_points_before(&mut points[..anchor_index], anchor_point, angle, size);

        // アンカーから後方の点を配置
        Self::place_points_after(
            &mut points[anchor_index + 1..],
            anchor_point,
            anchor_index,
            angle,
            size,
        );

        Self {
            size,
            angle,
            points,
        }
    }

    /// アンカーより前方の点を配置する（反時計回り）
    fn place_points_before(points: &mut [Vec2], start: Vec2, angle: Angle, size: f32) {
        let mut p = start;
        for (i, point) in points.iter_mut().enumerate().rev() {
            let dir = Self::direction_vector(angle, Self::DIRECTIONS[i]);
            p -= dir * size;
            *point = p;
        }
    }

    /// アンカーより後方の点を配置する（時計回り）
    fn place_points_after(
        points: &mut [Vec2],
        start: Vec2,
        anchor_index: usize,
        angle: Angle,
        size: f32,
    ) {
        let mut p = start;
        for (i, point) in points.iter_mut().enumerate() {
            let dir = Self::direction_vector(angle, Self::DIRECTIONS[anchor_index + i]);
            p += dir * size;
            *point = p;
        }
    }

    /// アンカーから出る辺の方向を取得する
    fn edge_direction(anchor: Anchor) -> Angle {
        Self::DIRECTIONS[anchor.index()]
    }

    /// アンカーに入る辺の方向を取得する
    fn prev_edge_direction(anchor: Anchor) -> Angle {
        Self::DIRECTIONS[(anchor.index() + Self::VERTEX_COUNT - 1) % Self::VERTEX_COUNT]
    }

    /// 指定されたアンカー同士を接続した新しいSpectreを生成する
    ///
    /// # Arguments
    /// * `from_anchor` - このSpectreの接続元アンカー
    /// * `to_anchor` - 新しいSpectreの接続先アンカー
    ///
    /// # Returns
    /// 接続された新しいSpectre。このSpectreのfrom_anchorと新しいSpectreのto_anchorが接続される。
    pub fn adjacent_anchor(&self, from_anchor: Anchor, to_anchor: Anchor) -> Spectre {
        // 接続する辺の方向を取得
        let out_dir = Self::edge_direction(from_anchor);
        let in_dir = Self::prev_edge_direction(to_anchor);

        // 新しいSpectreの角度を計算
        // 1. 現在の角度を基準に
        // 2. 出る辺の方向を加える
        // 3. 入る辺の方向を引く
        // 4. 180度（6方向）回転させて反対向きにする
        let angle = self.angle + out_dir - (in_dir - Angle::OPPOSITE);

        // 新しいSpectreを生成：接続点を基準に配置
        Self::new_with_anchor(
            self.points[from_anchor.index()],
            to_anchor,
            self.size,
            angle,
        )
    }

    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.points[anchor.index()]
    }
}
