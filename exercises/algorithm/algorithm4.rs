/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord + Debug,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>, // 移除多余的逗号
}
#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + Debug,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Debug,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Debug,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut existing_node) => {
                if value < existing_node.value {
                    existing_node.left = Self::insert_node(existing_node.left.take(), value);
                } else if value > existing_node.value {
                    existing_node.right = Self::insert_node(existing_node.right.take(), value);
                }
                Some(existing_node)
            }
            None => Some(Box::new(TreeNode::new(value))),
        }
    }

    fn insert(&mut self, value: T) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn search(&self, value: T) -> bool {
        Self::search_node(self.root.as_ref(), value)
    }

    fn search_node(node: Option<&Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            Some(existing_node) => {
                if value == existing_node.value {
                    true
                } else if value < existing_node.value {
                    Self::search_node(existing_node.left.as_ref(), value)
                } else {
                    Self::search_node(existing_node.right.as_ref(), value)
                }
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}