/* 
    This module translates our AST (Abstract Syntax Tree) into its Scheme representation.
    Chris Kendall
    15 October 2023
*/

use crate::parser::{TreeNode, Declaration, Assignment, Expression, LiteralValue};

// convert a TreeNode into its Scheme representation
pub fn convert_to_scheme(node: &TreeNode) -> String {
    node.to_scheme()
}

// Interface to convert types to scheme
trait ToScheme {
    fn to_scheme(&self) -> String;
}

// Implementing the ToScheme trait for TreeNode type
impl ToScheme for TreeNode {
    fn to_scheme(&self) -> String {
        // match the type of TreeNode and return its Scheme representation
        match self {
            // data declarations don't have a direct Scheme equivalent, so return an empty string
            TreeNode::Data(_) => String::new(),

            // For input nodes, convert each assignment to Scheme and join them with newlines
            TreeNode::Input(assignments) => {
                assignments.iter().map(|a| a.to_scheme()).collect::<Vec<String>>().join("\n")
            },
            TreeNode::Process(assignments) => assignments.iter().map(|a| a.to_scheme()).collect::<Vec<String>>().join("\n"),
            TreeNode::Output(exprs) => {
                exprs.iter().map(|e| {
                    match e {
                        Expression::Literal(LiteralValue::Str(s)) => format!("(display \"{}\")\n(newline)", s),
                        _ => format!("(display {})\n(newline)", e.to_scheme())
                    }
                }).collect::<Vec<String>>().join("\n")
            },            
            TreeNode::End => "".to_string(),
        }
    }
}

// Implementing the ToScheme trait for Declaration type
impl ToScheme for Declaration {
    fn to_scheme(&self) -> String {
        // declarations don't have a direct Scheme equivalent
        String::new()
    }
}

// Define how Assignments are converted to Scheme
impl Assignment {
    fn to_scheme(&self) -> String {
        match self {
            // If the assignment is a function call, convert it to its Scheme equivalent.
            // For the "read" function, special handling is applied (convert "read" function to "read-csv" function)
            Assignment::Assign(name, expr) => {
                match expr {
                    Expression::FunctionCall(func_name, args) => {
                        let args_str: Vec<String> = args.iter().map(|arg| arg.to_scheme()).collect();
                        match func_name.as_str() {
                            "read" => {
                                if let [file, false_val, col] = args_str.as_slice() {
                                    return format!("(define {} (read-csv \"./{}\" {} {}))", name, file.trim_matches('"'), false_val, col);
                                }
                                String::new()
                            },
                            _ => format!("(define {} ({} {}))", name, func_name.to_lowercase(), args_str.join(" "))
                        }
                    },
                    _ => expr.to_scheme()
                }
            }
        }
    }
}

// Define how Expressions are converted to Scheme
impl Expression {
    fn to_scheme(&self) -> String {
        match self {
            // Convert literals (strings, numbers, booleans) to their Scheme representation
            Expression::Literal(lit) => match lit {
                LiteralValue::Str(s) => format!("\"{}\"", s),  
                LiteralValue::Num(n) => n.to_string(),
                LiteralValue::Bool(b) => if *b { "#t" } else { "#f" }.to_string(),
            },
            // Convert identifiers directly to their name
            Expression::Identifier(id) => id.to_string(),
            // Convert function calls to their Scheme representation
            Expression::FunctionCall(name, args) => {
                let args_str = args.iter()
                    .map(|arg| arg.to_scheme())
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("({} {})", name.to_lowercase(), args_str)
            }            
        }
    }
}
