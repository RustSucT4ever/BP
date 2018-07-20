extern crate bv;

pub mod common_tree;
pub mod bp;
pub mod louds;
pub mod range_min_max;

#[cfg(test)]
mod load_tests {
    use bp::load_bp;
    use bp::save_bp;
    use bv::{BitVec, Bits};
    use louds::load_louds;
    use louds::save_louds;
    use louds::save_louds_k;
    use louds::load_louds_k;
    
    #[test]
    fn load_bp_the_same_that_has_been_saved() {
        // create an example BV tree
        let mut example = BitVec::new();
        example.push(true);
        example.push(true);
        example.push(false);
        example.push(true);
        example.push(false);
        example.push(false);

        // save BV to file
        let example_path = String::from(save_bp(&String::from("our_bp_tree.txt"), &example));

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

    #[test]
    fn load_louds_the_same_that_has_been_saved() {
        // create an example BV tree
        let mut example = BitVec::new();
        example.push(true);
        example.push(true);
        example.push(true);
        example.push(true);
        example.push(false);
        example.push(true);
        example.push(false);
        example.push(true);
        example.push(false);
        example.push(false);
        example.push(false);

        // save BV to file
        let example_path = String::from(save_louds(&String::from("our_louds_tree.txt"), &example));

        // load BV from file
        let checksum = load_louds(&example_path);

        // be happy!
        assert_eq!(checksum.get_bit(0), true);
        assert_eq!(checksum.get_bit(1), true);
        assert_eq!(checksum.get_bit(2), true);
        assert_eq!(checksum.get_bit(3), true);
        assert_eq!(checksum.get_bit(4), false);
        assert_eq!(checksum.get_bit(5), true);
        assert_eq!(checksum.get_bit(6), false);
        assert_eq!(checksum.get_bit(7), true);
        assert_eq!(checksum.get_bit(8), false);
        assert_eq!(checksum.get_bit(9), false);
        assert_eq!(checksum.get_bit(10), false);
    }

    #[test]
    fn test_load_louds_k() {
        // save BV to file
        let example_path = String::from(save_louds_k(&String::from("louds_k.txt"), 4));

        // load BV from file
        let k = load_louds_k(&example_path);

        // be happy!
        assert_eq!(k,4);
    }   

}

