use std::sync::{Arc, Mutex};
pub mod default;
mod internal;
pub mod scoped_branch;
mod test;
mod tree;
use scoped_branch::ScopedBranch;
use tree::Tree;
pub use default::default_tree;

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
    /// Stepping out of branch when end of scope is reached.
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

    /// Returns the depth of
    pub fn depth(&self) -> usize {
        self.0.lock().unwrap().depth()
    }

    /// Print the tree without clearing.
    ///
    /// # Example
    ///
    /// ```
    /// use debug_tree::TreeBuilder;
    /// let tree = TreeBuilder::new();
    /// tree.add_leaf("Test");
    /// tree.peek_print();
    /// ```
    pub fn peek_print(&self) {
        self.0.lock().unwrap().peek_print();
    }

    /// Print the tree and then clear.
    pub fn flush_print(&self) {
        self.0.lock().unwrap().flush_print();
    }

    /// Return the stringified tree without clearing.
    pub fn peek_string(&self) -> String {
        self.0.lock().unwrap().peek_string()
    }
    /// Return the stringified tree and then clear.
    pub fn flush_string(&self) -> String {
        self.0.lock().unwrap().flush_string()
    }

    /// Clear the tree.
    pub fn clear(&self) {
        self.0.lock().unwrap().clear()
    }
}

#[macro_export]
macro_rules! add_leaf_to {
    ($state:tt, $($arg:tt)*) => ($state.add_leaf(&format!($($arg)*)));
}
#[macro_export]
macro_rules! add_branch_to {
    ($arg:tt) => {
        let _l = $arg.enter_scoped();
    };
    ($state:tt, $($arg:tt)*) => {
        let _l = $state.add_branch(&format!($($arg)*));
    };
}
