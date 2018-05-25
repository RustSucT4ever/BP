fn load_bp(file_path: &String) -> String{
    // datei lesen
    // deserialisieren
    // überprüfen ob das geladene auch ein BP ist
    // ausgeben
    return String::from ("")
}

fn save_bp(tree: &String) -> String{
    // serialisieren
    // datei speichern
    return String::from ("")
}


#[cfg(test)]
mod tests {
    #[test]
    fn load_loads_the_same_that_has_been_saved() {
        let example = String::from("1100");
        let example_path = String::from(save_BP(&example));
        let checksum = String::from(load_BP(example_path));
        assert_eq!(checksum, example);
    }
}
