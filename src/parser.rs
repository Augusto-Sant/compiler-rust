use crate::lexer::Token;
use serde_json::{Error, Value};
use std::thread;
use std::time::Duration;

fn read_json_file() -> Result<Value, Error> {
    let file_path = "tabela.json";
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
    let mut pilha: Vec<&str> = Vec::new();
    pilha.push("0");
    println!("- iniciou com estado 0");

    let mut i = 0;
    const NON_TERMINALS: [&str; 3] = ["EXP", "EXP2", "EXP3"];

    while i < tokens.len() {
        let token_count = tokens.len();
        let token_value = &tokens[i].value;

        println!(" - Token: {}", token_value);

        if let Some(action) = afd[pilha.last().unwrap()]["ACTION"].get(token_value) {
            let move_parts: Vec<&str> = action.as_str().unwrap().split(' ').collect();
            println!(" - Action: {}", action);

            match move_parts[0] {
                "S" => {
                    // Shift - Push and advance the pointer
                    println!("  -- Shift operation");
                    let shift_value = move_parts[1];
                    pilha.push(shift_value);
                    i += 1;
                }
                "R" => {
                    // Reduce - Pop and Redirect (to indicate reduction)
                    println!("  -- Reduce operation");
                    let reduce_count = move_parts[1].parse::<usize>().unwrap();
                    let non_terminal_index = move_parts[2].parse::<usize>().unwrap();
                    pilha.truncate(pilha.len() - reduce_count);

                    let goto_value = &afd[pilha.last().unwrap()]["GOTO"]
                        [NON_TERMINALS[non_terminal_index]][token_value];
                    let desvio = goto_value.as_str().unwrap();

                    pilha.push(desvio);
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
