use crate::scoped_branch::ScopedBranch;
use crate::TreeBuilder;
pub fn default_tree() -> TreeBuilder {
    thread_local! {
        static DEFAULT_BUILDER: TreeBuilder = TreeBuilder::new();
    }
    DEFAULT_BUILDER.with(|f| f.clone())
}

#[macro_export]
macro_rules! add_leaf {
        ($($arg:tt)*) => ($crate::default_tree().add_leaf(&format!($($arg)*)));
    }
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
