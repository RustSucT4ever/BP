extern crate bv;
extern crate serde;
extern crate serde_json;
use common_tree::*;
use bv::{BitVec, Bits};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use range_min_max::RangeMinMax;

pub struct Bp {
    tree: RangeMinMax
}

impl BpLoudsCommonTrait for Bp {
    fn isleaf (&self, pos:u64) -> bool{
        self.maybe_panic(pos);
        return self.tree.get_bit(pos +1) == false;
    }
    fn parent(& self, pos:u64) -> Option<u64>{
        if pos == 0 {
            return None;
        }
        self.maybe_panic(pos);
        let possible = self.tree.bwdsearch(pos, -2);
        if possible.is_none() {
            return Option::from(0);
        }
        return Option::from(possible.unwrap()+1);
    }
    fn first_child(&self, pos:u64) -> Option<u64>{
        self.maybe_panic(pos);
        if self.isleaf(pos){
            return None;
        }
        return Option::from(pos+1);
    }
    fn next_sibling(&self, pos:u64) -> Option<u64>{
        self.maybe_panic(pos);
        let n = self.tree.fwdsearch(pos, -1).unwrap()+1;
        if self.tree.get_bit(n) {
            return Option::from(n);
        }else{
            return None;
        }
    }
}

impl Bp {
    pub fn new(bits: BitVec<u8>, block_size: u64) -> Bp{
        Bp {tree: RangeMinMax::new(bits, block_size)}
    }

    pub fn pre_rank(&self, pos:u64) -> u64 {
        return  self.tree.rmm_rank_one(pos);
    }

    pub fn pre_select(&self, pos:u64) -> u64 {
        return self.tree.rmm_select_one(pos);
    }

    pub fn depth(&self, pos:u64) -> u64 {
        self.maybe_panic(pos);
        return self.tree.calc_excess(pos);
    }
    pub fn find_close(&self, pos:u64) -> u64{
        self.maybe_panic(pos);
        return self.tree.fwdsearch(pos,-1).unwrap();
    }

    pub fn ancestor(&self, pos_x:u64,pos_y:u64) -> bool {
        self.maybe_panic(pos_x);
        self.maybe_panic(pos_y);

        return pos_x<=pos_y && pos_y<=self.find_close(pos_x);
    }

    pub fn subtree_size(&self, pos:u64) -> u64 {
        self.maybe_panic(pos);
        return (self.find_close(pos)-pos+1)/2;
    }
    
    pub fn load_file(file_path: &String) -> String{
        let mut f = File::open(&file_path).expect("file not found");    
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        return contents;
    }    
    pub fn maybe_panic(&self, pos: u64){
        if pos >= self.tree.size(){
            panic!("Index {} is out of bounds.", pos);
        }
        if !self.tree.get_bit(pos) {
            panic!("Index {} is not the beginning of a node.", pos);
        }
    }

    pub fn is_legit_index(&self, pos: u64)->bool{
        if pos >= self.tree.size(){
            return false;
        }
        if !self.tree.get_bit(pos) {
            return false;
        }
        return true;
    }
}



pub fn load_bp(file_path: &String) -> BitVec {
        // datei lesen
        let contents = Bp::load_file(&file_path);
        // deserialisieren
        let  bit_vec: BitVec = serde_json::from_str(&contents).unwrap();
        // überprüfen ob das geladene auch ein BP ist
        let l = bit_vec.len();
        let mut correct = true; 
        let mut count = 0;
        for i in 0..l {
            if bit_vec.get_bit(i) == true {
                count = count+1;
            }
            if bit_vec.get_bit(i) == false {
                count = count-1;
                if i != l-1 && count<=0 {
                    correct = false;
                }
            }
        }

        if count>0 {
            correct = false;
        }

        if !correct {
            println!("Falscher String!");
        }

        // ausgeben
        return bit_vec;
    }

pub fn save_bp(filepath: &String, tree: &BitVec) -> String{
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
