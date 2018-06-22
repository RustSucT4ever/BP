extern crate bv;
use std::vec::Vec;
use bv::{BitVec, Bits};
use std::cmp;

struct RangeMinMax {
    blockvector: Vec<Option<Block>>,
}
struct Block {
    excess: u32,
    min_ex:u32,
    max_ex:u32,
    count_min_ex:u32,
}

impl RangeMinMax{
    pub fn new (bp_vec:BitVec<u8>, block_size: u64) -> RangeMinMax{
        let mut m = 0;
        let length = bp_vec.len();
        let mut e = 0;
        let mut M = 0;
        let mut n = 0;
        let mut blocks = Vec::<Block>::new();
        for i in 0 .. length {
            if i%block_size == 0 {
                if i!=0 {
                    //Louds {bitString: bits, dataStruct: dataStruct}
                    let b =  Block{excess:e, min_ex:m, max_ex:M, count_min_ex:n};
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
        let b =  Block{excess:e, min_ex:m, max_ex:M, count_min_ex:n};
        blocks.push(b);
        let mut block_vecs = Vec::<Vec<Block>>::new();
        block_vecs.push(blocks);
        
        //End of collecting the leafs. At this point the Blocks of the leafs are in a vector that is inside of block_vecs

        let mut last_elem_size = blocks.len();
        let mut current_vec =  blocks;
        while last_elem_size>1 {
            let generate_vec = Vec::<Block> ::new();
            let mut block1;
            for i in 0..current_vec.len(){
                if i%2 == 0 {
                 block1=current_vec[i];  //check if first or last value is returned 
                }
                else{
                    block1 = Block{
                    excess:block1.excess+current_vec[i].excess, 
                    min_ex: cmp::min(block1.min_ex,block1.excess+current_vec[i].min_ex),
                    max_ex: cmp::max(block1.max_ex,block1.excess+current_vec[i].max_ex),
                    count_min_ex: calc_count_min_excess(block1, current_vec[i] )
                    };
                    generate_vec.push(block1);
                }

            }
            if current_vec.len()%2!=0{
                generate_vec.push(block1);
            }
            current_vec = generate_vec;
            block_vecs.push(generate_vec);
            last_elem_size = current_vec.len();
        }
        //Nun wurden die Elternknoten erstellt und in levelweise in block_vecs gepusht, wobei der Wurzelknoten ganz links steht.


        //Alle Blöcke in einen neuen vec speichern und eventuell lückenfüller hinzufügen

        let mut range_min_max_tree = Vec::<Option<Block>>::new();
        let pow = 1;
        for i in 0..block_vecs.len(){
            let curr_vec = block_vecs.pop().unwrap();
            for j in 0..pow{
                if j<curr_vec.len(){
                range_min_max_tree.push(Option::from(curr_vec[j]));
                }
                else{
                range_min_max_tree.push(Option::None);
                }
            }
            pow = pow * 2;
        }
    //Die Levels wurden nun auf einen einzelnen vektor gepusht, wobei gilt
    //Für alle elternknoten an position i befinden sich die zugehörigen kinder an position i*2 und i*2+1, leere Knoten sind als None gepusht
    

       return RangeMinMax {blockvector: range_min_max_tree};
    }
        
}
    pub fn calc_count_min_excess(block_left:Block, block_right:Block) ->u32{
        if block_left.min_ex < block_left.excess + block_right.min_ex {
            return block_left.count_min_ex
        }
        if block_left.min_ex > block_left.excess + block_right.min_ex {
            return block_right.count_min_ex
        }
        return block_right.count_min_ex + block_left.count_min_ex;

    }