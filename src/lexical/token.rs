#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Math
    PLUS, MINUS, STAR,

    // Comparisons
    EQ, GE, LE, LT, GT, UNEQ,

    // Boolean algebra
    NOT,

    // Literals
    ID(String), STRING(String), NUMBER(f64),

    // Single charecter tokens
    ASSIGN, LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE,
    RIGHT_BRACE, COMMA, DOT, SLASH, SEMICOLON,

    // Keywords
    VAR, CLASS, IF, WHILE, FOR, ELSE, FN, NIL, PRINT,
    RETURN, SUPER, THIS, TRUE, FALSE, EOF, OR, AND, 

    // Error
    ERR(String)
}
