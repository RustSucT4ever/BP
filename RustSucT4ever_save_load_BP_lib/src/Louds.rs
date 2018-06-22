extern crate bio;
extern crate bv;
use common_tree::BpLoudsCommonTrait;
use bv::{BitVec, Bits};
use louds::bio::data_structures::rank_select::RankSelect;

struct Louds {
    bitString: BitVec<u8>,
    dataStruct: RankSelect,
}

impl Louds {
    fn degree(&self, x: u64) -> u64{
        self.next_0(x) - x
    }
    fn child(&self,x: u64, i: u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.rank_1(x).unwrap()+i-2).unwrap() + 1
    }
    fn child_rank (&self, x: u64) -> u64{
        let y = self.dataStruct.select_1(self.dataStruct.rank_0(x-1).unwrap()).unwrap();
        y - self.prev_0(y)
    }
    pub fn new(bits: BitVec<u8>) -> Louds{
        let n = bits.bit_len();
        let b = n as f64;
        let mut bit = BitVec::new();
        for i in 0..n {
            bit.push(bits.get_bit(i));
        }
        let k = 2; //change this to the optimal k size like following, but without division by 0:
        //let k = b.ln().powi(2) /(32 as f64);
        // k = (log n)² / 32 
        let dataStruct = RankSelect::new(bit, k as usize);
        Louds {bitString: bits, dataStruct: dataStruct}
    }
    fn next_0(&self, x: u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.rank_0(x).unwrap()+1).unwrap()
    }
    fn prev_0(&self, x: u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.rank_0(x).unwrap()).unwrap()
    }
}

impl BpLoudsCommonTrait for Louds {
    fn isleaf (&self, pos:u64) -> bool{
        self.bitString.get_bit(pos)==false
    }
    fn parent(& self, pos:u64) -> u64{
        self.prev_0(self.dataStruct.select_1(self.dataStruct.rank_0(pos).unwrap()).unwrap()) +1
    }
    fn first_child(&self, pos:u64) -> u64{
        self.child(pos, 1)
    }
    fn next_sibling(&self, pos:u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.select_1(self.dataStruct.rank_0(pos-1).unwrap()+1).unwrap()+1).unwrap()+1
    }
}

//$Env:RUST_BACKTRACE=1
#[cfg(test)]
mod tests {
    use bv::{BitVec, Bits};
    use louds::*;

    #[test]
    fn test_is_leaf(){
        let test_tree : Louds = create_test_tree();
        assert!(test_tree.isleaf(3));
    } 

    #[test]
    fn test_parent(){
        let test_tree : Louds = create_test_tree();
        assert_eq!(test_tree.parent(3),1);
    }

    #[test]
    fn test_first_child(){
        let test_tree : Louds = create_test_tree();
        assert_eq!(test_tree.first_child(1),4);
    }

    #[test]
    fn test_next_sibling(){
        let test_tree : Louds = create_test_tree();
        assert_eq!(test_tree.next_sibling(4),5);
    }  

    #[test]
    fn test_degree(){
        let test_tree : Louds = create_test_tree();
        assert_eq!(test_tree.degree(4),0);
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
        let louds_example = Louds::new(louds_tree);
        return louds_example;
    }
}