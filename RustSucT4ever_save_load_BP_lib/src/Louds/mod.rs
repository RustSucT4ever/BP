extern crate bio;
extern crate bv;
use bv::{BitVec, Bits};
use Louds::bio::data_structures::rank_select::RankSelect;

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
        y - self.dataStruct.select_0(self.dataStruct.rank_0(x).unwrap()).unwrap()
    }
    pub fn new(bits: BitVec<u8>) -> Louds{
        let n = bits.bit_len();
        let b = n as f64;
        let k = b.ln().powi(2) /(32 as f64);
        // k = (log n)² / 32 
        let dataStruct = RankSelect::new(bits, k as usize);
        Louds {bitString: bits, dataStruct: dataStruct}
    }
}