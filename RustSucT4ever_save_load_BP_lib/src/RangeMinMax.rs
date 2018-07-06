extern crate bv;
use std::vec::Vec;
use bv::{BitVec, Bits};
use std::cmp;
use std::fs::File;
use std::path::Path;

use std::io::prelude::*;
use std::error::Error;



pub struct RangeMinMax {
    blockvector: Vec<Option<Block>>,
    bal_parentheses_vec: BitVec<u8>,
    block_size:u64
}
#[derive(Copy, Clone)]
pub struct Block {
    excess: i64,
    min_ex:i64,
    max_ex:i64,
    count_min_ex:i64,
}

impl RangeMinMax{
    pub fn new (bp_vec:BitVec<u8>, block_size: u64) -> RangeMinMax{
        println!("checkpoint 1");
        let mut m = 0;
        let length = bp_vec.len();
        let mut e = 0;
        let mut M = 0;
        let mut n = 0;
        println!("Vector length {}", length);
        let mut blocks = Vec::<Block>::new();
        println!("checkpoint 1");
        for i in 0 .. length {
            if i%block_size == 0 {
                
                if i!=0 {
                    //Louds {bitString: bits, dataStruct: dataStruct}
                    let b =  Block{excess:e, min_ex:m, max_ex:M, count_min_ex:n};
                    blocks.push(b);
                }
                m = block_size as i64;
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
        println!("checkpoint 1");
        let b =  Block{excess:e, min_ex:m, max_ex:M, count_min_ex:n};
        blocks.push(b);
        let mut block_vecs = Vec::<Vec<Block>>::new();
        block_vecs.push(blocks.clone());
        
        //End of collecting the leafs. At this point the Blocks of the leafs are in a vector that is inside of block_vecs

        let mut last_elem_size = blocks.len();
        println!("Blocks collected {}", blocks.len());
        let mut current_vec =  blocks;
        
        while last_elem_size>1 {
            let mut generate_vec = Vec::<Block> ::new();
            let mut block1=current_vec[0].clone();
            for i in 0..current_vec.len(){
                if i%2 == 0 {
                 block1=current_vec[i].clone();  //check if first or last value is returned 
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
            block_vecs.push(generate_vec.clone());
            current_vec = generate_vec;
            last_elem_size = current_vec.len();
        }
        //Nun wurden die Elternknoten erstellt und in levelweise in block_vecs gepusht, wobei der Wurzelknoten ganz links steht.

        println!("checkpoint 1");

        //Alle Blöcke in einen neuen vec speichern und eventuell lückenfüller hinzufügen

        let mut range_min_max_tree = Vec::<Option<Block>>::new();
        range_min_max_tree.push(Option::None);
        let mut pow = 1;
        //println!("Anzahl an levels {}", block_vecs.len());
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
    
        return RangeMinMax {blockvector: range_min_max_tree, bal_parentheses_vec: bp_vec, block_size: block_size};
    }
/*
    fn fwdsearch(&self, i:u64, d:i64) -> u64{
        let k = i/self.block_size;
        let mut e = 0;
        for j in i+1..((k+1)*self.block_size) {
            if self.bal_parentheses_vec.get_bit(j) {
                e+=1;
            }else{
                e-=1;
            }
            if  e == d {
                return j;
            }
        }
        let mut b = d-(self.excess(k*self.block_size)-self.excess(i));
        //let mut j = (self.blockvector.len()+1)/2 + k as usize; wenn bockvec nicht um eins zu groß
        let mut j = (self.blockvector.len())/2 + k as usize;
        loop{
            if j%2 == 1 {
                j =  (j as usize -1)/2;
            }else {
                j = j+1;
                if self.blockvector[j].unwrap().min_ex<=b && b<= self.blockvector[j].unwrap().max_ex {
                    break;
                    //self.step3( j,d)
                }else{
                    j = (j-1)/2;
                    b = b - self.blockvector[j].unwrap().excess;
                }
            }
        }


        loop{
            let n = (self.blockvector.len())/2;  //(self.blockvector.len()+1)/2 ohne extra block
            let l = n-1;  // n-2 ohne block
            if j> l{
                let mut e = 0;
                for m in (j-l-1)*(self.block_size as usize) .. (j-l-1)*(self.block_size as usize) + self.block_size as usize{
                    if self.bal_parentheses_vec.get_bit(m as u64){
                        e+=1;
                    }else{
                        e-=1;
                    }
                    if e==d {
                        return m as u64;
                    }
                }
            }else{
                let left = j*2;
                let right = j*2 +1;
                if self.blockvector[left].unwrap().min_ex <= b && b<= self.blockvector[left].unwrap().max_ex {
                    j = left;
                }else{
                    j = right;
                    b = b-self.blockvector[left].unwrap().excess;
                }
            }
        }
    }

    fn bwdsearch(&self, i:u64, d:i64) -> u64{
        let k = i/self.block_size; 
        let mut e = 0;
        /*
        for j in i+1..((k+1)*self.block_size) {
            if self.bal_parentheses_vec.get_bit(j) {
                e+=1;
            }else{
                e-=1;
            }
            if  e == b {
                return j;
            }
        }
        */
        let mut s = i;
        while s> k*self.block_size {
            if self.bal_parentheses_vec.get_bit(s) {
                e-=1;
            }else{
                e+=1;
            }
            if  e == d {
                return s-1;
            }
            s-=1;
        }
        let mut b = d-(self.excess(k*self.block_size)-self.excess(i));
        let n = (self.blockvector.len() +1)/2; //weil unser k um eins kleiner als das im paper
        let mut j = n + k as usize;
        loop{
            if j%2 == 0 {
                j =  (j as usize)/2;
            }else {
                j = j-1;
                if self.blockvector[j].unwrap().min_ex<=b && b<= self.blockvector[j].unwrap().max_ex {
                    break;
                    //self.step3( j,d)
                }else{
                    j = j/2;
                    b = b - self.blockvector[j].unwrap().excess;
                }
            }
        }


        loop{
            let n = (self.blockvector.len())/2;  //(self.blockvector.len()+1)/2 ohne extra block
            let l = n-1;  // n-2 ohne block
            //let l = (self.blockvector.len())/2;
            if j>=l{
                let mut e = 0;
                for m in (j-1-l)*(self.block_size as usize) .. (j-1-l)*(self.block_size as usize) + self.block_size as usize{
                    if self.bal_parentheses_vec.get_bit(m as u64){
                        e+=1;
                    }else{
                        e-=1;
                    }
                    if e==b {
                        return m as u64;
                    }
                }
            }else{
                let left = j*2;
                let right = j*2 +1;
                if self.blockvector[left].unwrap().min_ex <= b && b<= self.blockvector[left].unwrap().max_ex {
                    j = left;
                }else{
                    j = right;
                    b = b-self.blockvector[left].unwrap().excess;
                }
            }
        }
    }

        
    fn rmm_rank_one(&self, i: u64) -> u64 {
        //find the block
        let block_vec_index: u64 = i / self.block_size;
        let block_vec_start: u64 = block_vec_index * self.block_size;
        let mut block_index: u64 = (self.blockvector.len()/2 + (block_vec_index as usize)) as u64;

        // count r
        let r: u64 = 0;
        for k in block_vec_start..i{
            if self.bal_parentheses_vec.get_bit(k) {
                r+=1;
            }
        }

        let mut level_ups: u64 = 1;
        while block_index != 1 {            
            if block_index % 2 == 1 {
                //add L's openings
                block_index -= 1;                
                r += (level_ups*self.block_size + self.blockvector[block_index as usize].unwrap().excess) / 2
            }
            //go level up
            block_index /= 2;
            level_ups += 1;
        }

        //ready
        return r;
        }   
    

    fn rmm_rank_zero(&self, i: u64) -> u64 {
        return i - self.rmm_rank_one(i);
    }
    */
}

    fn calc_count_min_excess(block_left:Block, block_right:Block) ->i64{
        if block_left.min_ex < block_left.excess + block_right.min_ex {
            return block_left.count_min_ex
        }
        if block_left.min_ex > block_left.excess + block_right.min_ex {
            return block_right.count_min_ex
        }
        return block_right.count_min_ex + block_left.count_min_ex;

    }

/*
fn calc_excess(tree: RangeMinMax, desired_pos : u32) -> u32{

    let block_pos = desired_pos / tree.block_size;   //abrunden!
    let last_excess = part_excess(tree, block_pos); //berechnet den excess bis zum ende des letzten blocks.
    for i in block_pos .. desired_pos {
        if tree.bal_parentheses_vec[i] == 1{
            last_excess +=1;
        }
        else{
            last_excess -=1;
        }
    }
    return last_excess;

}
*/
/*
fn part_excess(tree:RangeMinMax, rounded_pos:u32) -> u32{
    
    let curr_pos = 1;
    let curr_scope = Tuple::new(0,tree.bal_parentheses_vec.len());
    let curr_excess = 0;
    while curr_pos!= rounded_pos{
        if curr_scope.snd/2 > rounded_pos{
            curr_pos = curr_pos * 2;
            curr_scope.snd = (curr_scope.snd - curr_scope.fst) /2
        }
        else if curr_scope-snd/2 < rounded_pos{
            curr_excess = curr_excess + tree.blockvector[curr_pos*2].of.excess;
            curr_pos = (curr_pos*2)+1
            curr_scope.fst = curr_scope.snd / 2;
        }

        else{
            curr_pos = curr_pos*2
        }
    }
    return curr_excess; 
} */


pub fn save_tree_as_file(tree: RangeMinMax) -> String{
    let path = Path::new("Range_min_max.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                            display,
                            why.description()),
            Ok(file) => file,
        };
        
        println!("file saved ----------------");
        let mut string_repr="".to_owned();
        let mut i=1;
        let mut pow = 2;
        println!("{}", tree.blockvector.len());
        while i<tree.blockvector.len(){
            if tree.blockvector[i].is_none(){
                string_repr.push_str("[NONE]");
            }
            else{
            string_repr.push_str("[excess: ");
            string_repr.push_str(&tree.blockvector[i].unwrap().excess.to_string());
            string_repr.push_str("min excess: ");
            string_repr.push_str(&tree.blockvector[i].unwrap().min_ex.to_string());
            string_repr.push_str("max excess: ");
            string_repr.push_str(&tree.blockvector[i].unwrap().max_ex.to_string());
            string_repr.push_str("#min excess: ");
            string_repr.push_str(&tree.blockvector[i].unwrap().count_min_ex.to_string());
            string_repr.push_str("]");
            }
            if i==pow-1 {
                string_repr.push_str("\n");
                pow = pow*2
            }
            i=i+1;

        }

        string_repr.push_str("\n [");

        for i in 0 .. tree.bal_parentheses_vec.len(){
            if i!=0 && (i%tree.block_size)==0{
                string_repr.push_str("|");
            }
        string_repr.push_str(&tree.bal_parentheses_vec[i].to_string());
        }
        string_repr.push_str("]");
        


            // datei speichern
        match file.write_all(string_repr.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                                why.description())
            },
            Ok(_) => println!("successfully wrote to {}", display),
        }
        return display.to_string();
}