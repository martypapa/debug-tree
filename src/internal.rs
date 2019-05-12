use crate::tree::Tree;
use std::sync::{Arc, Mutex};
/// Holds the current state of the tree, including the path to the branch.
/// Multiple trees may point to the same data.
#[derive(Debug, Clone)]
pub(crate) struct TreeBuilderBase {
    data: Arc<Mutex<Tree>>,
    path: Vec<usize>,
    dive_count: usize,
    indent: usize,
}

impl TreeBuilderBase {
    /// Create a new state
    pub fn new() -> TreeBuilderBase {
        TreeBuilderBase {
            data: Arc::new(Mutex::new(Tree::new(None))),
            path: vec![],
            dive_count: 1,
            indent: 2,
        }
    }

    pub fn set_indentation(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn add_leaf(&mut self, text: &str) {
        let &dive_count = &self.dive_count;
        if dive_count > 0 {
            for i in 0..dive_count {
                let mut n = 0;
                if let Some(x) = self.data.lock().unwrap().at_mut(&self.path) {
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
                .data
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

    pub fn enter(&mut self) {
        self.dive_count += 1;
    }

    /// Try stepping up to the parent tree branch.
    /// Returns false if already at the top branch.
    pub fn exit(&mut self) -> bool {
        if self.dive_count > 0 {
            self.dive_count -= 1;
            true
        } else {
            if self.path.len() > 1 {
                self.path.pop();
                true
            } else {
                false
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.path.len() + self.dive_count - 1
    }

    pub fn peek_print(&self) {
        for l in (&self.data.lock().unwrap().lines(&vec![], 0, 1, self.indent))[1..].iter() {
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
        (&self.data.lock().unwrap().lines(&vec![], 0, 1, self.indent)[1..]).join("\n")
    }
}
