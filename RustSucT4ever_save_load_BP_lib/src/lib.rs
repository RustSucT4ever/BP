extern crate bv;
mod common_tree;
mod bp;
mod Louds;
mod RangeMinMax;

trait BpLoudsCommonTrait {
    fn isleaf (pos:u64) -> bool;
    fn parent(pos:u64) -> u64;
    fn first_child(pos:u64) -> u64;
    fn next_sibling(pos:u64) -> u64; 
}

/*
#[cfg(test)]
mod tests {
    use bp::load_bp;
    use bp::save_bp;
    use bv::{BitVec, Bits};
    #[test]
    fn load_loads_the_same_that_has_been_saved() {
        // create an example BV tree
        let mut example = BitVec::new();
        example.push(true);
        example.push(true);
        example.push(false);
        example.push(true);
        example.push(false);
        example.push(false);

        // save BV to file
        let example_path = String::from(save_bp(&example));

        // load BV from file
        let checksum = load_bp(&example_path);

        // be happy!
        assert_eq!(checksum, example);
        assert_eq!(checksum.get_bit(0), true);
        assert_eq!(checksum.get_bit(1), true);
        assert_eq!(checksum.get_bit(2), false);
        assert_eq!(checksum.get_bit(3), true);
        assert_eq!(checksum.get_bit(4), false);
        assert_eq!(checksum.get_bit(5), false);
    }
}
*/

#[cfg(test)]
mod tests {
    use RangeMinMax;
    use bv::{BitVec};

    #[test]
    fn test_rank_0(){
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

        let test_tree = RangeMinMax::RangeMinMax::new(example, 4);
         let result = test_tree.rmm_rank_zero(12);
         assert_eq!(result, 5);
    }
    #[test]
    fn test_rank_1(){
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

        let test_tree = RangeMinMax::RangeMinMax::new(example, 4);
         let result = test_tree.rmm_rank_one(7);
         assert_eq!(result, 5);
    }




    #[test]
    fn test_fwd_search(){
        let mut example = BitVec::<u8>::new();
        println!("test running ----------------");
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

        println!("test running ----r------------");
        let test_tree = RangeMinMax::RangeMinMax::new(example, 4);
        let result_index = test_tree.fwdsearch(2, 1);
        println!("result: {} ",result_index );
        println!("expected: 9");
        assert_eq!(result_index, 9);
    }
    #[test]
    fn save_the_tree() {
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

        println!("test running ----r------------");
        let test_tree = RangeMinMax::RangeMinMax::new(example, 4);
        RangeMinMax::save_tree_as_file(test_tree);
     //  assert_eq!(true, true);
    }
}