fn load_BP(file_path: &string) -> (&string){
    // datei lesen
    // deserialisieren
    // überprüfen ob das geladene auch ein BP ist
    // ausgeben
}

fn save_BP(&string) -> (file_path: &string){
    // serialisieren
    // datei speichern
}


#[cfg(test)]
mod tests {
    #[test]
    fn load_loads_the_same_that_has_been_saved() {
        let example = "1100";
        example_path = save_BP(example);
        checksum = load_BP(example_path);
        assert_eq!(checksum, example);
    }
}
