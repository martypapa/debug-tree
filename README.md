Debug Tree
===========
This library allows you to build a tree one element at a time and output as a pretty tree string. This is particularly convenient for generating clean output from nested and recursive functions. A design goal was to allow this library to be used as a drop-in replacement of `println!(...)`. 

Simple Usage
--------
```rust
#[macro_use]
extern crate debug_tree;
use debug_tree::default_tree;
fn main() {
    {
        add_branch!("{} Branch", "1"); // Enter branch 1
        // tree is now pointed inside new branch.
        add_leaf!("{} Child", "1.1");
    } // Exit branch because scope ends.
    add_leaf!("2 Sibling");
    default_tree().flush_print();
}
```

```
1 Branch
└╼ 1.1 Child
2 Sibling
 ```

Line Breaks
---------
Newlines in multi-line strings are automatically indented.
```rust
#[macro_use]
extern crate debug_tree;
use debug_tree::default_tree;
fn main() {
    {
        add_branch!("1");
        add_leaf!("1.1\nNext line");
    }
    add_leaf!(&format!("1.2"));
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
```rust
extern crate debug_tree;
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