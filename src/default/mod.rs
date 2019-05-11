use crate::scoped_branch::ScopedBranch;
use crate::TreeBuilder;

/// Returns the default tree for the current thread
///
/// # Example
///
/// ```
/// use debug_tree::default_tree;
/// default_tree().add_leaf("A new leaf");
/// assert_eq!("A new leaf", default_tree().peek_string());
/// ```
#[macro_export]
pub fn default_tree() -> TreeBuilder {
    thread_local! {
        static DEFAULT_BUILDER: TreeBuilder = TreeBuilder::new();
    }
    DEFAULT_BUILDER.with(|f| f.clone())
}

/// Adds a leaf to the default tree with the given text and formatting arguments
///
/// # Arguments
/// * `text...` - Formatted text arguments, as per `format!(...)`.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate debug_tree;
/// use debug_tree::default_tree;
/// fn main() {
///     add_leaf!(tree, "A {} leaf", "new");
///     assert_eq!("A new leaf", &default_tree().peek_string());
/// }
/// ```
#[macro_export]
macro_rules! add_leaf {
        ($($arg:tt)*) => ($crate::default_tree().add_leaf(&format!($($arg)*)));
    }

/// Adds a scoped branch to the default tree with the given text and formatting arguments
/// The branch will be exited at the end of the current block.
///
/// # Arguments
/// * `text...` - Formatted text arguments, as per `format!(...)`.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate debug_tree;
/// use debug_tree::default_tree;
/// fn main() {
///     {
///         add_branch!("New {}", "Branch"); // _branch enters scope
///         // tree is now pointed inside new branch.
///         add_leaf!("Child of {}", "Branch");
///         // Block ends, so tree exits the current branch.
///     }
///     add_leaf!("Sibling of {}", "Branch");
///     assert_eq!("\
/// Branch
/// └╼ Child of Branch
/// Sibling of Branch" , &default_tree().flush_string());
/// }
/// ```
#[macro_export]
macro_rules! add_branch {
    () => {
        let _l = $crate::default_tree().enter_scoped();
    };
    ($($arg:tt)*) => {
        let _l = $crate::default_tree().add_branch(&format!($($arg)*));
    };

}

#[cfg(test)]
mod test {
    use crate::default_tree;
    use crate::*;

    #[test]
    fn unnamed_branch() {
        add_leaf!("1");
        add_branch!();
        add_leaf!("1.1");
        {
            add_branch!();
            add_leaf!("1.1.1");
        }
        add_leaf!("1.2");
        default_tree().peek_print();
        assert_eq!(
            "\
1
├╼ 1.1
│ └╼ 1.1.1
└╼ 1.2",
            default_tree().flush_string()
        );
    }
    #[test]
    fn named_branch() {
        add_branch!("11");
        {
            add_branch!("11.1");
            add_leaf!("11.1.1");
        }
        add_leaf!("11.2");
        default_tree().peek_print();
        assert_eq!(
            "\
11
├╼ 11.1
│ └╼ 11.1.1
└╼ 11.2",
            default_tree().flush_string()
        );
    }

}
