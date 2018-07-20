extern crate RustSucT4ever_lib;
extern crate bv;

use RustSucT4ever_lib::range_min_max;
use bv::{BitVec};

#[test]
fn test_rank_0(){
    let test_tree = create_rmm_test_tree();
    let result = test_tree.rmm_rank_zero(12);
    assert_eq!(result, 5);
}
#[test]
fn test_rank_1(){
    let test_tree = create_rmm_test_tree();
        let result = test_tree.rmm_rank_one(7);
        assert_eq!(result, 5);
}

#[test]
fn test_fwd_search(){
    let test_tree = create_rmm_test_tree();
    let result_index = test_tree.fwdsearch(2, 1).unwrap();
    assert_eq!(result_index, 9);
}
#[test]
fn test_bwd_search(){
    let test_tree = create_rmm_test_tree();
    let result_index = test_tree.bwdsearch(4, -2).unwrap();
    assert_eq!(result_index, 0);
    assert_eq!(test_tree.bwdsearch(17, -1).unwrap(), 14);
    assert_eq!(test_tree.bwdsearch(23, 3).unwrap(), 20);
}

#[test]
fn test_select_one(){
    let test_tree = create_rmm_test_tree();
    assert_eq!(test_tree.rmm_select_one(1), 0);
    assert_eq!(test_tree.rmm_select_one(2), 1);
    assert_eq!(test_tree.rmm_select_one(3), 2);
    assert_eq!(test_tree.rmm_select_one(4), 4);
    assert_eq!(test_tree.rmm_select_one(5), 7);
    assert_eq!(test_tree.rmm_select_one(6), 8);
    assert_eq!(test_tree.rmm_select_one(7), 9);
    assert_eq!(test_tree.rmm_select_one(8), 12);
    assert_eq!(test_tree.rmm_select_one(9), 15);
    assert_eq!(test_tree.rmm_select_one(10), 16);
    assert_eq!(test_tree.rmm_select_one(11), 18);
    assert_eq!(test_tree.rmm_select_one(12), 20);
}

#[test]
fn test_select_zero(){
    let test_tree = create_rmm_test_tree();
    assert_eq!(test_tree.rmm_select_zero(1), 3);
    assert_eq!(test_tree.rmm_select_zero(2), 5);
    assert_eq!(test_tree.rmm_select_zero(3), 6);
    assert_eq!(test_tree.rmm_select_zero(4), 10);
    assert_eq!(test_tree.rmm_select_zero(5), 11);
    assert_eq!(test_tree.rmm_select_zero(6), 13);
    assert_eq!(test_tree.rmm_select_zero(7), 14);
    assert_eq!(test_tree.rmm_select_zero(8), 17);
    assert_eq!(test_tree.rmm_select_zero(9), 19);
    assert_eq!(test_tree.rmm_select_zero(10), 21);
    assert_eq!(test_tree.rmm_select_zero(11), 22);
    assert_eq!(test_tree.rmm_select_zero(12), 23);
}

#[test]
fn save_the_tree() {
    let test_tree = create_rmm_test_tree();
    range_min_max::save_tree_as_file(test_tree);
    //  assert_eq!(true, true);
}

fn create_rmm_test_tree() -> range_min_max::RangeMinMax {
    // create an example BV tree
    let mut example = BitVec::<u8>::new();

    example.push(true);
    example.push(true);
    example.push(true);
    example.push(false);
    example.push(true);

    example.push(false);
    example.push(false);
    example.push(true);
    example.push(true);
    
    example.push(true);
    example.push(false);
    example.push(false);
    example.push(true);

    example.push(false);
    example.push(false);
    example.push(true);
    example.push(true);

    example.push(false);
    example.push(true);
    example.push(false);
    example.push(true);
    
    example.push(false);
    example.push(false);
    example.push(false);

    let test_tree = range_min_max::RangeMinMax::new(example, 4);
    return test_tree;
}
