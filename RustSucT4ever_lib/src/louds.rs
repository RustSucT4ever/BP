extern crate bio;
extern crate bv;
use common_tree::BpLoudsCommonTrait;
use bv::{BitVec, Bits};
use louds::bio::data_structures::rank_select::RankSelect;

pub struct Louds {
    bit_string: BitVec<u8>,
    data_struct: RankSelect,
}

impl Louds {
    pub fn degree(&self, x: u64) -> u64{
        self.next_0(x-1) - x
    }
    pub fn child(&self,x: u64, i: u64) -> u64{
        self.data_struct.select_0(self.data_struct.rank_1(x).unwrap()+i-2).unwrap() + 1
    }
    pub fn child_rank (&self, x: u64) -> u64{
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
    pub fn next_0(&self, x: u64) -> u64{
        self.data_struct.select_0(self.data_struct.rank_0(x).unwrap()+1).unwrap()
    }
    pub fn prev_0(&self, x: u64) -> u64{
        self.data_struct.select_0(self.data_struct.rank_0(x).unwrap()).unwrap()
    }
}

impl BpLoudsCommonTrait for Louds {
    fn isleaf (&self, pos:u64) -> bool{
        self.bit_string.get_bit(pos)==false
    }
    fn parent(& self, pos:u64) -> Option<u64>{
        Option::from(self.prev_0(self.data_struct.select_1(self.data_struct.rank_0(pos).unwrap()).unwrap()) +1)
    }

    fn first_child(&self, pos:u64) -> Option<u64>{
        Option::from(self.child(pos, 1))
    }
    fn next_sibling(&self, pos:u64) -> Option<u64>{
        Option::from(self.data_struct.select_0(self.data_struct.select_1(self.data_struct.rank_0(pos-1).unwrap()+1).unwrap()+1).unwrap()+1)
    }
}