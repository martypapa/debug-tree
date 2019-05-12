Debug Tree
===========
This library allows you to build a tree one element at a time and output it as a pretty string. This is particularly convenient for generating clean output from nested and recursive functions. A design goal was to allow this library to be used as a drop-in replacement of `println!(...)`. 

Recursive Example
--------
By adding the `add_branch!(...)` macro at the start of a recursive function, you can see the entire call tree, instantly.
```rust
use debug_tree::{default_tree, add_branch, add_leaf};
fn factors(x: usize) {
    add_branch!("{}", x);
    for i in 1..x {
        if x % i == 0 {
            factors(i);
        }
    }
}
fn main() {
    factors(6);
    default_tree().flush_print();
}
```
```
6
├╼ 1
├╼ 2
│ └╼ 1
└╼ 3
 └╼ 1
```

Nested Example
---------------
Branches also make nested function calls a lot easier to follow.
```rust
use debug_tree::{default_tree, add_branch, add_leaf};
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

fn main() {
    a();
    default_tree().flush_print();
}
```
```
a
├╼ b
│ └╼ c
│   └╼ Nothing to see here
└╼ c
  └╼ Nothing to see here
```

Line Breaks
---------
Newlines in multi-line strings are automatically indented.
```rust
use debug_tree::{default_tree, add_branch, add_leaf};
fn main() {
    add_branch!("1");
    add_leaf!("1.1\nNext line");
    add_leaf!("1.2");
    default_tree().flush_print();
}
```
```
1
├╼ 1.1
│  Next line
└╼ 1.2
```


Non-Macro Version
------------
In the case that multiple trees are needed, the trees can be created manually without the helper macros.
```rust
use debug_tree::TreeBuilder;
fn main() {
    // Make a new tree.
    let tree = TreeBuilder::new();
    
    // Add a scoped branch. The next item added will belong to the branch.
    let branch = tree.add_branch("1 Branch"); 
    
    // Add a leaf to the current branch
    tree.add_leaf("1.1 Child");
    
    // Leave scope early
    branch.release();
    tree.add_leaf("2 Sibling"); 
    
    tree.flush_print(); // Print and clear.
    // default_tree().peek_print(); // Would print but not clear.
}

```

```
1 Branch
└╼ 1.1 Child
2 Sibling
```
