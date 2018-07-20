extern crate RustSucT4ever_lib;
extern crate bv;

use bv::{BitVec};
use RustSucT4ever_lib::common_tree::BpLoudsCommonTrait;
use RustSucT4ever_lib::louds::Louds;
use std::time::{Duration, Instant};

#[test]
fn test_is_leaf(){
    let test_tree = create_test_tree();
    assert!(test_tree.isleaf(4));
    assert!(test_tree.isleaf(7));
} 

#[test]
fn test_parent(){
    let test_tree : Louds = create_test_tree();
    assert_eq!(test_tree.parent(4).unwrap(),1);
    assert_eq!(test_tree.parent(5).unwrap(),1);
    assert_eq!(test_tree.parent(7).unwrap(),5);
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

#[test]
fn test_big_louds() {
    let test_tree : Louds = create_big_louds_tree(1000);
    let start = Instant::now();
    let nextThing = test_tree.next_0(7);
    
    //let theValue = test_tree.degree(1281); // for 1.000 nodes
    //let theValue = test_tree.degree(1499777); // for 1.000.000 nodes
    //let theValue = test_tree.degree(14999809); // for 10.000.000 nodes
    let finish = Instant::now();
    println!("{:?}", finish.duration_since(start));
}    

fn create_big_louds_tree(nodes_number: u64) -> Louds {
    // create an example BV tree
    let mut example = BitVec::new();
    let mut ones = nodes_number;
    let mut zeroes = nodes_number;
    example.push(true);
    while ones > 0 {
        let mut children = ones / 2;
        if ones == 1 {
            children = 1;
        }
        for i in 1..children {                    
            example.push(true);
        }                    
        example.push(false);
        ones -= children;
        zeroes -= 1;
    }
    for i in 1..zeroes {                    
        example.push(false);
    }
    
    let louds_example = Louds::new(example, 2); 

    return louds_example;
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