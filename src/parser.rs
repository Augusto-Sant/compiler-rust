use crate::lexer::Token;
use serde_json::{Error, Value};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn untyped_example() -> Result<(), Error> {
    let file_path = Path::new("syntax_table.json");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {:?}", err),
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => {
            let code_text = buffer.to_string();
            code_text
        }
        Err(err) => panic!("Error reading file: {:?}", err),
    };
    let contents = buffer.as_str();
    // Parse the JSON data into serde_json::Value.
    let v: Value = serde_json::from_str(&contents)?;

    // Access parts of the data by indexing with square brackets.
    println!("{}", v["1"]["ACTION"]["b"]["d"]);
    println!("{}", v["1"]);
    println!("??");

    Ok(())
}

/**
 SLR syntax analysis on tokens
*/
pub fn syntax_parse(tokens: Vec<Token>) {
    untyped_example();
}
