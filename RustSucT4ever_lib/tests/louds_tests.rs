extern crate RustSucT4ever_lib;
extern crate bv;

use bv::{BitVec};
use RustSucT4ever_lib::common_tree::BpLoudsCommonTrait;
use RustSucT4ever_lib::louds::Louds;

#[test]
fn test_is_leaf(){
    let test_tree = create_test_tree();
    assert!(test_tree.isleaf(4));
    assert!(test_tree.isleaf(7));
} 

#[test]
fn test_parent(){
    let test_tree : Louds = create_test_tree();
    assert_eq!(test_tree.parent(4),1);
    assert_eq!(test_tree.parent(5),1);
    assert_eq!(test_tree.parent(7),5);
}

#[test]
fn test_first_child(){
    let test_tree : Louds = create_test_tree();
    assert_eq!(test_tree.first_child(1).unwrap(),4);
    assert_eq!(test_tree.first_child(5).unwrap(),7);
}

#[test]
fn test_next_sibling(){
    let test_tree : Louds = create_test_tree();
    assert_eq!(test_tree.next_sibling(4).unwrap(),5);
}  

#[test]
fn test_degree(){
    let test_tree : Louds = create_test_tree();
    assert_eq!(test_tree.degree(1),2);
    assert_eq!(test_tree.degree(4),0);
    assert_eq!(test_tree.degree(5),1);
    assert_eq!(test_tree.degree(7),0);
}  

#[test]
fn test_child(){
    let test_tree : Louds = create_test_tree();
    assert_eq!(test_tree.child(1,1),4);
    assert_eq!(test_tree.child(1,2),5);
    assert_eq!(test_tree.child(5,1),7);
} 

#[test]
fn test_child_rank(){
    let test_tree : Louds = create_test_tree();
    assert_eq!(test_tree.child_rank(1),0);
    assert_eq!(test_tree.child_rank(4),0);
    assert_eq!(test_tree.child_rank(5),1);
    assert_eq!(test_tree.child_rank(7),0);
} 

fn create_test_tree() -> Louds {
    let mut louds_tree = BitVec::new();
        louds_tree.push(true);
        louds_tree.push(true);
        louds_tree.push(true);
        louds_tree.push(false);
        louds_tree.push(false);
        louds_tree.push(true);
        louds_tree.push(false);
        louds_tree.push(false);
    let louds_example = Louds::new(louds_tree, 2);
    return louds_example;
}