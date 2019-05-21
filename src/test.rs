#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_branch() {
        let d: TreeBuilder = TreeBuilder::new();
        d.add_leaf("1");
        {
            let _l = d.enter_scoped();
            d.add_leaf("1.1");
            d.add_leaf("1.2");
        }
        d.add_leaf("2");
        d.add_leaf("3");
        let _l = d.enter_scoped();
        d.add_leaf("3.1");
        d.add_leaf("3.2");
        d.peek_print();
        assert_eq!(
            "\
1
├╼ 1.1
└╼ 1.2
2
3
├╼ 3.1
└╼ 3.2",
            d.flush_string()
        );
    }
    #[test]
    fn test_branch2() {
        let d = TreeBuilder::new();
        d.add_leaf("1");
        {
            let _scope = d.enter_scoped();
            d.add_leaf("1.1");
            {
                let _scope = d.enter_scoped();
                d.add_leaf("1.1.1");
            }
        }

        d.add_leaf("2");
        d.enter();
        d.add_leaf("2.1");
        d.enter();
        d.add_leaf("2.1.1");
        d.peek_print();
        assert_eq!(
            "\
1
└╼ 1.1
  └╼ 1.1.1
2
└╼ 2.1
  └╼ 2.1.1",
            d.flush_string()
        );
    }

    #[test]
    fn simple() {
        let d = TreeBuilder::new();
        d.add_leaf("Hi");
        assert_eq!("Hi", d.flush_string());
    }

    #[test]
    fn depth() {
        let d = TreeBuilder::new();
        assert_eq!(0, d.depth());
        d.add_leaf("Hi");
        assert_eq!(0, d.depth());
        let _b = d.add_branch("Hi");
        assert_eq!(1, d.depth());
        d.add_leaf("Hi");
        assert_eq!(1, d.depth());
    }
    #[test]
    fn indent() {
        let d = TreeBuilder::new();
        d.add_leaf("1");
        add_branch_to!(d);
        d.add_leaf("1.1");
        {
            add_branch_to!(d);
            d.add_leaf("1.1.1");
        }
        d.set_indentation(4);
        d.peek_print();
        assert_eq!(
            "\
1
└──╼ 1.1
    └──╼ 1.1.1",
            d.flush_string()
        );
    }
    #[test]
    fn macros() {
        let d = TreeBuilder::new();
        add_leaf_to!(d, "1");
        {
            add_branch_to!(d);
            add_leaf_to!(d, "1.1")
        }
        d.peek_print();
        assert_eq!(
            "\
1
└╼ 1.1",
            d.flush_string()
        );
    }
    #[test]
    fn leaf_with_value() {
        let d = TreeBuilder::new();
        let value = add_leaf_value_to!(d, 1);
        d.peek_print();
        assert_eq!("1", d.flush_string());
        assert_eq!(1, value);
    }
    #[test]
    fn macros2() {
        let d = TreeBuilder::new();
        add_branch_to!(d, "1");
        add_leaf_to!(d, "1.1");
        d.peek_print();
        assert_eq!(
            "\
1
└╼ 1.1",
            d.flush_string()
        );
    }

    #[test]
    fn mid() {
        let d = TreeBuilder::new();
        d.add_leaf(&format!("{}{}", "1", "0"));
        d.enter();
        d.add_leaf("10.1");
        d.add_leaf("10.2");
        d.enter();
        d.add_leaf("10.1.1");
        d.add_leaf("10.1.2\nNext line");
        d.exit();
        d.add_leaf(&format!("10.3"));
        d.peek_print();
        assert_eq!(
            "\
10
├╼ 10.1
├╼ 10.2
│ ├╼ 10.1.1
│ └╼ 10.1.2
│    Next line
└╼ 10.3",
            d.flush_string()
        );
    }

    fn factors(x: usize) {
        add_branch!("{}", x);
        for i in 1..x {
            if x % i == 0 {
                factors(i);
            }
        }
    }
    #[test]
    fn recursive() {
        factors(6);
        default_tree().peek_print();
        assert_eq!(
            "\
6
├╼ 1
├╼ 2
│ └╼ 1
└╼ 3
  └╼ 1",
            default_tree().flush_string()
        );
    }

    fn a() {
        add_branch!("a");
        b();
        c();
    }
    fn b() {
        add_branch!("b");
        c();
    }
    fn c() {
        add_branch!("c");
        add_leaf!("Nothing to see here");
    }

    #[test]
    fn nested() {
        a();
        default_tree().peek_print();
        assert_eq!(
            "\
a
├╼ b
│ └╼ c
│   └╼ Nothing to see here
└╼ c
  └╼ Nothing to see here",
            default_tree().flush_string()
        );
    }

    #[test]
    fn disabled_output() {
        let tree = TreeBuilder::new();
        tree.set_enabled(false);
        add_leaf_to!(tree, "Leaf");
        tree.add_leaf("Leaf");

        add_branch_to!(tree, "Branch");
        tree.add_branch("Branch");
        assert_eq!("", tree.flush_string());
    }
    #[test]
    fn enabled_output() {
        let tree = TreeBuilder::new();
        tree.set_enabled(false);
        add_branch_to!(tree, "Ignored branch");
        add_leaf_to!(tree, "Ignored leaf");
        tree.set_enabled(true);
        add_leaf_to!(tree, "Leaf");
        tree.add_leaf("Leaf");

        add_branch_to!(tree, "Branch");
        tree.add_branch("Branch");
        assert_eq!(
            "\
Leaf
Leaf
Branch
└╼ Branch",
            tree.flush_string()
        );
    }
}
