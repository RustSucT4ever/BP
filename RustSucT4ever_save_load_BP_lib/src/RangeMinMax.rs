extern crate bv;
use std::vec::Vec;
use bv::{BitVec, Bits};

struct RangeMinMax {
    blockvector: Vec<Block>,
}
struct Block {
    e: u32,
    m:u32,
    M:u32,
    n:u32,
}

impl RangeMinMax{
    pub fn new (bp_vec:BitVec<u8>, block_size: u32) -> RangeMinMax{
        let mut m = 0;
        let length = bp_vec.len();
        let mut e = 0;
        let mut M = 0;
        let mut n = 0;
        let mut blocks = Vec::<Block>::new();
        for i in 0 .. length {
            if i%4 == 0 {
                if i!=0 {
                    //Louds {bitString: bits, dataStruct: dataStruct}
                    let b =  Block{e:e, m:m, M:M, n:n};
                    blocks.push(b);
                }
                blocks = Vec::<Block>::new();
                m = 0;
                e = 0;
                M = 0;
                n = 0;
            }
            if bp_vec.get_bit(i) {
                e+=1;
            }else{
                e-=1;
            }
            if e>M {
                M = e;
            }
            if e<m {
                m = e;
                n = 0;
            }
            if e == m {
                n+=1;
            }
        }
        let mut block_vec = Vec::<Block>::new();
        let temp = blocks.len()  as f64;
        let l = ((temp.log(2.0)) + 0.5).round().powi(2) as usize;
        //block_vec.reserve(l*2 -1);
        //let k = l*2 -1;
        //block_vec[0] = Block{m:m,e:e,M:M,n:n};
        //for i in 0 .. blocks.len() {
        //    block_vec[k-1-i-(l-blocks.len())] = blocks.pop().unwrap();
        //}
        //let begin = k-l;

        for i in 0 .. l {
            if i<blocks.len() {

            }
        }
       

        RangeMinMax {blockvector: block_vec}
    }
}