extern crate bv;
use bv::{BitVec, Bits};
use std::fs::File;
use std::io::prelude::*;
mod bp;
mod louds;

trait BpLoudsCommonTrait {
    fn isleaf (pos:u64) -> bool;
    fn parent(pos:u64) -> u64;
    fn first_child(pos:u64) -> u64;
    fn next_sibling(pos:u64) -> u64; 
}

fn load_from_manual_file(file_path:String) -> BitVec{
    // datei lesen
    let contents = load_file(&file_path);
    let mut loaded_vec = BitVec::new();
    for i in 0..contents.len(){
        let to_push = contents.chars().nth(i)==Some('(')||contents.chars().nth(i)==Some('1');
        loaded_vec.push(to_push);
    }
    return loaded_vec
}
 
fn load_file(file_path: &String) -> String {
    let mut f = File::open(&file_path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

#[cfg(test)]
mod tests {
    use bp::*;
    use louds::*;
    use bv::{BitVec, Bits};
    use load_from_manual_file;

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
    #[test]
    fn loads_from_manual_file(){
        let bit_vec = load_from_manual_file("manualTree.txt".to_owned());
        assert_eq!(bit_vec.get_bit(0), true);
        assert_eq!(bit_vec.get_bit(1), true);
        assert_eq!(bit_vec.get_bit(2), false);
        assert_eq!(bit_vec.get_bit(3), true);
        assert_eq!(bit_vec.get_bit(4), false);
        assert_eq!(bit_vec.get_bit(5), false);
    }
}