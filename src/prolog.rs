/* 
    This module translates our AST (Abstract Syntax Tree) into its Prolog representation.
    Chris Kendall
    15 October 2023
*/

use crate::parser::{TreeNode, Declaration, Assignment, Expression, LiteralValue};

// Convert a TreeNode into its Prolog representation
pub fn convert_to_prolog(node: &TreeNode) -> String {
    node.to_prolog()
}

// Trait to define behavior for converting to Prolog
trait ToProlog {
    fn to_prolog(&self) -> String;
}

// Implement the ToProlog trait for TreeNode type
impl ToProlog for TreeNode {
    fn to_prolog(&self) -> String {
        match self {
            // If the node is a data declaration, nothing to convert for Prolog
            TreeNode::Data(_) => String::new(),
            // Convert each assignment in the input section to Prolog and concatenate
            TreeNode::Input(assignments) => {
                assignments.iter().map(|a| a.to_prolog()).collect::<Vec<String>>().join(",\n   ")
            },
            // Convert each assignment in the process section to Prolog and concatenate
            TreeNode::Process(assignments) => assignments.iter().map(|a| a.to_prolog()).collect::<Vec<String>>().join(",\n   "),
            // Convert each output expression to a writeln call in Prolog
            TreeNode::Output(exprs) => {
                exprs.iter().map(|e| {
                    match e {
                        Expression::Literal(LiteralValue::Str(_)) => format!("writeln({})", e.to_prolog()),
                        _ => format!("writeln({})", e.to_prolog())
                    }
                }).collect::<Vec<String>>().join(",\n   ")
            },            
            TreeNode::End => "".to_string(),
        }
    }
}

// Implement the ToProlog trait for Declaration type. (no Prolog rep for this so return empty string)
impl ToProlog for Declaration {
    fn to_prolog(&self) -> String {
        String::new()
    }
}

// Implement the ToProlog trait for Assignment type
impl Assignment {
    fn to_prolog(&self) -> String {
        match self {
            // Convert an assignment to its Prolog representation
            Assignment::Assign(name, expr) => {
                match expr {
                    // If the expression is a function call, convert it to Prolog
                    Expression::FunctionCall(func_name, args) => {
                        let args_str: Vec<String> = args.iter().map(|arg| arg.to_prolog()).collect();
                        match func_name.as_str() {
                            // Special handling for "read" function (i.e. "load_data_column")
                            "read" => {
                                if let [file, false_val, col] = args_str.as_slice() {
                                    return format!("load_data_column({}, {}, {}, V{})", file, false_val, col, name);
                                }
                                String::new() // If the match pattern isn't met, return an empty string
                            },
                            // use basic conversion for other functions
                            _ => format!("{}({}, V{})", func_name.to_lowercase(), args_str.join(", "), name)
                        }
                    },
                    _ => expr.to_prolog()  // directly produce the expression
                }
            }
        }
    }
}

// Implement the ToProlog trait for Expression type
impl Expression {
    fn to_prolog(&self) -> String {
        match self {
            // Convert literals to their Prolog representation
            Expression::Literal(lit) => match lit {
                LiteralValue::Str(s) => format!("\"{}\"", s),  
                LiteralValue::Num(n) => n.to_string(),
                LiteralValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            },
            // Convert identifiers (prefixing with 'V') for variables
            Expression::Identifier(id) => format!("V{}", id),
            // Convert function calls to Prolog (appending function name with 'V') for variables
            Expression::FunctionCall(name, args) => {
                let args_str = args.iter()
                    .map(|arg| arg.to_prolog())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({}, V{})", name.to_lowercase(), args_str, name.to_lowercase())
            }            
        }
    }
}