use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex, Weak};

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Tree {
    text: Option<String>,
    children: Vec<Tree>,
}
#[derive(Copy, Clone, Debug)]
enum Position {
    Inside,
    First,
    Last,
    Only,
}

impl Tree {
    fn new(text: Option<&str>) -> Tree {
        Tree {
            text: text.map(|x| x.to_string()),
            children: Vec::new(),
        }
    }

    fn at(&self, path: &[usize]) -> Option<&Tree> {
        match path.first() {
            Some(&i) => match self.children.get(i) {
                Some(x) => x.at(&path[1..]),
                _ => None,
            },
            _ => Some(&self),
        }
    }
    fn at_mut(&mut self, path: &[usize]) -> Option<&mut Tree> {
        match path.first() {
            Some(&i) => match self.children.get_mut(i) {
                Some(x) => x.at_mut(&path[1..]),
                _ => None,
            },
            _ => Some(self),
        }
    }

    /// `does_continue` a bool for each column indicating whether the tree continues
    fn lines(&self, does_continue: &Vec<bool>, index: usize, pool_size: usize) -> Vec<String> {
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
        let mut pad = String::new();
        if does_continue.len() > 1 {
            for &i in &does_continue[2..] {
                txt.push_str(if i { "│ " } else { "  " });
            }
            pad = txt.clone();
            txt.push_str(match position {
                Position::Only | Position::Last => "└╼",
                Position::First | Position::Inside => "├╼",
            });

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
            for line in x.lines(&next_continue, index, self.children.len()) {
                ret.push(line);
            }
        }
        ret
    }
}
#[derive(Debug)]
pub struct State {
    root: Tree,
    path: Vec<usize>,
    dive_count: usize,
}

impl State {
    pub fn new() -> State {
        State {
            root: Tree::new(None),
            path: vec![],
            dive_count: 1,
        }
    }

    fn current(&self) -> Option<&Tree> {
        self.root.at(&self.path)
    }
    fn current_mut(&mut self) -> Option<&mut Tree> {
        self.root.at_mut(&self.path)
    }
    fn current_parent(&self) -> Option<&Tree> {
        self.root.at(&self.path[..self.path.len() - 1])
    }
    fn current_parent_mut(&mut self) -> Option<&mut Tree> {
        self.root.at_mut(&self.path[..self.path.len() - 1])
    }

    pub fn group<F>(&mut self, f: F)
    where
        F: Fn(&mut Self) -> (),
    {
        self.step_in();
        f(self);
        self.step_out();
    }

    pub fn dbg_add(&mut self, text: &str) {
        let &dive_count = &self.dive_count;
        if dive_count > 0 {
            for i in 0..dive_count {
                let mut n = 0;
                if let Some(x) = self.current_mut() {
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
            if let Some(x) = self.current_parent_mut() {
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

    pub fn print(&self) {
        for l in (&self.root.lines(&vec![], 0, 1))[1..].iter() {
            println!("{}", l);
        }
    }

    pub fn to_string(&self) -> String {
        (&self.root.lines(&vec![], 0, 1)[1..]).join("\n")
    }
}

fn get() -> &'static Arc<Mutex<State>> {
    lazy_static! {
        static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
    }
    &(*STATE)
}

pub fn group<F>(f: F)
where
    F: Fn(&mut State) -> (),
{
    get().lock().unwrap().group(f);
}

pub fn dbg_add(text: &str) {
    get().lock().unwrap().dbg_add(&text);
}
pub fn step_in() {
    get().lock().unwrap().step_in();
}
pub fn step_out() {
    get().lock().unwrap().step_out();
}

pub fn print() {
    get().lock().unwrap().print();
}

pub fn to_string() -> String {
    get().lock().unwrap().to_string()
}

#[macro_export]
macro_rules! dbg_add {
    ($($arg:tt)*) => ($crate::dbg_add(&format!($($arg)*)))
}

#[cfg(test)]
mod test {
    use crate::State;
    use crate::*;

    #[test]
    fn simple() {
        dbg_add("Hi");
        assert_eq!("\n   ―Hi", to_string());
    }

    #[test]
    fn mid() {
        dbg_add!("{}{}", "1", "0");
        step_in();
        dbg_add("10.1");
        dbg_add("10.2");
        step_in();
        dbg_add("10.1.1");
        dbg_add("10.1.2\nNext line");
        step_out();
        dbg_add!("10.3");
        print();
        assert_eq!(
            "\
10
├╼ 10.1
├╼ 10.2
│ ├╼ 10.1.1
│ └╼ 10.1.2
│    Next line
└╼ 10.3",
            to_string()
        );
    }
}
