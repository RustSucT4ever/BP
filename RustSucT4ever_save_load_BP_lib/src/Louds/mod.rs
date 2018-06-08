extern crate bv;
extern crate serde;
extern crate serde_json;
use bv::{BitVec, Bits};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

pub fn degree(pos:u64) -> u64 {
    return 1;
}

pub fn child(pos:u64,child_number:u64) -> u64 {
    return 1;
}

pub fn child_rank(pos:u64) -> u64 {
    return 1;
}

pub fn load_louds(file_path: &String) -> BitVec {
    // datei lesen
    let contents = load_file(&file_path);
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

pub fn save_louds(tree: &BitVec) -> String{
    // define where to store file
    let path = Path::new("our_bv_tree.txt");
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

fn load_file(file_path: &String) -> String{
    let mut f = File::open(&file_path).expect("file not found");    
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

