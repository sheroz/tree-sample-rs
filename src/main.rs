mod tree;

use tree::{Tree, TreeNode, TreeNodeRef, TreeNodeRefBuild};

pub fn main() {
    let mut tree = Tree::new();

    let root = TreeNode::new();
    let root_ref = TreeNodeRef::build_from(root);
    tree.root = Some(root_ref.clone());

    let node1 = TreeNode::new();
    let node1_ref = TreeNodeRef::build_from(node1);
    tree.add_child(root_ref, node1_ref.clone());

    let child1 = TreeNode::new();
    tree.add_child(node1_ref.clone(), TreeNodeRef::build_from(child1));

    let child2 = TreeNode::new();
    tree.add_child(node1_ref.clone(), TreeNodeRef::build_from(child2));

    assert_eq!(tree.count(), 4);
    
    let flatten = tree.flatten().iter().map(|n| n.as_ref().borrow().uuid.to_string()).collect::<Vec<_>>().join("\n");
    println!("Nodes flatten:\n{}", flatten);
}