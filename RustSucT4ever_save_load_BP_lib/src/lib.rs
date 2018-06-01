static SOME_BV_TREE: &'static str =
"(()())
";

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate bv;

fn load_bp(file_path: &String) -> String{
    // datei lesen
    // deserialisieren
    // überprüfen ob das geladene auch ein BP ist
    // ausgeben
    return String::from ("")
}

fn save_bp(tree: &String) -> String{ 
    // serialisieren
    //
    // datei speichern
    let path = Path::new("out/our_bv_tree.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(SOME_BV_TREE.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
    return String::from ("")
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
