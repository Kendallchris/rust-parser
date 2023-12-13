/* 
    This module runs Lexical Analysis, generating a list of Tokens to represent the source string
    Chris Kendall
    15 October 2023
*/

// lexer enum
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    DATA,
    INPUT,
    PROCESS,
    OUTPUT,
    END,
    ID(String),
    NUM(u32),
    TRUE,
    FALSE,
    READ,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    VECTOR,
    NUMBER,
    REGRESSIONA,
    REGRESSIONB,
    MEAN,
    STDDEV,
    CORRELATION,
    STRING(String),
}

// This function performs lexical analysis on the given source string.
// It scans the string character by character to produce a list of tokens.
// If something does not match a possible token, it returns a lexical error.
pub fn lexical_analysis(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\n' | '\t' => continue, // Skip whitespace and newline
            ':' => tokens.push(Token::COLON),
            ',' => tokens.push(Token::COMMA),
            '.' => tokens.push(Token::PERIOD),
            '(' => tokens.push(Token::LPAREN),
            ')' => tokens.push(Token::RPAREN),
            '=' => tokens.push(Token::ASSIGN),

            // handle strings - denoted by '"'
            '"' => {
                let mut string_content = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch != '"' {
                        string_content.push(chars.next().unwrap());
                    } else {
                        chars.next();  // Consume the closing quotation mark
                        break;
                    }
                }
                tokens.push(Token::STRING(string_content));
            }

            // handle numbers
            '0'..='9' => {
                let mut num = ch.to_string();
                while let Some(next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        num.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                // convert string num to unsigned int (u32)
                let pars_result = num.parse::<u32>();
                if let Ok(parsed_num) = pars_result {
                    tokens.push(Token::NUM(parsed_num));
                } else {
                    return Err(format!("Failed to parse number: {}", num));
                }
            }

            // Recognize identifiers, numbers, strings, etc.
            'a'..='z' => {
                let mut id = ch.to_string();
                while let Some(next_ch) = chars.peek() {
                    if next_ch.is_ascii_alphabetic() {
                        id.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                // Check if id is a keyword - if not then store it as an ID
                match id.as_str() {
                    "data" => tokens.push(Token::DATA),
                    "input" => tokens.push(Token::INPUT),
                    "process" => tokens.push(Token::PROCESS),
                    "output" => tokens.push(Token::OUTPUT),
                    "end" => tokens.push(Token::END),
                    "true" => tokens.push(Token::TRUE),
                    "false" => tokens.push(Token::FALSE),
                    "read" => tokens.push(Token::READ),
                    "vector" => tokens.push(Token::VECTOR),
                    "number" => tokens.push(Token::NUMBER),
                    "regressiona" => tokens.push(Token::REGRESSIONA),
                    "regressionb" => tokens.push(Token::REGRESSIONB),
                    "mean" => tokens.push(Token::MEAN),
                    "stddev" => tokens.push(Token::STDDEV),
                    "correlation" => tokens.push(Token::CORRELATION),

                    _ => tokens.push(Token::ID(id)),
                }
        
            }

            // If we encounter an unrecognized character, return an error
            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }

    Ok(tokens)
}