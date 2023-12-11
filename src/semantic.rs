use crate::parser::SyntaxTreeNode;
use std::collections::HashMap;

fn get_nested_child<'a>(node: &'a SyntaxTreeNode, levels: usize) -> Option<&'a SyntaxTreeNode> {
    let mut current_node = node;
    for _ in 0..levels {
        match current_node.children.get(0) {
            Some(child) => current_node = child,
            None => return None,
        }
    }
    Some(current_node)
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Variable,
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

fn infer_type_from_exp(exp_node: &SyntaxTreeNode) -> Result<TokenType, SemanticError> {
    // Extend this logic to handle more complex expressions
    if exp_node.token.value == "NUMBER" {
        Ok(TokenType::Integer)
    } else {
        Err(SemanticError {
            message: "Unsupported expression type".to_string(),
            line: exp_node.token.line,
            initial_position: exp_node.token.initial_position,
            final_position: exp_node.token.final_position,
        })
    }
}

fn parse_assign(
    node: &SyntaxTreeNode,
    scopes: &mut Vec<HashMap<String, TokenType>>,
) -> Result<(), SemanticError> {
    let variable_node = get_nested_child(node, 2).expect("Variable Node");
    let var_name = &variable_node.token.lexeme;

    // Check if variable is already declared in any scope
    let is_declared = scopes.iter().any(|scope| scope.contains_key(var_name));

    // If not declared, infer type from the right-hand side of the assignment
    if !is_declared {
        let rhs_node = node.children.get(2).ok_or_else(|| SemanticError {
            message: "Missing right-hand side of assignment".to_string(),
            line: node.token.line,
            initial_position: node.token.initial_position,
            final_position: node.token.final_position,
        })?;
        let inferred_type = infer_type_from_exp(rhs_node)?;

        // Add the new variable to the current scope
        scopes
            .last_mut()
            .unwrap()
            .insert(var_name.clone(), inferred_type);
    }

    Ok(())
}

fn traverse(
    node: &SyntaxTreeNode,
    scopes: &mut Vec<HashMap<String, TokenType>>,
) -> Result<(), SemanticError> {
    match &node.token.value[..] {
        "assign-nt" => {
            println!("S: {:?}", &node.token);
            parse_assign(node, scopes)?;
        }
        "scope-begin" => {
            scopes.push(HashMap::new());
        }
        _ => {}
    }

    for child in &node.children {
        traverse(child, scopes)?;
    }

    if &node.token.value == "scope-begin" {
        scopes.pop();
    }

    Ok(())
}

pub fn semantic_analysis(syntax_tree: &Vec<Box<SyntaxTreeNode>>) -> Result<(), SemanticError> {
    println!("Starting semantic analysis");

    let mut scopes: Vec<HashMap<String, TokenType>> = Vec::new();
    scopes.push(HashMap::new());

    for node in syntax_tree {
        traverse(node, &mut scopes)?;
    }

    println!("Semantic analysis completed.");
    Ok(())
}
