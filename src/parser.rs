use crate::lexer::Token;
use serde_json::{Error, Value};
use std::thread;
use std::time::Duration;

fn read_json_file() -> Result<Value, Error> {
    let file_path = "syntax_table.json";
    let file_contents = std::fs::read_to_string(file_path).expect("Error reading file");
    let json_value: serde_json::Value =
        serde_json::from_str(&file_contents).expect("Error parsing JSON");
    Ok(json_value)
}

fn time_sleep(seconds: u64) {
    let sleep_duration = Duration::from_secs(seconds);
    thread::sleep(sleep_duration);
}

/**
 SLR syntax analysis on tokens
*/
pub fn syntax_parse(tokens: Vec<Token>) -> bool {
    println!("- iniciando analisador sintatico");

    let afd = read_json_file().unwrap();
    let mut pilha: Vec<String> = Vec::new();
    pilha.push("0".to_string()); // Convert the initial state to String
    println!("- iniciou com estado 0");

    let mut i = 0;

    while i < tokens.len() {
        let token_value = &tokens[i].value;

        println!(" - Token: {}", token_value);
        if let Some(action) = afd[&pilha.last().unwrap()]["ACTION"]
            .get(token_value)
            .or(afd[&pilha.last().unwrap()]["ACTION"].get("ANY"))
        {
            let move_parts: Vec<&str> = action.as_str().unwrap().split(' ').collect();
            println!(" - Action: {}", action);

            match move_parts[0] {
                "S" => {
                    // Shift - Push and advance the pointer
                    println!("  -- Shift operation");
                    let shift_value = move_parts[1].to_string();
                    pilha.push(shift_value);
                    i += 1;
                }
                "R" => {
                    // Reduce - Pop and Redirect (to indicate reduction)
                    println!("  -- Reduce operation");
                    let reduce_count = move_parts[1].parse::<usize>().unwrap();
                    pilha.truncate(pilha.len() - reduce_count);
                    let goto_value = &afd[&pilha.last().unwrap()]["GOTO"][move_parts[2]];
                    if goto_value.get(token_value).is_some() {
                        // Handle the case when the first key is not "ANY"
                        let desvio = goto_value[token_value].to_string();
                        pilha.push(desvio);
                    } else if goto_value.get("ANY").is_some() {
                        // Handle the "ANY" case
                        let desvio = goto_value["ANY"].to_string();
                        pilha.push(desvio);
                    }
                }
                "ACC" => {
                    println!("- \x1b[32mOk\x1b[0m Accept operation: Parsing successful");
                    return true;
                }
                _ => {
                    println!("- Error: Invalid action");
                    return false;
                }
            }

            println!("- Current stack: {:?}", pilha);
            println!(" ")
        } else {
            return false;
        }
    }
    false
}
