extern crate bio;
extern crate bv;
extern crate serde;
extern crate serde_json;
use common_tree::BpLoudsCommonTrait;
use bv::{BitVec, Bits};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
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
    pub fn load_file(file_path: &String) -> String{
        let mut f = File::open(&file_path).expect("file not found");    
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        return contents;
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


pub fn load_louds(file_path: &String) -> BitVec {
        // datei lesen
        let contents = Louds::load_file(&file_path);
        // deserialisieren
        let  bit_vec: BitVec = serde_json::from_str(&contents).unwrap();
        let mut count_ones = 0;
        let mut count_zeroes = 0;
        //überprüfen, ob es auch ein louds ist.
        for i in 0.. bit_vec.len() {
            if bit_vec.get_bit(i)==true{
                count_ones = count_ones +1;
            }
            else{
                count_zeroes = count_zeroes +1
            }
        }
        if count_ones != count_zeroes+1{
            println!("falscher string!einsen: {}, nullen: {}", count_ones, count_zeroes);
            return BitVec::new();
        }

        // ausgeben
        return bit_vec;
    }

pub fn load_louds_k(file_path: &String) -> usize {
        // datei lesen
        let contents = Louds::load_file(&file_path);
        // deserialisieren
        let  k: usize = serde_json::from_str(&contents).unwrap();

        // ausgeben
        return k;
    }


pub fn save_louds_k(filepath: &String, k: usize) -> String{
    // define where to store file
    let path = Path::new(filepath);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                        display,
                        why.description()),
        Ok(file) => file,
    };
    
    // serialisieren
    let k_str : String = serde_json::to_string(&k).unwrap();

    // datei speichern
    match file.write_all(k_str.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                            why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
    return display.to_string();
}


pub fn save_louds(filepath: &String, tree: &BitVec<u8>) -> String{
    // define where to store file
    let path = Path::new(filepath);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                        display,
                        why.description()),
        Ok(file) => file,
    };
    
    // serialisieren
    let bv_tree_str : String = serde_json::to_string(tree).unwrap();

    // datei speichern
    match file.write_all(bv_tree_str.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                            why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
    return display.to_string();
}


