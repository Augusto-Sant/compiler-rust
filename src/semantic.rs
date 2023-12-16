use crate::lexer::Token;
use crate::parser::SyntaxTreeNode;
use std::collections::HashMap;

// fn get_nested_child<'a>(node: &'a SyntaxTreeNode, levels: usize) -> Option<&'a SyntaxTreeNode> {
//     let mut current_node = node;
//     for _ in 0..levels {
//         match current_node.children.get(0) {
//             Some(child) => current_node = child,
//             None => return None,
//         }
//     }
//     Some(current_node)
// }

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    String,
    Integer,
    // ... other token types ...
}

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
    pub line: i32,
    pub initial_position: i32,
    pub final_position: i32,
}

fn find_terminal_nodes_with_terms(
    node: &SyntaxTreeNode,
    terminal_tokens: &mut Vec<Token>,
    include_terms: &Vec<String>,
    exclude_terms: &Vec<String>,
) {
    for child in &node.children {
        if exclude_terms
            .iter()
            .any(|term| child.token.value.contains(term))
        {
            continue;
        } else if include_terms
            .iter()
            .any(|term| child.token.value.contains(term))
        {
            find_terminal_nodes_with_terms(child, terminal_tokens, include_terms, exclude_terms);
        } else {
            terminal_tokens.push(child.token.clone());
        }
    }
}

fn check_types_exp(exp_node: &SyntaxTreeNode) -> Result<TokenType, SemanticError> {
    // check if types are all the same, and if variable is the same type
    let mut terminal_tokens: Vec<Token> = Vec::new();
    let include_terms: Vec<String> = vec![
        "exp1-nt".to_string(),
        "exp2-nt".to_string(),
        "exp-nt".to_string(),
    ];
    let exclude_terms: Vec<String> = vec![
        "operator-nt".to_string(),
        "EQUAL".to_string(),
        "variable-nt".to_string(),
    ];
    find_terminal_nodes_with_terms(
        exp_node,
        &mut terminal_tokens,
        &include_terms,
        &exclude_terms,
    );
    let first_token_value = terminal_tokens[0].value.clone();
    let is_all_the_same: bool = terminal_tokens
        .iter()
        .all(|token| token.value == first_token_value);

    if is_all_the_same {
        match terminal_tokens.last() {
            Some(token) if token.value == "NUMBER" => return Ok(TokenType::Integer),
            Some(token) if token.value == "STRING" => return Ok(TokenType::String),
            Some(_) => panic!("Type does not Exist"),
            None => panic!("Declaration Type"),
        }
    } else {
        return Err(SemanticError {
            message: "Missing right-hand side of assignment".to_string(),
            line: exp_node.token.line,
            initial_position: exp_node.token.initial_position,
            final_position: exp_node.token.final_position,
        });
    }
}

fn parse_declare(
    node: &SyntaxTreeNode,
    scopes: &mut Vec<HashMap<String, TokenType>>,
    scope_pointer: &mut i32,
) -> Result<(), SemanticError> {
    let mut terminal_tokens: Vec<Token> = Vec::new();
    let include_terms: Vec<String> = vec![
        "exp1-nt".to_string(),
        "exp2-nt".to_string(),
        "variable-nt".to_string(),
    ];
    let exclude_terms: Vec<String> = vec!["operator-nt".to_string()];
    find_terminal_nodes_with_terms(node, &mut terminal_tokens, &include_terms, &exclude_terms);
    let new_variable = &terminal_tokens.first().expect("New declared variable");

    let is_declared = scopes[scope_pointer.clone() as usize].contains_key(&new_variable.lexeme);
    if !is_declared {
        match terminal_tokens.last() {
            Some(token) if token.value == "INTEGER_TYPE" => {
                scopes[scope_pointer.clone() as usize]
                    .insert(new_variable.lexeme.clone(), TokenType::Integer);
            }
            Some(token) if token.value == "STRING_TYPE" => {
                scopes[scope_pointer.clone() as usize]
                    .insert(new_variable.lexeme.clone(), TokenType::String);
            }
            Some(_) => panic!("Type does not Exist"),
            None => panic!("Declaration Type"),
        }
    } else {
        return Err(SemanticError {
            message: format!("Variable {} already declared", new_variable.lexeme).to_string(),
            line: new_variable.line,
            initial_position: new_variable.initial_position,
            final_position: new_variable.final_position,
        });
    }

    Ok(())
}

fn parse_assign(
    node: &SyntaxTreeNode,
    scopes: &mut Vec<HashMap<String, TokenType>>,
    scope_pointer: &mut i32,
) -> Result<(), SemanticError> {
    let mut terminal_tokens: Vec<Token> = Vec::new();
    let include_terms: Vec<String> = vec![
        "exp1-nt".to_string(),
        "exp2-nt".to_string(),
        "variable-nt".to_string(),
    ];
    let exclude_terms: Vec<String> = vec!["operator-nt".to_string()];
    find_terminal_nodes_with_terms(node, &mut terminal_tokens, &include_terms, &exclude_terms);
    let variable = &terminal_tokens.first().expect("expects a Variable");

    let is_declared = scopes
        .iter()
        .any(|scope| scope.contains_key(&variable.lexeme));
    if !is_declared {
        return Err(SemanticError {
            message: format!("Variable {} not assigned", variable.lexeme).to_string(),
            line: variable.line,
            initial_position: variable.initial_position,
            final_position: variable.final_position,
        });
    }

    let inferred_type = check_types_exp(node).expect("expects Inferred Type");
    let variable_type = scopes[scope_pointer.clone() as usize]
        .get(&variable.lexeme)
        .expect("expects variable Type");
    if variable_type != &inferred_type {
        return Err(SemanticError {
            message: format!(
                "Variable {} assigned wrong type {:?}",
                variable.lexeme, inferred_type
            )
            .to_string(),
            line: variable.line,
            initial_position: variable.initial_position,
            final_position: variable.final_position,
        });
    }

    Ok(())
}

fn traverse(
    node: &SyntaxTreeNode,
    scopes: &mut Vec<HashMap<String, TokenType>>,
    scope_pointer: &mut i32,
) -> Result<(), SemanticError> {
    match &node.token.value[..] {
        "assign-nt" => {
            println!("semantic assign-nt: {:?}", &node.token);
            parse_assign(node, scopes, scope_pointer)?;
        }
        "declare-nt" => {
            println!("semantic declare-nt: {:?}", &node.token);
            parse_declare(node, scopes, scope_pointer)?;
        }
        "LEFT_CURLY_BRACE" => {
            *scope_pointer += 1;
            scopes.push(HashMap::new());
        }
        "RIGHT_CURLY_BRACE" => {
            scopes.remove(scope_pointer.clone() as usize);
            *scope_pointer -= 1
        }
        "program-nt" | "command-nt" | "command-list-nt" => {
            for child in &node.children {
                traverse(child, scopes, scope_pointer)?;
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn semantic_analysis(syntax_tree: &Vec<Box<SyntaxTreeNode>>) -> Result<(), SemanticError> {
    println!("Starting semantic analysis");

    let mut scope_pointer = 0;
    let mut scopes: Vec<HashMap<String, TokenType>> = Vec::new();
    scopes.push(HashMap::new());

    for node in syntax_tree {
        traverse(node, &mut scopes, &mut scope_pointer)?;
    }

    println!("Semantic analysis completed.");
    Ok(())
}
