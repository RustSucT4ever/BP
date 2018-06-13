extern crate bio;
extern crate bv;
use bv::{BitVec, Bits};
use Louds::bio::data_structures::rank_select::RankSelect;
trait BpLoudsCommonTrait {
    fn isleaf (&self,pos:u64) -> bool;
    fn parent(&self,pos:u64) -> u64;
    fn first_child(&self,pos:u64) -> u64;
    fn next_sibling(&self,pos:u64) -> u64; 
}
struct Louds {
    bitString: BitVec<u8>,
    dataStruct: RankSelect,
}

impl Louds {
    fn degree(&self, x: u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.rank_0(x).unwrap()+1).unwrap() - x
    }
    fn child(&self,x: u64, i: u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.rank_1(x).unwrap()+i-1).unwrap() + 1
    }
    fn child_rank (&self, x: u64) -> u64{
        let y = self.dataStruct.select_1(self.dataStruct.rank_0(x-1).unwrap()).unwrap();
        y - self.dataStruct.select_0(self.dataStruct.rank_0(y).unwrap()).unwrap()
    }
    pub fn new(bits: BitVec<u8>) -> Louds{
        let n = bits.bit_len();
        let b = n as f64;
        let mut bit = BitVec::new();
        for i in 0..n {
            bit.push(bits.get_bit(i));
        }
        let k = b.ln().powi(2) /(32 as f64);
        // k = (log n)² / 32 
        let dataStruct = RankSelect::new(bit, k as usize);
        Louds {bitString: bits, dataStruct: dataStruct}
    }
}

impl BpLoudsCommonTrait for Louds {
    fn isleaf (&self, pos:u64) -> bool{
        self.bitString.get_bit(pos)==false
    }
    fn parent(& self, pos:u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.rank_0(self.dataStruct.select_1(self.dataStruct.rank_0(pos-1).unwrap()).unwrap()).unwrap()).unwrap() +1
    }
    fn first_child(&self, pos:u64) -> u64{
        self.child(pos, 1)
    }
    fn next_sibling(&self, pos:u64) -> u64{
        self.dataStruct.select_0(self.dataStruct.select_1(self.dataStruct.rank_0(pos-1).unwrap()+1).unwrap()+1).unwrap()
    }
}