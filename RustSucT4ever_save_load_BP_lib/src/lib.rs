extern crate bv;
extern crate serde;
extern crate serde_json;
use bv::{BitVec};
use std::fs::File;
use std::io::prelude::*;
static SOME_BV_TREE: &'static str =
"(()())
";
use std::error::Error;
use std::path::Path;

fn load_bp(file_path: &String) -> BitVec {
    let mut f = File::open(&file_path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    let mut bitVec: BitVec = serde_json::from_str(&contents).unwrap();
    return bitVec;
    // datei lesen
    // deserialisieren
    // überprüfen ob das geladene auch ein BP ist
    // ausgeben
}

fn save_bp(tree: &BitVec) -> String{ 
    // serialisieren
    //
    // datei speichern
    let path = Path::new("our_bv_tree.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };
    
    let bv_tree_str : String = serde_json::to_string(tree).unwrap();
    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(bv_tree_str.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
    return String::from ("path------")
}

#[cfg(test)]
mod tests {
    use load_bp;
    use save_bp;
    #[test]
    fn load_loads_the_same_that_has_been_saved() {
        let example = String::from("1100");
        let example_path = String::from(save_bp(&example));
        let checksum = String::from(load_bp(&example_path));
        assert_eq!(checksum, example);
    }
}
