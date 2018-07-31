use lexical::token::TokenType;
use lexical::token::TokenType::*;
use std::collections::VecDeque;

pub struct Scanner {
    input: VecDeque<char>,
    scanner_sym: char,
    line_number: u64
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        let line_number = 0;
        Scanner {
            // reader: reader,
            input: input.chars().collect(),
            scanner_sym: '\0',
            line_number: line_number
        }
    }
    
    fn next(&mut self) {
        let reader_sym = self.input.pop_front();

        match reader_sym {
            Some(sym) => self.scanner_sym = sym,
            None => self.scanner_sym = '\0'
        }
    }

    fn has_next(&self) -> bool {
        self.input.len() > 0
    }

    fn match_symbol(&mut self, token: char) -> bool {
        // LL(1) parser, because we lookahead at one character
        self.next();

        self.scanner_sym == token
    }

    pub fn get_token(&mut self) -> Option<TokenType> {
        self.next_token()
    }

    fn next_token (&mut self) -> Option<TokenType> {
        self.next();

        match self.scanner_sym {
            '(' => Some(LEFT_PAREN),
            ')' => Some(RIGHT_PAREN),
            '{' => Some(LEFT_BRACE),
            '}' => Some(RIGHT_BRACE),
            ',' => Some(COMMA),
            '.' => Some(DOT),
            ';' => Some(SEMICOLON),

            // Math (without divide, because it can also be a commnet :o)
            '-' => Some(MINUS),
            '+' => Some(PLUS),
            '*' => Some(STAR),

            // Special cases:
            '/' => {
                // Comment, so till end of line
                if self.match_symbol('/') {
                    while self.scanner_sym != '\n' {
                        self.next();
                    }
                    self.next_token()
                }
                else {
                    // Not a comment, so single slash
                    Some(SLASH)
                }
            },

            // Operators
            '!' => { 
                if self.match_symbol('='){
                    Some(UNEQ)
                }else{
                    Some(NOT)
                }
            },
            '=' =>  { 
                if self.match_symbol('='){
                    Some(EQ)
                }else{
                    Some(ASSIGN)
                }
            },
            '<' =>  { 
                if self.match_symbol('='){
                    Some(LE)
                }else{
                    Some(LT)
                }
            },
            '>' =>   { 
                if self.match_symbol('='){
                    Some(GE)
                }else{
                    Some(GT)
                }
            },
            
            // identifiers
            identifier if identifier.is_alphabetic() => {
                self.identifier()
            },

            // Numbers
            '0'...'9' => {
                self.number()
            },

            // Strings or identifiers
            '"' => {
                self.string()
            },

            '\0'=> None,

            // Character is not defined or not interesting.
            other => {
                match other {
                    // Ignore all whitespace
                    '\n' => {
                        //No new token found, so call again.
                        self.line_number += 1;
                        self.next_token()
                    },
                    ' ' => self.next_token(),
                    '\r' => self.next_token(),
                    '\t' => self.next_token(),
                    random_char => {
                        error!("Unrecognized character  {}", random_char);
                        self.next_token()
                    }
                }
            }
        }
    }

    fn string(&mut self) -> Option<TokenType> {
        let mut literal = String::new();
        self.next();

        while self.scanner_sym != '"' && self.has_next() {
            literal.push(self.scanner_sym);
            self.next();
        }

        if self.scanner_sym != '"'{
            error!("Missing closing \" for string");
            self.next_token()
        }
        else {
            Some(STRING(literal))
        }        
    }

    fn number(&mut self) -> Option<TokenType>{
        let mut value_string = self.scanner_sym.to_string();
        self.next();

        while self.scanner_sym.is_numeric() && self.has_next() {
            value_string.push(self.scanner_sym);
            self.next();
        }

        if self.scanner_sym == '.' {
            value_string.push(self.scanner_sym);
            self.next();
            while self.scanner_sym.is_numeric() {
                value_string.push(self.scanner_sym);
                
                if !self.has_next() {
                    break;
                }
                else {
                    self.next();
                }
            }
        }
        let value: f64 = value_string.parse().unwrap();
        Some(NUMBER(value))
    }

    fn identifier(&mut self) -> Option<TokenType> {
        let mut literal : String = self.scanner_sym.to_string();
        self.next();

        while self.scanner_sym.is_alphabetic() || self.scanner_sym.is_numeric() {
            literal.push(self.scanner_sym);
            self.next();
        }

        let token: Option<TokenType> = self.keywords(&literal);

        match token {
            Some(val) => Some(val),
            None => Some(ID(literal))
        }
    }

    fn keywords (&mut self, token: &str) -> Option<TokenType> {
        match token {
            "var" => Some(VAR),
            "class" => Some(CLASS),
            "if" => Some(IF),
            "while" => Some(WHILE),
            "for" => Some(FOR),
            "else" => Some(ELSE),
            "fn" => Some(FN),
            "nil" => Some(NIL),
            "print" => Some(PRINT),
            "return" => Some(RETURN),
            "super" => Some(SUPER),
            "this" => Some(THIS),
            "true" => Some(TRUE),
            "false" => Some(FALSE),
            "or" => Some(OR),
            "and" => Some(AND),
            _ => None
        }
    }
}

impl Iterator for Scanner {
    type Item = TokenType;

    fn next(&mut self) -> Option<TokenType> {
        self.get_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_initialization() {
        let mut scanner = Scanner::new("print");

        scanner.next();
        assert_eq!(scanner.scanner_sym, 'p');

        scanner.next();
        assert_eq!(scanner.scanner_sym, 'r');

        scanner.next();
        assert_eq!(scanner.scanner_sym, 'i');
    }

    #[test]
    fn test_scanner_hello_world() {
        // Given: A reader and a input program hello world
        let mut scanner = Scanner::new("print \"Hello World!\";");


        // When we get the tokens, then the order should be PRINT, STRING("Hello World!"), and SEMICOLON
        assert_eq!(scanner.get_token().unwrap(), TokenType::PRINT);
        assert_eq!(scanner.get_token().unwrap(), TokenType::STRING(String::from("Hello World!")));
        assert_eq!(scanner.get_token().unwrap(), TokenType::SEMICOLON);
    }

    #[test]
    fn test_string() {
        // given A number as a string
        let mut scanner = Scanner::new("\"Hello World\"");

        // when: get the number as a token:
        let token: TokenType = scanner.get_token().unwrap();

        assert_eq!(token, TokenType::STRING(String::from("Hello World")));
    }

    #[test]
    fn test_number() {
        // given A number as a string
        println!("statt...");
        let mut scanner = Scanner::new("12345.01");

        println!("yay?");
        // when: get the number as a token:
        let token: TokenType = scanner.get_token().unwrap();

        assert_eq!(token, TokenType::NUMBER(12345.01));
    }
}