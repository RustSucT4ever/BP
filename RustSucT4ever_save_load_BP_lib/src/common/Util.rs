    extern crate bv;
    extern crate serde;
    extern crate serde_json;
    use bv::{BitVec, Bits};
    use std::fs::File;
    use std::io::prelude::*;
    use std::error::Error;
    use std::path::Path;


fn load_file(file_path: &String) -> String{
    let mut f = File::open(&file_path).expect("file not found");    
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}
