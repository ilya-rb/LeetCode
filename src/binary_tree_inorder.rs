use std::cell::{RefCell};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: Option::None,
            right: Option::None,
        }
    }

    pub fn wrap_with_ref_cell(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        TreeNode::inorder_traversal_internal(root, &mut result);
        result
    }

    fn inorder_traversal_internal(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            TreeNode::inorder_traversal_internal(node.left.clone(), result);
            result.push(node.val);
            TreeNode::inorder_traversal_internal(node.right.clone(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree_inorder::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;
    use rstest::rstest;

    #[rstest(expected, case(vec ! [1, 3, 2]))]
    fn test_inorder_traversal(expected: Vec<i32>) {
        let root = create_test_tree();
        assert_eq!(
            expected,
            TreeNode::inorder_traversal(root),
        )
    }

    fn create_test_tree() -> Option<Rc<RefCell<TreeNode>>> {
        let third = TreeNode::new(3).wrap_with_ref_cell();
        let second = TreeNode {
            val: 2,
            right: Option::None,
            left: Some(third),
        }.wrap_with_ref_cell();

        Some(
            TreeNode {
                val: 1,
                left: Option::None,
                right: Some(second),
            }.wrap_with_ref_cell()
        )
    }
}