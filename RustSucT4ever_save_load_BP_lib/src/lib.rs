extern crate bv;
mod bp;
mod louds;

trait BpLoudsCommonTrait {
    fn isleaf (pos:u64) -> bool;
    fn parent(pos:u64) -> u64;
    fn first_child(pos:u64) -> u64;
    fn next_sibling(pos:u64) -> u64; 
}

#[cfg(test)]
mod tests {
    use bp::*;
    use louds::*;
    use bv::{BitVec, Bits};

    #[test]
    fn check_bp_save_and_load() {
        // create an example BV tree
        let mut bp_example = BitVec::new();
        bp_example.push(true);
        bp_example.push(true);
        bp_example.push(false);
        bp_example.push(true);
        bp_example.push(false);
        bp_example.push(false);

        // save BV to file
        let mut bp_example_path = String::from(save_bp(&bp_example));

        // load BV from file
        let mut bp_checksum = load_bp(&bp_example_path);

        // be happy!
        assert_eq!(bp_checksum, bp_example);
        assert_eq!(bp_checksum.get_bit(0), true);
        assert_eq!(bp_checksum.get_bit(1), true);
        assert_eq!(bp_checksum.get_bit(2), false);
        assert_eq!(bp_checksum.get_bit(3), true);
        assert_eq!(bp_checksum.get_bit(4), false);
        assert_eq!(bp_checksum.get_bit(5), false);
    }
    
    #[test]
    fn check_louds_save_and_load() {
        // create an example BV tree
        let mut example = BitVec::new();
        example.push(true);
        example.push(true);
        example.push(true);
        example.push(false);
        example.push(true);
        example.push(false);
        example.push(false);
        example.push(false);

        // save BV to file
        let mut example_path = String::from(save_louds(&example));

        // load BV from file
        let mut checksum = load_louds(&example_path);

        // be happy!
        assert_eq!(checksum, example);
        assert_eq!(checksum.get_bit(0), true);
        assert_eq!(checksum.get_bit(1), true);
        assert_eq!(checksum.get_bit(2), true);
        assert_eq!(checksum.get_bit(3), false);
        assert_eq!(checksum.get_bit(4), true);
        assert_eq!(checksum.get_bit(5), false);
        assert_eq!(checksum.get_bit(6), false);
        assert_eq!(checksum.get_bit(7), false);
    }
}