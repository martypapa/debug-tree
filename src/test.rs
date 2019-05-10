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
}
