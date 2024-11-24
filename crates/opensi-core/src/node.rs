use std::fmt::Display;

use derive_more::{AsRef, Deref, From};

/// [`Package`](crate::package::Package) tree node which
/// operates on indices and is easy to copy.
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    From,
)]
pub enum PackageNode {
    Round(RoundIdx),
    Theme(ThemeIdx),
    Question(QuestionIdx),
}

impl PackageNode {
    /// Get parent of the node, unless it's a [`PackageNode::Round`].
    pub fn parent(&self) -> Option<PackageNode> {
        match self {
            PackageNode::Round(_) => None,
            PackageNode::Theme(node) => Some(node.parent().into()),
            PackageNode::Question(node) => Some(node.parent().into()),
        }
    }

    /// Get index of the node itself.
    pub fn index(&self) -> usize {
        match self {
            &PackageNode::Round(RoundIdx { index, .. })
            | &PackageNode::Theme(ThemeIdx { index, .. })
            | &PackageNode::Question(QuestionIdx { index, .. }) => index,
        }
    }
}

impl From<usize> for PackageNode {
    fn from(idx: usize) -> Self {
        Self::Round(idx.into())
    }
}

impl From<(usize, usize)> for PackageNode {
    fn from(idx: (usize, usize)) -> Self {
        Self::Theme(idx.into())
    }
}

impl From<(usize, usize, usize)> for PackageNode {
    fn from(idx: (usize, usize, usize)) -> Self {
        Self::Question(idx.into())
    }
}

/// Typed [`Round`](crate::components::Round) index.
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    AsRef,
)]
pub struct RoundIdx {
    pub index: usize,
}

impl RoundIdx {
    /// Get a [`ThemeIdx`] with a selected index.
    pub fn theme(&self, index: usize) -> ThemeIdx {
        ThemeIdx { round_index: self.index, index }
    }

    /// Get next round.
    pub fn next(&self) -> Self {
        Self { index: self.index + 1 }
    }
}

impl From<usize> for RoundIdx {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl Display for RoundIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}]", self.index))
    }
}

/// Typed [`Theme`](crate::components::Theme) indices.
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    AsRef,
)]
pub struct ThemeIdx {
    pub round_index: usize,
    #[deref]
    #[as_ref]
    pub index: usize,
}

impl ThemeIdx {
    /// Get a [`QuestionIdx`] with a selected index.
    pub fn question(&self, index: usize) -> QuestionIdx {
        QuestionIdx { round_index: self.round_index, theme_index: self.index, index }
    }

    /// Get a parent [`RoundIdx`].
    pub fn parent(&self) -> RoundIdx {
        RoundIdx { index: self.round_index }
    }

    /// Get next theme.
    pub fn next(&self) -> Self {
        Self { index: self.index + 1, ..*self }
    }
}

impl From<(usize, usize)> for ThemeIdx {
    fn from((round_index, index): (usize, usize)) -> Self {
        Self { round_index, index }
    }
}

impl Display for ThemeIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} > {}]", self.round_index, self.index))
    }
}

/// Typed [`Question`](crate::components::Question) indices.
#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deref,
    AsRef,
)]
pub struct QuestionIdx {
    pub round_index: usize,
    pub theme_index: usize,
    #[deref]
    #[as_ref]
    pub index: usize,
}

impl QuestionIdx {
    /// Get a parent [`ThemeIdx`]
    pub fn parent(&self) -> ThemeIdx {
        ThemeIdx { round_index: self.round_index, index: self.theme_index }
    }

    /// Get next question.
    pub fn next(&self) -> Self {
        Self { index: self.index + 1, ..*self }
    }
}

impl From<(usize, usize, usize)> for QuestionIdx {
    fn from((round_index, theme_index, index): (usize, usize, usize)) -> Self {
        Self { round_index, theme_index, index }
    }
}

impl Display for QuestionIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} > {} > {}]", self.round_index, self.theme_index, self.index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_hierarchy() {
        let base: RoundIdx = 0.into();
        assert_eq!(base, RoundIdx { index: 0 });
        assert_eq!(base.theme(5), ThemeIdx { round_index: 0, index: 5 });
        assert_eq!(
            base.theme(1).question(2),
            QuestionIdx { round_index: 0, theme_index: 1, index: 2 }
        );

        let question: QuestionIdx = (1, 2, 3).into();
        assert_eq!(question, QuestionIdx { round_index: 1, theme_index: 2, index: 3 });
        assert_eq!(question.parent(), ThemeIdx { round_index: 1, index: 2 });
        assert_eq!(question.parent().parent(), RoundIdx { index: 1 });
    }
}
