:examples: ../examples/

= Debug Tree

This library allows you to build a tree one element at a time and output it as a pretty string.

The tree can easily be output to a `String`, `stdout` or a file.

This is particularly convenient for generating clean output from nested and recursive functions.


:toc:

== Recursive Fibonacci Example

Using the `add_branch!()` macro at the start of the `factors()` function, you can generate an entire call tree, with minimal effort.

[source,rust]
----
use debug_tree::*;

fn factors(x: usize) {
    add_branch!("{}", x); // <~ THE MAGIC LINE
    for i in 1..x {
        if x % i == 0 {
            factors(i);
        }
    }
}

fn main() {
    // output to file at the end of this block
    defer_write!("examples/out/fibonacci.txt");
    add_branch!("A Fibonacci Tree");
    factors(6);
    add_leaf!("That's All Folks!");
}
----

----
A Fibonacci Tree
├╼ 6
│ ├╼ 1
│ ├╼ 2
│ │ └╼ 1
│ └╼ 3
│   └╼ 1
└╼ That's All Folks!
----
== Overview

* Add a branch
- `add_branch!("Hello, {}", "World")`
- The branch will exit at the end of the current block

* Add a leaf
- `add_leaf!("I am a {}", "leaf")`
- Added to the current scoped branch

* Print a tree, or write it to file at the end of a block
- `defer_print!()`
- `defer_write!("filename.txt")`
- The tree will be empty after these calls
- To prevent clearing, use `defer_peek_print!` and `defer_peek_write!`

* Get the trees pretty-string
-

* Handle multiple trees using named trees
- `add_branch_to!("A", "I'm a branch on tree 'A'")`
- `add_leaf_to!("A", "I'm a leaf on tree 'A'")`
- `defer_print!("A")`
- `defer_write!("A", "filename.txt")`

* Get a named tree
- `tree("TREE_NAME")`

* Retrieve the pretty-string from a tree
- `tree("TREE_NAME").string()`


* Usage across threads
- `default_tree()` is local to each thread
- Named trees are shared between threads

== More Examples

=== Multiple Tagged Trees

If you need multiple, separated trees you can use a name tag.

[source,rust]
----
use debug_tree::*;

fn populate(tree_name: &str, n_children: usize) {
    add_branch_to!(tree_name, "{} TREE", tree_name);
    for _ in 0..n_children {
        populate(tree_name, n_children / 2);
    }
}
fn main() {
    // Override tree config (just for "B")
    let b_tree = tree("B");
    b_tree.set_config_override(
        TreeConfig::new()
            .indent(4)
            .symbols(TreeSymbols::with_rounded().leaf("> ")),
    );
    defer_write!(b_tree, "examples/out/multiple_trees_B.txt");
    defer_write!("A", "examples/out/multiple_trees_A.txt");

    populate("A", 2);
    populate("B", 3);
}
----
----
A TREE
├╼ A TREE
│ └╼ A TREE
└╼ A TREE
  └╼ A TREE
----
----
B TREE
├──> B TREE
│   ╰──> B TREE
├──> B TREE
│   ╰──> B TREE
╰──> B TREE
    ╰──> B TREE
----

=== Nested Functions

Branches also make nested function calls a lot easier to follow.

[source,rust]
----
use debug_tree::*;
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
    defer_write!("examples/out/nested.txt");
    a();
}
----
----
a
├╼ b
│ └╼ c
│   └╼ Nothing to see here
└╼ c
  └╼ Nothing to see here
----

=== Line Breaks

Newlines in multi-line strings are automatically indented.

[source,rust]
----
use debug_tree::*;
fn main() {
    // output to file at the end of this block
    defer_write!("examples/out/multi_line.txt");
    add_branch!("1");
    add_leaf!("1.1\nAnother line...\n... and one more line");
    add_leaf!("1.2");
}
----
----
1
├╼ 1.1
│  Another line...
│  ... and one more line
└╼ 1.2
----

=== Panics
Even if there is a panic, the tree is not lost!
The `defer_` functions were introduced to allow the tree
to be printed our written to file in the case of a `panic!` or early return.

[source,rust]
----
use debug_tree::*;

fn i_will_panic() {
    add_branch!("Here are my last words");
    add_leaf!("Stay calm, and try not to panic");
    panic!("I told you so...")
}

fn main() {
    // output to file at the end of this block
    defer_write!("examples/out/panic.txt");
    // print at the end of this block
    {
        add_branch!("By using the 'defer_' functions");
        add_branch!("Output will still be generated");
        add_branch!("Otherwise you might lose your valuable tree!");
    }
    add_branch!("Now for something crazy...");
    i_will_panic();
}
----
----
By using the 'defer_' functions
└╼ Output will still be generated
  └╼ Otherwise you might lose your valuable tree!
Now for something crazy...
└╼ Here are my last words
  └╼ Stay calm, and try not to panic
----


=== Without Macros

If you prefer not using macros, you can construct `TreeBuilder`s manually.

[source,rust]
----
use debug_tree::TreeBuilder;

fn main() {
    // Make a new tree.
    let tree = TreeBuilder::new();

    // Add a scoped branch. The next item added will belong to the branch.
    let mut branch = tree.add_branch("1 Branch");

    // Add a leaf to the current branch
    tree.add_leaf("1.1 Child");

    // Leave scope early
    branch.release();
    tree.add_leaf("2 Sibling");
    // output to file
    tree.write("examples/out/no_macros.txt").ok(); // Write and flush.
}
----
----
1 Branch
└╼ 1.1 Child
2 Sibling
----