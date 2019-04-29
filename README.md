Debug Tree
===========
This library allows you to build a tree one element at a time and output as a pretty tree string. This is particularly convenient for generating clean output from recursive functions.

Multi-line strings are reformatted to conform to the current branch.

The `dbg_add!` macro is used to add a formatted string leaf to the current branch.

`step_in()` will create a new branch attached to the last added leaf.

`step_out()` will jump up one level out of the current branch.

Output can be retreived using `to_string()` or can be printed using `print()`.

Example
--------
```rust
#[macro_use]
extern crate debug_tree;
dbg_add!("{}{}", "1", "0");

// Create a branch
debug_tree::step_in();
dbg_add!("10.1");
dbg_add!("10.2");
debug_tree::step_in();
dbg_add!("10.1.1");
dbg_add!("10.1.2\nNext line");

// Jump out of branch
debug_tree::step_out();
dbg_add!("10.3");

assert_eq!("\
10
├╼ 10.1
├╼ 10.2
│ ├╼ 10.1.1
│ └╼ 10.1.2
│    Next line
└╼ 10.3", 
debug_tree::to_string);
```