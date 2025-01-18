use super::{Aabb, Geometry as _, MysticLike, Spectre, SpectreLike};

pub trait SpectreContainer {
    fn get_spectre(&self, index: usize) -> Option<&SpectreLike>;
    fn get_mystic(&self) -> Option<&MysticLike>;
    fn max_index(&self) -> usize;
    fn level(&self) -> usize;
}

#[derive(Clone)]
pub struct SpectreIter<'a> {
    pub parents: Vec<(&'a dyn SpectreContainer, usize)>,
    pub aabb: Aabb,
}

impl<'a> Iterator for SpectreIter<'a> {
    type Item = &'a Spectre;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((parent, index)) = self.parents.pop() {
            if parent.level() == 1 {
                // abcdefgを辿る
                for i in index..parent.max_index() {
                    if let Some(SpectreLike::Spectre(spectre)) = parent.get_spectre(i) {
                        if !spectre.aabb().has_intersection(&self.aabb) {
                            continue;
                        }
                        self.parents.push((parent, i + 1));
                        return Some(spectre);
                    }
                }

                // hを辿る
                if let Some(MysticLike::Mystic(mystic)) = parent.get_mystic() {
                    if mystic.aabb().has_intersection(&self.aabb) {
                        if index <= parent.max_index() {
                            // Mysticのaを判定する
                            if mystic.a.aabb().has_intersection(&self.aabb) {
                                self.parents.push((parent, parent.max_index() + 1));
                                return Some(&mystic.a);
                            }
                        }
                        // Mysticのbを判定する
                        if mystic.b.aabb().has_intersection(&self.aabb) {
                            // 最後なのでparentsに追加しない
                            return Some(&mystic.b);
                        }
                    }
                }
            } else {
                // SuperSpectreを辿る
                for i in index..parent.max_index() {
                    if let Some(SpectreLike::SuperSpectre(super_spectre)) = parent.get_spectre(i) {
                        if !super_spectre.aabb().has_intersection(&self.aabb) {
                            continue;
                        }
                        self.parents.push((parent, i + 1));
                        self.parents.push((super_spectre, 0));
                        return self.next();
                    }
                }
                // SuperMysticを辿る
                if index <= parent.max_index() {
                    if let Some(MysticLike::SuperMystic(super_mystic)) = parent.get_mystic() {
                        if super_mystic.aabb().has_intersection(&self.aabb) {
                            self.parents.push((parent, parent.max_index() + 1));
                            self.parents.push((super_mystic, 0));
                            return self.next();
                        }
                    }
                }
            }
        }
        None
    }
}
