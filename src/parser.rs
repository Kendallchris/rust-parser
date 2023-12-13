/* 
    This module runs syntactical analysis on the generated Tokens from the Lexical Analysis
    Chris Kendall
    15 October 2023
*/

use crate::lexer::Token;

#[derive(Debug)]
pub enum TreeNode {
    Data(Vec<Declaration>),
    Input(Vec<Assignment>),
    Process(Vec<Assignment>),
    Output(Vec<Expression>),
    End,
}

#[derive(Debug)]
pub enum Declaration {
    Vector(String),
    Number(String),
}

#[derive(Debug)]
pub enum Assignment {
    Assign(String, Expression),
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralValue), // For strings and numbers
    Identifier(String),
    FunctionCall(String, Vec<Expression>), // function name and arguments
    
}

#[derive(Debug)]
pub enum LiteralValue {
    Str(String),
    Num(u32),
    Bool(bool),
}

// this will be our parse tree - provides methods for parsing
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// parse tree functions
impl Parser {

    // Constructor to initialize the parser with tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // main function to parse tokens into tree nodes
    pub fn parse(&mut self) -> Result<Vec<TreeNode>, String> {
        let mut nodes = Vec::new();
        
        while self.current < self.tokens.len() {
            if self.check_and_advance_token(Token::DATA) {
                nodes.push(self.parse_data()?);
            } else if self.check_and_advance_token(Token::INPUT) {
                nodes.push(self.parse_input()?);
            } else if self.check_and_advance_token(Token::PROCESS) {
                nodes.push(self.parse_process()?);
            } else if self.check_and_advance_token(Token::OUTPUT) {
                nodes.push(self.parse_output()?);
            } else if self.check_and_advance_token(Token::END) {
                // Check for the PERIOD token after END
                if self.check_and_advance_token(Token::PERIOD) {
                    nodes.push(TreeNode::End);
                    break;  // Exit the parsing loop as "END." signifies the end of the program
                } else {
                    return Err("Expected '.' after 'END'".to_string());
                }
            } else {
                return Err(format!("Unexpected token: {:?}", self.tokens[self.current]));
            }
        }
        Ok(nodes)
    }

    // Checks if current Token matches the expected and advances current by 1
    fn check_and_advance_token(&mut self, expected: Token) -> bool {
        if self.current >= self.tokens.len() {
            return false;
        }
        if self.tokens[self.current] == expected {
            self.current += 1;
            true
        } else {
            false
        }
    }

    // Parses the "data" section /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    fn parse_data(&mut self) -> Result<TreeNode, String> {
        // Ensure that the token after "DATA" is a colon.
        if !self.check_and_advance_token(Token::COLON) {
            return Err("Expected ':' after DATA".to_string());
        }
    
        let mut declarations = Vec::new();
        while self.current < self.tokens.len() {
            if self.tokens[self.current] == Token::INPUT {
                break;
            }
            declarations.push(self.parse_declaration()?);
        }
        Ok(TreeNode::Data(declarations))
    }
    
    // Parses declarations
    fn parse_declaration(&mut self) -> Result<Declaration, String> {
        if let Some(Token::ID(name)) = self.tokens.get(self.current).cloned() {
            self.current += 1;  // Move past the identifier
    
            if !self.check_and_advance_token(Token::COLON) {
                return Err("Expected ':' after declaration name".to_string());
            }
    
            if self.check_and_advance_token(Token::VECTOR) {
                self.check_and_advance_token(Token::COMMA);  // Try to match comma and move past it if it's there
                return Ok(Declaration::Vector(name));
            } else if self.check_and_advance_token(Token::NUMBER) {
                self.check_and_advance_token(Token::COMMA);  // Try to match comma and move past it if it's there
                return Ok(Declaration::Number(name));
            }
            // Handle assignment case if needed.
        }
        Err("Invalid declaration".to_string())
    }    
    
    // Parses expressions
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let current_token = self.tokens[self.current].clone();
        
        // Check for literals first
        match current_token {
            Token::STRING(s) => {
                self.current += 1;
                return Ok(Expression::Literal(LiteralValue::Str(s)));
            }
            Token::NUM(n) => {
                self.current += 1;
                return Ok(Expression::Literal(LiteralValue::Num(n)));
            }
            Token::TRUE => {
                self.current += 1;
                return Ok(Expression::Literal(LiteralValue::Bool(true)));
            }
            Token::FALSE => {
                self.current += 1;
                return Ok(Expression::Literal(LiteralValue::Bool(false)));
            }
            _ => {} // if none of the previous, do nothing and continue
        }
        
        // Now check for identifiers and function calls
        if let Token::ID(name) = &current_token {
            if self.check_and_advance_token(Token::LPAREN) {
                let mut arguments = Vec::new();
    
                // Loop until we see a closing parenthesis
                while !self.check_and_advance_token(Token::RPAREN) {
                    arguments.push(self.parse_expression()?);
    
                    if !self.check_and_advance_token(Token::COMMA) && self.tokens[self.current] != Token::RPAREN {
                        return Err(format!("Expected ',' or ')' in function arguments, found {:?}", self.tokens[self.current]));
                    }
                }
                return Ok(Expression::FunctionCall(name.clone(), arguments));
            } else {
                self.current += 1;
                return Ok(Expression::Identifier(name.clone()));
            }
        } else if current_token == Token::REGRESSIONA 
                || current_token == Token::REGRESSIONB 
                || current_token == Token::CORRELATION
                || current_token == Token::MEAN 
                || current_token == Token::STDDEV {
            let function_name = match current_token {
                Token::REGRESSIONA => "REGRESSIONA",
                Token::REGRESSIONB => "REGRESSIONB",
                Token::CORRELATION => "CORRELATION",
                Token::MEAN => "MEAN",
                Token::STDDEV => "STDDEV",
                _ => unreachable!(),  // we should never hit this case based on our condition
            };
    
            self.current += 1;  // Move past the function name
    
            if self.check_and_advance_token(Token::LPAREN) {
                let mut arguments = Vec::new();
    
                // Expect one argument for mean and stddev, two for the others
                let expected_args = if function_name == "MEAN" || function_name == "STDDEV" { 1 } else { 2 };
    
                for i in 0..expected_args {
                    arguments.push(self.parse_expression()?);
    
                    if i < expected_args - 1 && !self.check_and_advance_token(Token::COMMA) {
                        return Err(format!("Expected ',' after argument of {}", function_name));
                    }
                }
    
                if !self.check_and_advance_token(Token::RPAREN) {
                    return Err(format!("Expected ')' after arguments of {}", function_name));
                }
    
                return Ok(Expression::FunctionCall(function_name.to_string(), arguments));
            } else {
                return Err(format!("Expected '(' after {}", function_name));
            }
        }
    
        Err(format!("Invalid expression for token: {:?}", self.tokens[self.current]))
    }
    

    // Parses the "input" section /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    fn parse_input(&mut self) -> Result<TreeNode, String> {
        // Ensure that the token after "input" is a colon
        if !self.check_and_advance_token(Token::COLON) {
            return Err("Expected ':' after input".to_string());
        }
    
        let mut assignments = Vec::new();
        while self.current < self.tokens.len() && !self.check_and_advance_token(Token::PROCESS) {  // Ensure we don't go out of bounds
            // parse assignment before checking for a comma, indicating another
            assignments.push(self.parse_input_op()?);
    
            // If there's no comma, we don't expect another INPUTOP, so break out of the loop
            if !self.check_and_advance_token(Token::COMMA) {
                break;
            }
        }
        Ok(TreeNode::Input(assignments))
    }
    
    fn parse_input_op(&mut self) -> Result<Assignment, String> {
        // make sure that the token is an ID - if so, make a copy of the Token to work with
        if let Some(Token::ID(name)) = self.tokens.get(self.current).cloned() {
            self.current += 1;  // Move past the identifier
    
            // make sure the next Token is an '=' and advance iterator if so
            if !self.check_and_advance_token(Token::ASSIGN) {
                return Err("Expected '=' in input operation".to_string());
            }
    
            // make sure the next Token is a 'read' and advance iterator if so
            if !self.check_and_advance_token(Token::READ) {
                return Err("Expected 'read' function in input operation".to_string());
            }
            
            // make sure the next Token is a '(' and advance iterator if so
            if !self.check_and_advance_token(Token::LPAREN) {
                return Err("Expected '(' after 'read'".to_string());
            }
    
            // make sure the next Token is a STRING - if so, assign the value to prompt
            let prompt = if let Token::STRING(s) = &self.tokens[self.current] {
                self.current += 1;
                s.clone()
            } else {
                return Err("Expected STRING argument for 'read' function".to_string());
            };

            // make sure the next Token is a COMMA
            if !self.check_and_advance_token(Token::COMMA) {
                return Err("Expected ',' in input operation".to_string());
            }
    
            // make sure the next Token is a "true" or "false" and assign value to echo
            let echo = if self.check_and_advance_token(Token::TRUE) {
                true
            } else if self.check_and_advance_token(Token::FALSE) {
                false
            } else {
                return Err("Expected BOOL argument for 'read' function".to_string());
            };

            // make sure the next Token is a COMMA
            if !self.check_and_advance_token(Token::COMMA) {
                return Err("Expected ',' in input operation".to_string());
            }
    
            // make sure the next Token is a NUM - if so, assign to length
            let length = if let Token::NUM(n) = self.tokens[self.current] {
                self.current += 1;
                n
            } else {
                return Err("Expected NUM argument for 'read' function".to_string());
            };
            
            // make sure the next Token is a '(' and advance iterator if so
            if !self.check_and_advance_token(Token::RPAREN) {
                return Err("Expected ')' after 'read' arguments".to_string());
            }
    
            // store all 3 values into the FunctionCall variant of the Expression enum
            let read_expr = Expression::FunctionCall(
                "read".to_string(),
                vec![
                    Expression::Literal(LiteralValue::Str(prompt)),
                    Expression::Literal(LiteralValue::Bool(echo)),
                    Expression::Literal(LiteralValue::Num(length)),
                ],
            );
    
            // store all of this into Assignment enum
            Ok(Assignment::Assign(name, read_expr))
        } else {
            Err("Invalid input operation".to_string())
        }
    }
    
    // Parses the process section /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    fn parse_process(&mut self) -> Result<TreeNode, String> {
        
        // Ensure that the token after "PROCESS" is a colon
        if !self.check_and_advance_token(Token::COLON) {
            return Err("Expected ':' after PROCESS".to_string());
        }
    
        let mut assignments = Vec::new();
        while self.current < self.tokens.len() && !self.check_and_advance_token(Token::OUTPUT) {  // Ensure we don't go out of bounds
            // parse assignment before checking for a comma, indicating another
            assignments.push(self.parse_process_op()?);
    
            // If there's no comma, we don't expect another PROCESSOP, so break out of the loop
            if !self.check_and_advance_token(Token::COMMA) {
                break;
            }
        }
        Ok(TreeNode::Process(assignments))
    }
        
    fn parse_process_op(&mut self) -> Result<Assignment, String> {
        // make sure that the token is an ID - if so, make a copy of the Token to work with
        if let Some(Token::ID(name)) = self.tokens.get(self.current).cloned() {
            self.current += 1;  // Move past the identifier
    
            // make sure the next Token is an '=' and advance iterator if so
            if !self.check_and_advance_token(Token::ASSIGN) {
                return Err("Expected '=' in process operation".to_string());
            }
    
            // Parse the right-hand expression which represents a function call or some computation
            let rhs_expression = self.parse_expression()?;  // Assuming the parse_expression function handles function calls too
    
            // store the variable and its expression into Assignment enum
            Ok(Assignment::Assign(name, rhs_expression))
        } else {
            Err("Invalid process operation".to_string())
        }
    }        

    // Parses the output section /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    fn parse_output(&mut self) -> Result<TreeNode, String> {
            
        // Ensure that the token after "OUTPUT" is a colon
        if !self.check_and_advance_token(Token::COLON) {
            return Err("Expected ':' after OUTPUT".to_string());
        }
    
        let mut expressions = Vec::new();
        while self.current < self.tokens.len() {
            // Parse the expression and push it to our list of expressions
            expressions.push(self.parse_expression()?);
    
            // If there's no comma, we don't expect another expression, so break out of the loop
            if !self.check_and_advance_token(Token::COMMA) {
                break;
            }
        }
        Ok(TreeNode::Output(expressions))
    }
}

