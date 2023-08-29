# Tree and parent-child relationship in Rust

Tree and parent-child relationship samples for learning purposes.

## Tree structure

```rust

pub struct Tree {
    pub root: Option<TreeNodeRef>,
}

pub struct TreeNode {
    pub uuid: Uuid,
    pub number: u32,
    pub text: String,
    pub parent: TreeNodeWeakRef,
    pub children: Option<Vec<TreeNodeRef>>,
}

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
pub type TreeNodeWeakRef = Weak<RefCell<TreeNode>>;
```

## Sample methods

- new
- add_child
- count
- search
- remove
- flatten

Please look at tests in [src/tree.rs](src/tree.rs) for more details

## Useful insights

- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
