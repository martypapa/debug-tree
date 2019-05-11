use std::sync::{Arc, Mutex};
pub mod default;
mod internal;
pub mod scoped_branch;
mod test;
mod tree;
pub use default::default_tree;
use scoped_branch::ScopedBranch;
use tree::Tree;

/// Reference wrapper for `State`
#[derive(Debug, Clone)]
pub struct TreeBuilder(Arc<Mutex<internal::TreeBuilderBase>>);

impl TreeBuilder {
    /// Returns a new `TreeBuilder` with an empty `Tree`.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// ```
    pub fn new() -> TreeBuilder {
        TreeBuilder {
            0: Arc::new(Mutex::new(internal::TreeBuilderBase::new())),
        }
    }

    /// Returns a new `TreeBuilder`, for an existing `Tree`.
    /// The `TreeBuilder` will be positioned on the same branch as the current tree
    pub(crate) fn from_tree(tree: Arc<Mutex<Tree>>) -> TreeBuilder {
        TreeBuilder {
            0: Arc::new(Mutex::new(internal::TreeBuilderBase::from_tree(
                tree.clone(),
            ))),
        }
    }

    /// Sets the indentation level between tree branches.
    /// Aside from the first branch, `indent` is equal to the number of spaces a child branch is
    /// shifted from its parent.
    ///
    /// # Arguments
    /// * `indent` - The number of spaces used for indenting.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.set_indentation(4);
    /// ```
    pub fn set_indentation(&self, indent: usize) {
        self.0.lock().unwrap().set_indentation(indent);
    }

    /// Adds a new branch with text, `text` and returns a `ScopedBranch`.
    /// When the returned `ScopedBranch` goes out of scope, (likely the end of the current block),
    /// or if its `release()` method is called, the tree will step back out of the added branch.
    ///
    /// # Arguments
    /// * `text` - A string slice to use as the newly added branch's text.
    ///
    /// # Examples
    ///
    /// Exiting branch when end of scope is reached.
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// {
    ///     let _branch = tree.add_branch("Branch"); // _branch enters scope
    ///     // tree is now pointed inside new branch.
    ///     tree.add_leaf("Child of Branch");
    ///     // _branch leaves scope, tree moves up to parent branch.
    /// }
    /// tree.add_leaf("Sibling of Branch");
    /// assert_eq!("\
    /// Branch
    /// └╼ Child of Branch
    /// Sibling of Branch" , &tree.flush_string());
    /// ```
    ///
    /// Using `release()` before out of scope.
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// {
    ///     let mut branch = tree.add_branch("Branch"); // branch enters scope
    ///     // tree is now pointed inside new branch.
    ///     tree.add_leaf("Child of Branch");
    ///     branch.release();
    ///     tree.add_leaf("Sibling of Branch");
    ///     // branch leaves scope, but no effect because its `release()` method has already been called
    /// }
    /// assert_eq!("\
    /// Branch
    /// └╼ Child of Branch
    /// Sibling of Branch", &tree.flush_string());
    /// ```
    pub fn add_branch(&self, text: &str) -> ScopedBranch {
        self.add_leaf(text);
        ScopedBranch::new(self.clone())
    }

    /// Adds a new branch with text, `text` and returns a `ScopedBranch`.
    /// When the returned `ScopedBranch` goes out of scope, (likely the end of the current block),
    /// or if its `release()` method is called, the tree tree will step back out of the added branch.
    ///
    /// # Arguments
    /// * `text` - A string slice to use as the newly added branch's text.
    ///
    /// # Examples
    ///
    /// Stepping out of branch when end of scope is reached.
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// {
    ///     tree.add_leaf("Branch");
    ///     let _branch = tree.enter_scoped(); // _branch enters scope
    ///     // tree is now pointed inside new branch.
    ///     tree.add_leaf("Child of Branch");
    ///     // _branch leaves scope, tree moves up to parent branch.
    /// }
    /// tree.add_leaf("Sibling of Branch");
    /// assert_eq!("\
    /// Branch
    /// └╼ Child of Branch
    /// Sibling of Branch", &tree.flush_string());
    /// ```
    ///
    /// Using `release()` before out of scope.
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// {
    ///     tree.add_leaf("Branch");
    ///     let mut branch = tree.enter_scoped(); // branch enters scope
    ///     // tree is now pointed inside new branch.
    ///     tree.add_leaf("Child of Branch");
    ///     branch.release();
    ///     tree.add_leaf("Sibling of Branch");
    ///     // branch leaves scope, but no effect because its `release()` method has already been called
    /// }
    /// assert_eq!("\
    /// Branch
    /// └╼ Child of Branch
    /// Sibling of Branch", &tree.flush_string());
    /// ```
    pub fn enter_scoped(&self) -> ScopedBranch {
        ScopedBranch::new(self.clone())
    }

    /// Adds a leaf to current branch with the given text, `text`.
    ///
    /// # Arguments
    /// * `text` - A string slice to use as the newly added leaf's text.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("New leaf");
    /// ```
    pub fn add_leaf(&self, text: &str) {
        self.0.lock().unwrap().add_leaf(&text);
    }

    /// Steps into a new child branch.
    /// Stepping out of the branch requires calling `exit()`.
    ///
    /// # Example
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Branch");
    /// tree.enter();
    /// tree.add_leaf("Child of Branch");
    /// assert_eq!("\
    /// Branch
    /// └╼ Child of Branch", &tree.flush_string());
    /// ```
    pub fn enter(&self) {
        self.0.lock().unwrap().enter();
    }

    /// Exits the current branch, to the parent branch.
    /// If no parent branch exists, no action is taken
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Branch");
    /// tree.enter();
    /// tree.add_leaf("Child of Branch");
    /// tree.exit();
    /// tree.add_leaf("Sibling of Branch");
    /// assert_eq!("\
    /// Branch
    /// └╼ Child of Branch
    /// Sibling of Branch", &tree.flush_string());
    /// ```
    pub fn exit(&self) -> bool {
        self.0.lock().unwrap().exit()
    }

    /// Returns the depth of the current branch
    /// The initial depth when no branches have been adeed is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// assert_eq!(0, tree.depth());
    /// let _b = tree.add_branch("Branch");
    /// assert_eq!(1, tree.depth());
    /// let _b = tree.add_branch("Child branch");
    /// assert_eq!(2, tree.depth());
    /// ```
    pub fn depth(&self) -> usize {
        self.0.lock().unwrap().depth()
    }

    /// Prints the tree without clearing.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Leaf");
    /// tree.peek_print();
    /// // Leaf
    /// tree.peek_print();
    /// // Leaf
    /// // Leaf 2
    /// ```
    pub fn peek_print(&self) {
        self.0.lock().unwrap().peek_print();
    }

    /// Prints the tree and then clears it.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Leaf");
    /// tree.flush_print();
    /// // Leaf
    /// tree.add_leaf("Leaf 2");
    /// tree.flush_print();
    /// // Leaf 2
    /// ```
    pub fn flush_print(&self) {
        self.0.lock().unwrap().flush_print();
    }

    /// Returns the tree as a string without clearing the tree.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Leaf");
    /// assert_eq!("Leaf", tree.peek_string());
    /// tree.add_leaf("Leaf 2");
    /// assert_eq!("Leaf\nLeaf 2", tree.peek_string());
    /// ```
    pub fn peek_string(&self) -> String {
        self.0.lock().unwrap().peek_string()
    }

    /// Returns the tree as a string and clears the tree.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Leaf");
    /// assert_eq!("Leaf", tree.flush_string());
    /// tree.add_leaf("Leaf 2");
    /// assert_eq!("Leaf 2", tree.flush_string());
    /// ```
    pub fn flush_string(&self) -> String {
        self.0.lock().unwrap().flush_string()
    }

    /// Clears the tree.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Leaf");
    /// assert_eq!("Leaf", tree.peek_string());
    /// tree.clear();
    /// assert_eq!("", tree.peek_string());
    /// ```
    pub fn clear(&self) {
        self.0.lock().unwrap().clear()
    }
}

/// Adds a leaf to given tree with the given text and formatting arguments
///
/// # Arguments
/// * `tree` - The tree that the leaf should be added to
/// * `text...` - Formatted text arguments, as per `format!(...)`.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate debug_tree;
/// use debug_tree::TreeBuilder;
/// fn main() {
///     let tree = TreeBuilder::new();
///     add_leaf_to!(tree, "A {} leaf", "new");
///     assert_eq!("A new leaf", &tree.peek_string());
/// }
/// ```
#[macro_export]
macro_rules! add_leaf_to {
    ($state:tt, $($arg:tt)*) => ($state.add_leaf(&format!($($arg)*)));
}

/// Adds a scoped branch to given tree with the given text and formatting arguments
/// The branch will be exited at the end of the current block.
///
/// # Arguments
/// * `tree` - The tree that the leaf should be added to
/// * `text...` - Formatted text arguments, as per `format!(...)`.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate debug_tree;
/// use debug_tree::TreeBuilder;
/// fn main() {
///     let tree = TreeBuilder::new();
///     {
///         add_branch_to!(tree, "New {}", "Branch"); // _branch enters scope
///         // tree is now pointed inside new branch.
///         add_leaf_to!(tree, "Child of {}", "Branch");
///         // Block ends, so tree exits the current branch.
///     }
///     add_leaf_to!(tree, "Sibling of {}", "Branch");
///     assert_eq!("\
/// New Branch
/// └╼ Child of Branch
/// Sibling of Branch" , &tree.flush_string());
/// }
/// ```
#[macro_export]
macro_rules! add_branch_to {
    ($arg:tt) => {
        let _l = $arg.enter_scoped();
    };
    ($state:tt, $($arg:tt)*) => {
        let _l = $state.add_branch(&format!($($arg)*));
    };
}
