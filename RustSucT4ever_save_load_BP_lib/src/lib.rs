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


fn load_bp(file_path: &String) -> BitVec {
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

fn save_bp(tree: &BitVec) -> String{
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

#[cfg(test)]
mod tests {
    use load_bp;
    use save_bp;
    use bv::{BitVec, Bits};
    #[test]
    fn load_loads_the_same_that_has_been_saved() {
        // create an example BV tree
        let mut example = BitVec::new();
        example.push(true);
        example.push(true);
        example.push(false);
        example.push(true);
        example.push(false);
        example.push(false);

        // save BV to file
        let example_path = String::from(save_bp(&example));

        // load BV from file
        let checksum = load_bp(&example_path);

        // be happy!
        assert_eq!(checksum, example);
        assert_eq!(checksum.get_bit(0), true);
        assert_eq!(checksum.get_bit(1), true);
        assert_eq!(checksum.get_bit(2), false);
        assert_eq!(checksum.get_bit(3), true);
        assert_eq!(checksum.get_bit(4), false);
        assert_eq!(checksum.get_bit(5), false);
    }
}


