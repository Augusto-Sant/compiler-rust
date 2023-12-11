use crate::lexer::Token;
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn read_json_file() -> Result<Value, Error> {
    let file_path = "syntax_table.json";
    let file_contents = std::fs::read_to_string(file_path).expect("Error reading file");
    let json_value: serde_json::Value =
        serde_json::from_str(&file_contents).expect("Error parsing JSON");
    Ok(json_value)
}

fn print_syntax_tree(node: &SyntaxTreeNode, depth: usize) {
    let indent = " ".repeat(depth * 2);

    // Print the current node's token value (replace with actual token information)
    println!("{}({:#?})", indent, node.token.value); // Adjust this according to your Token struct

    // Recursively print each child
    for child in &node.children {
        print_syntax_tree(child, depth + 1);
    }
}

fn time_sleep(seconds: u64) {
    let sleep_duration = Duration::from_secs(seconds);
    thread::sleep(sleep_duration);
}

#[derive(Debug)]
pub struct SyntaxTreeNode {
    pub token: Token,
    pub children: Vec<Box<SyntaxTreeNode>>,
}

/**
 SLR syntax analysis on tokens
*/
pub fn syntax_parse(tokens: Vec<Token>) -> (bool, Option<Vec<Box<SyntaxTreeNode>>>) {
    println!("- iniciando analisador sintatico");

    let afd = read_json_file().unwrap();
    let mut pilha: Vec<String> = Vec::new();
    pilha.push("0".to_string()); // Convert the initial state to String
    println!("- iniciou com estado 0");

    let mut node_stack: Vec<Box<SyntaxTreeNode>> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        let token_value = &tokens[i].value;

        println!(" - Token: {}", token_value);
        if let Some(action) = afd
            .get(pilha.last().expect("Failed to get last element from pilha"))
            .and_then(|entry| {
                entry["ACTION"]
                    .get(token_value)
                    .or(entry["ACTION"].get("ANY"))
            })
        {
            let move_parts: Vec<&str> = action
                .as_str()
                .expect("Failed to convert action to a string")
                .split(' ')
                .collect();
            println!(" - Action: {}", action);

            match move_parts[0] {
                "S" => {
                    let new_node = Box::new(SyntaxTreeNode {
                        token: tokens[i].clone(),
                        children: Vec::new(),
                    });
                    node_stack.push(new_node);
                    // Shift - Push and advance the pointer
                    println!("  -- Shift operation");
                    let shift_value = move_parts[1].to_string();
                    pilha.push(shift_value);
                    i += 1;
                }
                "R" => {
                    // Reduce - Pop and Redirect (to indicate reduction)
                    println!("  -- Reduce operation");
                    let reduce_count = move_parts[1]
                        .parse::<usize>()
                        .expect("Failed to parse move_parts[1] into usize");

                    pilha.truncate(pilha.len() - reduce_count);
                    let pilha_last = pilha.last().expect("Failed to get last element from pilha");
                    let goto_value = if move_parts.len() > 2 {
                        &afd[pilha_last]["GOTO"][move_parts[2]]
                    } else {
                        panic!("move_parts does not have enough elements to access move_parts[2]");
                    };
                    if goto_value.get(token_value).is_some() {
                        // Handle the case when the first key is not "ANY"
                        let desvio = goto_value[token_value].to_string();
                        pilha.push(desvio);
                    } else if goto_value.get("ANY").is_some() {
                        // Handle the "ANY" case
                        let desvio = goto_value["ANY"].to_string();
                        pilha.push(desvio);
                    }
                    //
                    let mut children = Vec::new();
                    for _ in 0..reduce_count {
                        children.push(node_stack.pop().unwrap());
                    }
                    children.reverse(); // Ensure the children are in the correct order
                    let new_node = Box::new(SyntaxTreeNode {
                        token: Token::new(move_parts[2], "", 0, 0, 0),
                        children,
                    });
                    node_stack.push(new_node);
                }
                "ACC" => {
                    println!("- \x1b[32mOk\x1b[0m Accept operation: Parsing successful");
                    println!("- \x1b[32mSyntax Tree produced:\x1b[0m");
                    print_syntax_tree(&node_stack.get(0).unwrap(), 0);
                    return (true, Some(node_stack));
                }
                _ => {
                    println!("- Error: Invalid action");
                    return (false, None);
                }
            }
            println!("- Current stack: {:?}", pilha);
            println!(" ")
        } else {
            return (false, None);
        }
    }
    (false, None)
}
