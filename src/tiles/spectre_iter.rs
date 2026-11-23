use crate::utils::Aabb;

use super::{
    MAX_CLUSTER_LEVEL, Mystic, MysticCluster, MysticLike, Spectre, SpectreCluster, SpectreLike,
};

#[derive(Clone, Copy)]
enum Node<'a> {
    SpectreCluster(&'a SpectreCluster),
    MysticCluster(&'a MysticCluster),
    Spectre(&'a Spectre),
    Mystic(&'a Mystic),
}

impl<'a> Node<'a> {
    #[inline]
    fn from_spectre_like(child: &'a SpectreLike) -> Option<Self> {
        match child {
            SpectreLike::Spectre(tile) => Some(Self::Spectre(tile)),
            SpectreLike::Cluster(cluster) => Some(Self::SpectreCluster(cluster)),
            SpectreLike::Skeleton(_) => None,
        }
    }

    #[inline]
    fn from_mystic_like(child: &'a MysticLike) -> Option<Self> {
        match child {
            MysticLike::Mystic(pair) => Some(Self::Mystic(pair)),
            MysticLike::Cluster(cluster) => Some(Self::MysticCluster(cluster)),
            MysticLike::Skeleton(_) => None,
        }
    }

    #[inline]
    fn get_child(self, index: u8) -> Option<Node<'a>> {
        let idx = index as usize;
        match self {
            Node::SpectreCluster(cluster) => {
                if idx == 7 {
                    Self::from_mystic_like(&cluster.h)
                } else {
                    Self::from_spectre_like(&cluster.children[idx])
                }
            }
            Node::MysticCluster(cluster) => {
                if idx == 6 {
                    Self::from_mystic_like(&cluster.h)
                } else {
                    Self::from_spectre_like(&cluster.children[idx])
                }
            }
            Node::Spectre(_) => None,
            Node::Mystic(mystic) => match idx {
                0 => Some(Node::Spectre(mystic.lower())),
                1 => Some(Node::Spectre(mystic.upper())),
                _ => None,
            },
        }
    }

    #[inline]
    fn num_children(self) -> u8 {
        match self {
            Node::SpectreCluster(_) => 8,
            Node::MysticCluster(_) => 7,
            Node::Spectre(_) => 0,
            Node::Mystic(_) => 2,
        }
    }

    #[inline]
    fn bbox(self) -> Aabb {
        match self {
            Node::SpectreCluster(cluster) => cluster.bbox(),
            Node::MysticCluster(cluster) => cluster.bbox(),
            Node::Spectre(spectre) => spectre.bbox(),
            Node::Mystic(mystic) => mystic.bbox(),
        }
    }
}

#[derive(Clone, Copy)]
struct Frame<'a> {
    node: Node<'a>,
    index: u8,
    fully_inside: bool,
}

const STACK_CAPACITY: usize = MAX_CLUSTER_LEVEL + 2;

#[derive(Clone)]
pub struct SpectreIter<'a> {
    stack: [Option<Frame<'a>>; STACK_CAPACITY],
    len: usize,
    bbox: Aabb,
}

impl<'a> SpectreIter<'a> {
    #[inline]
    pub fn new(root: &'a SpectreCluster, bbox: Aabb) -> SpectreIter<'a> {
        let mut stack = [None; STACK_CAPACITY];
        let root_bbox = root.bbox();
        let len = if root_bbox.has_intersection(&bbox) {
            stack[0] = Some(Frame {
                node: Node::SpectreCluster(root),
                index: 0,
                fully_inside: bbox.contains_aabb(&root_bbox),
            });
            1
        } else {
            0
        };
        SpectreIter { stack, len, bbox }
    }
}

impl<'a> Iterator for SpectreIter<'a> {
    type Item = &'a Spectre;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.len > 0 {
            let top_idx = self.len - 1;
            let frame = self.stack[top_idx].as_mut().unwrap();
            let parent = frame.node;
            let num_children = parent.num_children();
            let parent_inside = frame.fully_inside;

            if frame.index >= num_children {
                self.len -= 1;
                continue;
            }

            let child_idx = frame.index;
            frame.index = child_idx + 1;

            let Some(child) = parent.get_child(child_idx) else {
                continue;
            };

            let child_inside = if parent_inside {
                true
            } else {
                let child_bbox = child.bbox();
                if !child_bbox.has_intersection(&self.bbox) {
                    continue;
                }
                self.bbox.contains_aabb(&child_bbox)
            };

            if let Node::Spectre(spectre) = child {
                return Some(spectre);
            }

            self.stack[self.len] = Some(Frame {
                node: child,
                index: 0,
                fully_inside: child_inside,
            });
            self.len += 1;
        }
        None
    }
}
