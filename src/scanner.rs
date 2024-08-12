
use crate::MyError;


pub struct Scanner<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
    line_number: u64,
}
#[derive(Debug)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    IDENTIFIER,
    STRING,
    NUMBER,

    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}


impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum LiteralValue{
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
    
}
#[derive(Debug)]
pub struct Token {
    token_type:TokenType,
    lexeme:String,
    literal: Option<LiteralValue>,
    line_number:u64
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: u64) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut scanner = Scanner {
            input,
            position: 0,
            current_char: None,
            line_number:0,
            

        };
        scanner.advance(); // Initialize the current character
        scanner
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = self.input[self.position..].chars().next();
            self.position += self.current_char.unwrap_or('\0').len_utf8();
        } else {
            self.current_char = None;
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, MyError> {
        self.skip_whitespace();

        let token_type = match self.current_char {
            Some('(') => TokenType::LEFT_PAREN,
            Some(')') => TokenType::RIGHT_PAREN,
            Some('{') => TokenType::LEFT_BRACE,
            Some('}') => TokenType::RIGHT_BRACE,
            Some(',') => TokenType::COMMA,
            Some('.') => TokenType::DOT,
            Some('-') => TokenType::MINUS,
            Some('+') => TokenType::PLUS,
            Some(';') => TokenType::SEMICOLON,
            Some('*') => TokenType::STAR,
            Some('!') => {
                if let Some(nextchar) = self.forward_look() {
                    if nextchar == '=' {
                        self.advance(); // Consume the '=' in "!="
                        TokenType::BANG_EQUAL
                    } else {
                        TokenType::BANG
                    }
                } else {
                    TokenType::BANG
                }
            },
            Some('=') => {
                if let Some(nextchar) = self.forward_look() {
                    if nextchar == '=' {
                        self.advance(); // Consume the '=' in "=="
                        TokenType::EQUAL_EQUAL
                    } else {
                        TokenType::EQUAL
                    }
                } else {
                    TokenType::EQUAL
                }
            },
            Some('<') => {
                if let Some(nextchar) = self.forward_look() {
                    if nextchar == '=' {
                        self.advance(); // Consume the '=' in "<="
                        TokenType::LESS_EQUAL
                    } else {
                        TokenType::LESS
                    }
                } else {
                    TokenType::LESS
                }
            },
            Some('>') => {
                if let Some(nextchar) = self.forward_look() {
                    if nextchar == '=' {
                        self.advance(); // Consume the '=' in ">="
                        TokenType::GREATER_EQUAL
                    } else {
                        TokenType::GREATER
                    }
                } else {
                    TokenType::GREATER
                }
            },
            Some('/') => {
                if let Some(nextchar) = self.forward_look() {
                    if nextchar == '/' {
                        while let Some(c) = self.current_char {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        return self.next_token(); 
                    } else {
                        TokenType::SLASH
                    }
                } else {
                    TokenType::SLASH
                }
            },
            Some('\n') => {
                self.line_number += 1;
                self.advance();
                return self.next_token(); 
            },

            Some(c) if c.is_digit(10) => {  
                let start = self.position;

                while let Some(c) = self.current_char {
                    if c.is_digit(10) {
                        self.advance();
                    } else {
                        break;
                    }
                }

                if self.current_char == Some('.') {
                    self.advance();
                    while let Some(c) = self.current_char {
                        if c.is_digit(10) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let lexeme = &self.input[start..self.position];
                    return Ok(Some(Token::new(
                        TokenType::NUMBER,
                        lexeme.to_string(),
                        Some(LiteralValue::FValue(lexeme.parse::<f64>().unwrap())),
                        self.line_number,
                    )));
                }
            
                let lexeme = &self.input[start..self.position];
                return Ok(Some(Token::new(
                    TokenType::NUMBER,
                    lexeme.to_string(),
                    Some(LiteralValue::IntValue(lexeme.parse::<i64>().unwrap())),
                    self.line_number,
                )));
            },

            Some(c) if c.is_alphanumeric() => {
                dbg!(self.position);
                let start = self.position ;
                while let Some(c) = self.current_char {
                    if c.is_whitespace() {
                        break
                    }
                    if c.is_alphanumeric() || c == '_' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let lexeme = self.input[start -1..self.position].trim();
                let token_type = match lexeme {
                    "and" => TokenType::AND,
                    "class" => TokenType::CLASS,
                    "else" => TokenType::ELSE,
                    "false" => TokenType::FALSE,
                    "for" => TokenType::FOR,
                    "fun" => TokenType::FUN,
                    "if" => TokenType::IF,
                    "nil" => TokenType::NIL,
                    "or" => TokenType::OR,
                    "print" => TokenType::PRINT,
                    "return" => TokenType::RETURN,
                    "super" => TokenType::SUPER,
                    "this" => TokenType::THIS,
                    "true" => TokenType::TRUE,
                    "var" => TokenType::VAR,
                    "while" => TokenType::WHILE,
                    _ => TokenType::IDENTIFIER,
                };
                return Ok(Some(Token::new(
                    token_type,
                    lexeme.to_string(),
                    None,
                    self.line_number,
                )));
            }
            Some(_) => {
                let error_message = format!(
                    "Unexpected character '{}' on line {}",
                    self.current_char.unwrap(),
                    self.line_number
                );
                return Err(MyError::ParsingError(error_message));
            }
            None => return Ok(None), // End of input
        };

        let lexeme = self.current_char.unwrap().to_string(); // Convert character to string
        self.advance();
        Ok(Some(Token::new(
            token_type,
            lexeme,
            None,
            self.line_number,
        )))
    }
    
    
    fn forward_look(&self) -> Option<char> {
        dbg!(self.position + 1 <= self.input.len());
        if self.position + 1 <= self.input.len() {

         let x = self.input[self.position..].chars().next();
         dbg!(x,self.position);

         return x
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, MyError> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }
}
