use std::{collections::VecDeque, fmt::Debug, mem};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: Vec<Tree<T>>,
}

impl<T: Debug + Ord> Tree<T> {
    /// Constructs a singleton Tree
    pub fn new(label: T) -> Self {
        Self {
            label,
            children: Vec::new(),
        }
    }

    /// Adds a child to the tree
    pub fn with_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self.children.sort_unstable();
        self
    }

    /// Reroots the tree so that [from] becomes the root.
    ///
    /// First it finds a path from current root to `from`, then flips the
    /// direction of the edges along this path: parents become children,
    /// children become parents.
    pub fn pov_from(&mut self, from: &T) -> bool {
        if let Some(path) = self.find_path(from) {
            for i in path {
                let mut subtree = self.children.remove(i);
                self.children.sort_unstable();
                mem::swap(self, &mut subtree);
                self.children.push(subtree);
            }
            true
        } else {
            false
        }
    }

    /// Reroots the tree with [pov_from], then finds the path from the new root to `to`.
    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if self.pov_from(from) {
            // This gives us, for each level from the new root (`from`) to `root`, the child that is part
            // of the path. Now we walk this path from root to child while taking a reference of the labels
            // of each node.
            let indices = self.find_path(to)?;
            let mut path = Vec::with_capacity(indices.len() + 1);
            let mut tree = self;
            path.push(&tree.label);
            for i in indices {
                tree = &mut tree.children[i];
                path.push(&tree.label)
            }
            Some(path)
        } else {
            None
        }
    }

    /// Finds a path to `to` using depth-first search.
    ///
    /// Returns the indices of the childs that lead to `to` for each level.
    /// Returns `None` if a path cannot be found.
    ///
    /// A fact: `to` is a descendant of `self`.
    ///
    /// ## Example
    ///
    /// Given the current tree istance
    ///
    /// ```txt
    /// grapndparent
    ///     parent
    ///         sibling-0
    ///         sibling-1
    ///         x
    ///             kid-0
    ///             kid-1
    ///     uncle
    ///         cousin-0
    ///         cousin-1
    /// ```
    ///
    /// when this method is called with `x` as `to` it will return `[0, 2]`,
    /// because we take `parent` which has index 0 at the first level, and then
    /// `x` at the second level which has index `2`
    ///
    fn find_path(&self, to: &T) -> Option<VecDeque<usize>> {
        if &self.label == to {
            return Some(VecDeque::new());
        }
        for (i, subtree) in self.children.iter().enumerate() {
            if let Some(mut path) = subtree.find_path(to) {
                path.push_front(i);
                return Some(path);
            }
        }
        None
    }
}
