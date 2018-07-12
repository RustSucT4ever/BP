extern crate bio;
extern crate bv;
use common_tree::BpLoudsCommonTrait;
use bv::{BitVec, Bits};
use louds::bio::data_structures::rank_select::RankSelect;

struct Louds {
    bit_string: BitVec<u8>,
    data_struct: RankSelect,
}

impl Louds {
    fn degree(&self, x: u64) -> u64{
        self.next_0(x-1) - x
    }
    fn child(&self,x: u64, i: u64) -> u64{
        self.data_struct.select_0(self.data_struct.rank_1(x).unwrap()+i-2).unwrap() + 1
    }
    fn child_rank (&self, x: u64) -> u64{
        if x == 1 {
            return 0
        }
        else {
            let y = self.data_struct.select_1(self.data_struct.rank_0(x-1).unwrap()).unwrap() + 1;
            let z = self.prev_0(y);
            if z > 0 {        
                y - z
            }
            else {
                y - 1
            }
        }
    }
    pub fn new(bits: BitVec<u8>, k: usize) -> Louds{
        let n = bits.bit_len();
        let b = n as f64;
        let mut bit = BitVec::new();
        for i in 0..n {
            bit.push(bits.get_bit(i));
        }
        //let k = 2; //change this to the optimal k size like following, but without division by 0:
        //let k = b.ln().powi(2) /(32 as f64);
        // k = (log n)² / 32 
        let data_struct = RankSelect::new(bit, k);
        Louds {bit_string: bits, data_struct: data_struct}
    }
    fn next_0(&self, x: u64) -> u64{
        self.data_struct.select_0(self.data_struct.rank_0(x).unwrap()+1).unwrap()
    }
    fn prev_0(&self, x: u64) -> u64{
        self.data_struct.select_0(self.data_struct.rank_0(x).unwrap()).unwrap()
    }
}

impl BpLoudsCommonTrait for Louds {
    fn isleaf (&self, pos:u64) -> bool{
        self.bit_string.get_bit(pos)==false
    }
    fn parent(& self, pos:u64) -> u64{
        self.prev_0(self.data_struct.select_1(self.data_struct.rank_0(pos).unwrap()).unwrap()) +1
    }
    fn first_child(&self, pos:u64) -> Option<u64>{
        Option::from(self.child(pos, 1))
    }
    fn next_sibling(&self, pos:u64) -> Option<u64>{
        Option::from(self.data_struct.select_0(self.data_struct.select_1(self.data_struct.rank_0(pos-1).unwrap()+1).unwrap()+1).unwrap()+1)
    }
}

//$Env:RUST_BACKTRACE=1
#[cfg(test)]
mod tests {
    use bv::{BitVec};
    use louds::*;

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
}