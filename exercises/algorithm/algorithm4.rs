/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::SeekFrom;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
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
    T: Ord+std::fmt::Display,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root.as_mut(){
            Some(root)=> root.insert(value),
            None=>self.root = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)))   
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut cmp_node =  self.root.as_ref();
        while cmp_node.is_some(){
            println!("compare {},{}",cmp_node.unwrap().value,value);
            if cmp_node.unwrap().value > value{
                cmp_node = cmp_node.unwrap().left.as_ref();
            }
            else if cmp_node.unwrap().value < value{
                cmp_node = cmp_node.unwrap().right.as_ref();
            }
            else{
                return true;
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if self.value > value{
            if let Some(left) = self.left.as_mut(){
                left.insert(value);
            }
            else{
                self.left = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
            }
        }
        else if self.value < value{
            if let Some(right) = self.right.as_mut(){
                right.insert(value);
            }
            else {
                self.right = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
            }
        }
        else{
            return;
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


