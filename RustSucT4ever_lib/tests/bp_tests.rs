extern crate RustSucT4ever_lib;
extern crate bv;

use bv::{BitVec};
use RustSucT4ever_lib::common_tree::BpLoudsCommonTrait;
use RustSucT4ever_lib::bp::*;

#[test]
fn test_pre_rank(){
    let test_tree = create_test_tree();
    assert_eq!(test_tree.pre_rank(0), 1);
    assert_eq!(test_tree.pre_rank(1), 2);
    assert_eq!(test_tree.pre_rank(2), 2);
    assert_eq!(test_tree.pre_rank(3), 3);
    assert_eq!(test_tree.pre_rank(4), 3);
    assert_eq!(test_tree.pre_rank(5), 3);
}

#[test]
fn test_pre_select(){
    let test_tree = create_test_tree();
    assert_eq!(test_tree.pre_select(1), 0);
    assert_eq!(test_tree.pre_select(2), 1);
    assert_eq!(test_tree.pre_select(3), 3);
}

#[test]
fn test_ancestor(){
    let test_tree = create_test_tree();
    assert!(test_tree.ancestor(0,3));
    assert!(test_tree.ancestor(0,1));
    assert!(!test_tree.ancestor(1,3));
}

#[test]
fn test_subtree_size(){
    let test_tree = create_test_tree();
    assert_eq!(test_tree.subtree_size(0), 3);
    assert_eq!(test_tree.subtree_size(1), 1);
    assert_eq!(test_tree.subtree_size(3), 1);
}

#[test]
fn test_depth(){
    let test_tree = create_test_tree();
    assert_eq!(test_tree.depth(0), 1);
    assert_eq!(test_tree.depth(1), 2);
    assert_eq!(test_tree.depth(3), 2);
}

#[test]
fn test_isleaf(){
    let test_tree = create_test_tree();
    assert!(!test_tree.isleaf(0));
    assert!(test_tree.isleaf(1));
    assert!(test_tree.isleaf(3));
}

#[test]
fn test_parent(){
    let test_tree = create_test_tree();
    assert_eq!(test_tree.parent(1), 0);
    assert_eq!(test_tree.parent(3), 0);
}

#[test]
fn test_first_child(){
    let test_tree = create_test_tree();
    assert_eq!(test_tree.first_child(0).unwrap(), 1);
}

#[test]
fn test_next_sibling(){
    let test_tree = create_test_tree();
    assert_eq!(test_tree.next_sibling(1).unwrap(), 3);
}

fn create_test_tree() -> Bp {
    let mut bp_tree = BitVec::new();
        bp_tree.push(true);
        bp_tree.push(true);
        bp_tree.push(false);
        bp_tree.push(true);
        bp_tree.push(false);
        bp_tree.push(false);
    let bp_example = Bp::new(bp_tree, 2);
    return bp_example;
}