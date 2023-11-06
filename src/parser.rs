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
    let mut pilha: Vec<&str> = Vec::new();
    pilha.push("0");
    println!("- iniciou com estado 0");

    let mut i = 0;
    const NON_TERMINALS: [&str; 16] = [
        "<PROGRAM>",
        "<VAR_LIST>",
        "<COMMAND_LIST>",
        "<VAR>",
        "<TYPE>",
        "<COMMAND>",
        "<PRINT>",
        "<ASSIGN>",
        "<EXP>",
        "<EXP1>",
        "<EXP2>",
        "<EXP3>",
        "<IF>",
        "<EXP_LOG>",
        "<EXP_LOG_2>",
        "<LOGICAL_OP>",
    ];

    while i < tokens.len() {
        println!("qtd tokens -> {}", tokens.len());
        println!("| Token: {}", tokens[i].value);
        if let Some(action) = afd[pilha.last().unwrap()]["ACTION"].get(&tokens[i].value) {
            let move_parts: Vec<&str> = action.as_str().unwrap().split(' ').collect();
            println!(" | Ação -> {}", action);

            match move_parts[0] {
                "S" => {
                    //  Shift - Empilha e avança o ponteiro
                    println!("SHIFT");
                    let shift_value = move_parts[1];
                    pilha.push(shift_value);
                    i += 1;
                }
                "R" => {
                    // Reduce - Desempilha e Desvia (para indicar a redução)
                    println!("REDUCE");
                    let reduce_count = move_parts[1].parse::<usize>().unwrap();
                    for _ in 0..reduce_count {
                        pilha.pop();
                    }
                    if &tokens[i].value == "SEMICOLON" {
                        pilha.push("14");
                        i += 1;
                    }
                    // println!(
                    //     "reduced pilha -> {:?} {:?}",
                    //     pilha,
                    //     NON_TERMINALS[move_parts[1].parse::<usize>().unwrap()]
                    // );
                    // let desvio = afd[pilha.last().unwrap()]["GOTO"]
                    //     [NON_TERMINALS[move_parts[1].parse::<usize>().unwrap()]][&tokens[i].value]
                    //     .as_str()
                    //     .unwrap();
                    //
                    // pilha.push(desvio);
                }
                "ACC" => {
                    println!("Ok");
                    return true;
                }
                _ => {
                    println!("Erro");
                    return false;
                }
            }

            println!("pilha -> {:?}", pilha)
        } else {
            return false;
        }
        time_sleep(5);
    }

    false
}
