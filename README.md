# A sample tree and parent-child relationship in Rust

## Tree structure

```rust

pub struct Tree {
    pub root: Option<TreeNodeRef>,
}

pub struct TreeNode {
    pub uuid: Uuid,
    pub number: u32,
    pub text: String,
    pub parent: Option<TreeNodeRef>,
    pub children: Option<Vec<TreeNodeRef>>,
}

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
```

## Sample methods

- new
- add_child
- count
- search
- remove
- flatten

### Please look at tests in [src/tree.rs](src/tree.rs) for more details
