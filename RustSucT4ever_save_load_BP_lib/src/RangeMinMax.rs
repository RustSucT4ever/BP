extern crate bv;
use Vec;
use bv::{BitVec, Bits};

struct RangeMinMax {
    blockvector: Vec<Block>;
}
struct Block {
    e: u32;
    m:u32;
    M:u32;
    n:u32;
}

impl RangeMinMax{
    pub fn new (bp_vec:BitVec<u8>, block_size: u32) -> RangeMinMax{
        let mut block_vec = Vec::new();
        bp_vec.
    }
}