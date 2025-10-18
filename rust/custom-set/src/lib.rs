use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Hash + Eq>(HashSet<T>);

impl<T: Hash + Eq + Copy> CustomSet<T> {
    pub fn new(value: &[T]) -> Self {
        Self(HashSet::from_iter(value.iter().copied()))
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }

    pub fn add(&mut self, value: T) {
        self.0.insert(value);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self(
            self.0
                .intersection(&other.0)
                .copied()
                .collect::<HashSet<_>>(),
        )
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self(self.0.difference(&other.0).copied().collect::<HashSet<_>>())
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0.union(&other.0).copied().collect::<HashSet<_>>())
    }
}
