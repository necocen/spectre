use crate::utils::Aabb;

use super::{Mystic, MysticCluster, MysticLike, Spectre, SpectreCluster, SpectreLike};

#[derive(Clone)]
enum Node<'a> {
    SpectreCluster(&'a SpectreCluster),
    MysticCluster(&'a MysticCluster),
    Spectre(&'a Spectre),
    Mystic(&'a Mystic),
}

impl<'a> Node<'a> {
    fn from_spectre_like(child: &'a SpectreLike) -> Option<Self> {
        match child {
            SpectreLike::Spectre(tile) => Some(Self::Spectre(tile)),
            SpectreLike::Cluster(cluster) => Some(Self::SpectreCluster(cluster)),
            SpectreLike::Skeleton(_) => None,
        }
    }

    fn from_mystic_like(child: &'a MysticLike) -> Option<Self> {
        match child {
            MysticLike::Mystic(pair) => Some(Self::Mystic(pair)),
            MysticLike::Cluster(cluster) => Some(Self::MysticCluster(cluster)),
            MysticLike::Skeleton(_) => None,
        }
    }

    fn get_child(&self, index: usize) -> Option<Node<'a>> {
        match self {
            Node::SpectreCluster(cluster) => {
                if index == 7 {
                    Self::from_mystic_like(&cluster.h)
                } else {
                    [
                        &cluster.a, &cluster.b, &cluster.c, &cluster.d, &cluster.e, &cluster.f,
                        &cluster.g,
                    ]
                    .get(index)
                    .and_then(|child| Self::from_spectre_like(child))
                }
            }
            Node::MysticCluster(cluster) => {
                if index == 6 {
                    Self::from_mystic_like(&cluster.h)
                } else {
                    [
                        &cluster.a, &cluster.b, &cluster.c, &cluster.d, &cluster.f, &cluster.g,
                    ]
                    .get(index)
                    .and_then(|child| Self::from_spectre_like(child))
                }
            }
            Node::Spectre(_) => None,
            Node::Mystic(mystic) => match index {
                0 => Some(Node::Spectre(mystic.lower())),
                1 => Some(Node::Spectre(mystic.upper())),
                _ => None,
            },
        }
    }

    fn num_children(&self) -> usize {
        match self {
            Node::SpectreCluster(_) => 8,
            Node::MysticCluster(_) => 7,
            Node::Spectre(_) => 0,
            Node::Mystic(_) => 2,
        }
    }

    fn bbox(&self) -> Aabb {
        match self {
            Node::SpectreCluster(cluster) => cluster.bbox(),
            Node::MysticCluster(cluster) => cluster.bbox(),
            Node::Spectre(spectre) => spectre.bbox(),
            Node::Mystic(mystic) => mystic.bbox(),
        }
    }
}

impl<'a> From<&'a SpectreCluster> for Node<'a> {
    fn from(cluster: &'a SpectreCluster) -> Self {
        Node::SpectreCluster(cluster)
    }
}

#[derive(Clone)]
pub struct SpectreIter<'a> {
    parents: Vec<(Node<'a>, usize)>,
    bbox: Aabb,
}

impl<'a> SpectreIter<'a> {
    pub fn new(root: &'a SpectreCluster, bbox: Aabb) -> SpectreIter<'a> {
        let mut parents = Vec::with_capacity(root.level() + 2);
        parents.push((root.into(), 0));
        SpectreIter { parents, bbox }
    }
}

impl<'a> Iterator for SpectreIter<'a> {
    type Item = &'a Spectre;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: while let Some((parent, index)) = self.parents.pop() {
            for i in index..parent.num_children() {
                if let Some(child) = parent.get_child(i)
                    && child.bbox().has_intersection(&self.bbox)
                {
                    if let Node::Spectre(spectre) = child {
                        self.parents.push((parent, i + 1));
                        return Some(spectre);
                    } else {
                        self.parents.push((parent, i + 1));
                        self.parents.push((child, 0));
                        continue 'outer;
                    }
                }
            }
        }
        None
    }
}
