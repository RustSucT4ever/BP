extern crate bv;
mod common_tree;
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