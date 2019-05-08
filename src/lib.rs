use std::sync::{Arc, Mutex};

/// Tree that holds `text` for the current leaf and a list of `children` that are the branches.
#[derive(Debug)]
struct Tree {
    text: Option<String>,
    children: Vec<Tree>,
}

/// Position of the element relative to its siblings
#[derive(Copy, Clone, Debug)]
enum Position {
    Inside,
    First,
    Last,
    Only,
}

impl Tree {
    /// Create a new tree with some optional text.
    fn new(text: Option<&str>) -> Tree {
        Tree {
            text: text.map(|x| x.to_string()),
            children: Vec::new(),
        }
    }

    /// Navigate to the branch at the given `path` relative to this tree.
    /// If a valid branch is found by following the path, it is returned.
    fn at_mut(&mut self, path: &[usize]) -> Option<&mut Tree> {
        match path.first() {
            Some(&i) => match self.children.get_mut(i) {
                Some(x) => x.at_mut(&path[1..]),
                _ => None,
            },
            _ => Some(self),
        }
    }

    /// "Render" this tree as a list of `String`s.
    /// Each string represents a line in the tree.
    /// `does_continue` is a bool for each column indicating whether the tree continues.
    fn lines(
        &self,
        does_continue: &Vec<bool>,
        index: usize,
        pool_size: usize,
        indent: usize,
    ) -> Vec<String> {
        let position = match index {
            _ if pool_size == 1 => Position::Only,
            _ if index == pool_size - 1 => Position::Last,
            0 => Position::First,
            _ => Position::Inside,
        };
        let mut next_continue = does_continue.clone();
        next_continue.push(match position {
            Position::Inside | Position::First => true,
            Position::Last | Position::Only => false,
        });

        let mut txt = String::new();
        let mut pad: String;
        if does_continue.len() > 1 {
            for &i in &does_continue[2..] {
                txt.push_str(&format!(
                    "{}{:indent$}",
                    if i { "│" } else { " " },
                    "",
                    indent = indent - 1
                ));
            }
            pad = txt.clone();
            txt.push_str(&format!(
                "{}{}╼",
                match position {
                    Position::Only | Position::Last => "└",
                    Position::First | Position::Inside => "├",
                },
                "─".repeat(indent - 2),
            ));

            let s = match &self.text {
                Some(x) => match x.contains("\n") {
                    true => format!(
                        " {}",
                        x.replace(
                            "\n",
                            &format!(
                                "\n{}{}  ",
                                &pad,
                                match position {
                                    Position::Only | Position::Last => " ",
                                    _ => "│",
                                },
                            )
                        )
                    ),
                    false => format!(" {}", x),
                },
                _ => String::new(),
            };
            txt.push_str(&s);
        } else {
            if let Some(x) = &self.text {
                txt.push_str(&x);
            }
        }
        let mut ret = vec![txt];
        for (index, x) in self.children.iter().enumerate() {
            for line in x.lines(&next_continue, index, self.children.len(), indent) {
                ret.push(line);
            }
        }
        ret
    }
}

#[derive(Debug, Clone)]
pub struct State {
    root: Arc<Mutex<Tree>>,
    path: Vec<usize>,
    dive_count: usize,
    indent: usize,
}

#[derive(Debug, Clone)]
pub struct StateRef(pub Arc<Mutex<State>>);

impl StateRef {
    pub fn new() -> StateRef {
        StateRef {
            0: Arc::new(Mutex::new(State::new())),
        }
    }

    pub fn set_indentation(&self, indent: usize) {
        self.0.lock().unwrap().set_indentation(indent);
    }

    /// Add a leaf to current branch.
    pub fn add(&self, text: &str) {
        self.0.lock().unwrap().add(&text);
    }

    /// Steps into a new child branch.
    pub fn step_in(&self) {
        self.0.lock().unwrap().step_in();
    }

    /// Steps up to the parent branch.
    pub fn step_out(&self) {
        self.0.lock().unwrap().step_out();
    }

    /// Steps in to child branch until the returned value is removed from scope,
    /// or `release()` is called
    pub fn step_in_scoped(&self) -> ScopedBranch {
        ScopedBranch::new(self.clone())
    }

    /// Print the tree without clearing.
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

impl State {
    pub fn new() -> State {
        State {
            root: Arc::new(Mutex::new(Tree::new(None))),
            path: vec![],
            dive_count: 1,
            indent: 2,
        }
    }
    pub fn new_ref() -> StateRef {
        StateRef::new()
    }
    pub fn set_indentation(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn add(&mut self, text: &str) {
        let &dive_count = &self.dive_count;
        if dive_count > 0 {
            for i in 0..dive_count {
                let mut n = 0;
                if let Some(x) = self.root.lock().unwrap().at_mut(&self.path) {
                    x.children.push(Tree::new(if i == dive_count - 1 {
                        Some(&text)
                    } else {
                        None
                    }));
                    n = x.children.len() - 1;
                }
                self.path.push(n);
            }
            self.dive_count = 0;
        } else {
            if let Some(x) = self
                .root
                .lock()
                .unwrap()
                .at_mut(&self.path[..self.path.len() - 1])
            {
                x.children.push(Tree::new(Some(&text)));
                let n = match self.path.last() {
                    Some(&x) => x + 1,
                    _ => 0,
                };
                self.path.last_mut().map(|x| *x = n);
            }
        }
    }
    pub fn step_in(&mut self) {
        self.dive_count += 1;
    }
    pub fn step_out(&mut self) {
        if self.dive_count > 0 {
            self.dive_count -= 1;
        } else {
            self.path.pop();
            assert!(!self.path.is_empty());
        }
    }

    pub fn peek_print(&self) {
        for l in (&self.root.lock().unwrap().lines(&vec![], 0, 1, self.indent))[1..].iter() {
            println!("{}", l);
        }
    }

    pub fn flush_print(&mut self) {
        self.peek_print();
        self.clear();
    }
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn flush_string(&mut self) -> String {
        let s = self.peek_string();
        self.clear();
        s
    }

    pub fn peek_string(&self) -> String {
        (&self.root.lock().unwrap().lines(&vec![], 0, 1, self.indent)[1..]).join("\n")
    }
}

pub struct ScopedBranch {
    state: Option<StateRef>,
}

impl ScopedBranch {
    pub fn new(state: StateRef) -> ScopedBranch {
        state.step_in();
        ScopedBranch { state: Some(state) }
    }
    pub fn release(&mut self) {
        if let Some(x) = &self.state {
            x.step_out();
        }
        self.state = None;
    }
}
impl Drop for ScopedBranch {
    fn drop(&mut self) {
        self.release();
    }
}

pub mod glob {
    use crate::ScopedBranch;
    use crate::StateRef;
    pub fn get() -> StateRef {
        thread_local! {
            static STATE: StateRef = StateRef::new();
        }
        STATE.with(|f| f.clone())
    }

    pub fn set_indentation(indent: usize) {
        get().set_indentation(indent);
    }
    pub fn step_in_scoped() -> ScopedBranch {
        ScopedBranch::new(get().clone())
    }

    pub fn add(text: &str) {
        get().add(&text);
    }
    pub fn step_in() {
        get().step_in();
    }
    pub fn step_out() {
        get().step_out();
    }

    pub fn peek_print() {
        get().peek_print();
    }

    pub fn flush_print() {
        get().flush_print();
    }

    pub fn peek_string() -> String {
        get().peek_string()
    }
    pub fn flush_string() -> String {
        get().flush_string()
    }
    pub fn clear() {
        get().clear()
    }

    #[macro_export]
    macro_rules! dbg_add_glob {
        ($($arg:tt)*) => ($crate::glob::get().add(&format!($($arg)*)));
    }
    #[macro_export]
    macro_rules! dbg_branch_glob {
        () => {
            let _l = $crate::glob::step_in_scoped();
        };
    }
}

#[macro_export]
macro_rules! dbg_add {
    ($state:tt, $($arg:tt)*) => ($state.add(&format!($($arg)*)));
}
#[macro_export]
macro_rules! dbg_branch {
    ($arg:tt) => {
        let _l = $arg.step_in_scoped();
    };
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_branch() {
        let d: StateRef = StateRef::new();
        d.add("1");
        {
            let _l = d.step_in_scoped();
            d.add("1.1");
            d.add("1.2");
        }
        d.add("2");
        d.add("3");
        let _l = d.step_in_scoped();
        d.add("3.1");
        d.add("3.2");
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
        let d = StateRef::new();
        d.add("1");
        {
            let _scope = d.step_in_scoped();
            d.add("1.1");
            {
                let _scope = d.step_in_scoped();
                d.add("1.1.1");
            }
        }

        d.add("2");
        d.step_in();
        d.add("2.1");
        d.step_in();
        d.add("2.1.1");
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
        let d = StateRef::new();
        d.add("Hi");
        assert_eq!("Hi", d.flush_string());
    }
    #[test]
    fn indent() {
        let d = StateRef::new();
        d.add("1");
        dbg_branch!(d);
        d.add("1.1");
        {
            dbg_branch!(d);
            d.add("1.1.1");
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
        let d = StateRef::new();
        dbg_add!(d, "1");
        {
            dbg_branch!(d);
            dbg_add!(d, "1.1")
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
    fn global() {
        dbg_add_glob!("1");
        dbg_branch_glob!();
        dbg_add_glob!("1.1");
        {
            dbg_branch_glob!();
            dbg_add_glob!("1.1.1");
        }
        dbg_add_glob!("1.2");
        glob::peek_print();
        assert_eq!(
            "\
1
├╼ 1.1
│ └╼ 1.1.1
└╼ 1.2",
            glob::flush_string()
        );
    }
    #[test]
    fn global2() {
        dbg_add_glob!("11");
        dbg_branch_glob!();
        dbg_add_glob!("11.1");
        {
            dbg_branch_glob!();
            dbg_add_glob!("11.1.1");
        }
        dbg_add_glob!("11.2");
        glob::peek_print();
        assert_eq!(
            "\
11
├╼ 11.1
│ └╼ 11.1.1
└╼ 11.2",
            glob::flush_string()
        );
    }

    #[test]
    fn mid() {
        let d = StateRef::new();
        d.add(&format!("{}{}", "1", "0"));
        d.step_in();
        d.add("10.1");
        d.add("10.2");
        d.step_in();
        d.add("10.1.1");
        d.add("10.1.2\nNext line");
        d.step_out();
        d.add(&format!("10.3"));
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
