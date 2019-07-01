use std::sync::{Arc, Mutex};
pub mod default;
mod internal;
pub mod scoped_branch;
mod test;
mod tree;
pub use default::default_tree;
use scoped_branch::ScopedBranch;

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
        if self.is_enabled() {
            ScopedBranch::new(self.clone())
        } else {
           ScopedBranch::none()
        }
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
        let mut x = self.0.lock().unwrap();
        if x.is_enabled() {
            x.add_leaf(&text);
        }
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
        let mut x = self.0.lock().unwrap();
        if x.is_enabled() {
            x.enter();
        }
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
        let mut x = self.0.lock().unwrap();
        if x.is_enabled() {
            x.exit()
        } else {
            false
        }
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

    /// Sets the enabled state of the tree.
    ///
    /// If not enabled, the tree will not be modified by adding leaves or branches.
    /// Additionally, if called using the `add_`... macros, arguments will not be processed.
    /// This is particularly useful for suppressing output in production, with very little overhead.
    ///
    /// # Example
    /// ```
    /// #[macro_use]
    /// use debug_tree::{TreeBuilder, add_leaf_to};
    /// let mut tree = TreeBuilder::new();
    /// tree.add_leaf("Leaf 1");
    /// tree.set_enabled(false);
    /// add_leaf_to!(tree, "Leaf 2");
    /// tree.set_enabled(true);
    /// add_leaf_to!(tree, "Leaf 3");
    /// assert_eq!("Leaf 1\nLeaf 3", tree.peek_string());
    /// ```
    pub fn set_enabled(&self, enabled: bool) {
        self.0.lock().unwrap().set_enabled(enabled);
    }

    /// Returns the enabled state of the tree.
    ///
    /// # Example
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let mut tree = TreeBuilder::new();
    /// assert_eq!(true, tree.is_enabled());
    /// tree.set_enabled(false);
    /// assert_eq!(false, tree.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.0.lock().unwrap().is_enabled()
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
/// use debug_tree::{TreeBuilder, add_leaf_to};
/// fn main() {
///     let tree = TreeBuilder::new();
///     add_leaf_to!(tree, "A {} leaf", "new");
///     assert_eq!("A new leaf", &tree.peek_string());
/// }
/// ```
#[macro_export]
macro_rules! add_leaf_to {
    ($state:expr, $($arg:tt)*) => (if $state.is_enabled() { $state.add_leaf(&format!($($arg)*))});
}

/// Adds a leaf to given tree with the given `value` argument
///
/// # Arguments
/// * `tree` - The tree that the leaf should be added to
/// * `value` - An expression that implements the `Display` trait.
///
/// # Example
///
/// ```
/// #[macro_use]
/// use debug_tree::{TreeBuilder, add_leaf_value_to};
/// fn main() {
///     let tree = TreeBuilder::new();
///     let value = add_leaf_value_to!(tree, 5 * 4 * 3 * 2);
///     assert_eq!(120, value);
///     assert_eq!("120", &tree.peek_string());
/// }
/// ```
#[macro_export]
macro_rules! add_leaf_value_to {
    ($state:expr, $value:expr) => {{
        let v = $value;
        if $state.is_enabled() {
            $state.add_leaf(&format!("{}", &v));
        }
        v
    }};
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
/// use debug_tree::{TreeBuilder, add_branch_to, add_leaf_to};
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
    ($arg:expr) => {
        let _debug_tree_branch = if $arg.is_enabled() {
            $arg.enter_scoped()
        } else {
            $crate::scoped_branch::ScopedBranch::none()
        };
    };
    ($state:expr, $($arg:tt)*) => {
        let _debug_tree_branch = if $state.is_enabled() {
            $state.add_branch(&format!($($arg)*))
        } else {
            $crate::scoped_branch::ScopedBranch::none()
        };
    };
}
