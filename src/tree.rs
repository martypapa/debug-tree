/// Tree that holds `text` for the current leaf and a list of `children` that are the branches.
#[derive(Debug)]
pub struct Tree {
    pub text: Option<String>,
    pub children: Vec<Tree>,
}

/// Position of the element relative to its siblings
#[derive(Copy, Clone, Debug)]
pub enum Position {
    Inside,
    First,
    Last,
    Only,
}

impl Tree {
    /// Create a new tree with some optional text.
    pub fn new(text: Option<&str>) -> Tree {
        Tree {
            text: text.map(|x| x.to_string()),
            children: Vec::new(),
        }
    }

    /// Navigate to the branch at the given `path` relative to this tree.
    /// If a valid branch is found by following the path, it is returned.
    pub fn at_mut(&mut self, path: &[usize]) -> Option<&mut Tree> {
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
    pub fn lines(
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
