pub enum TokenType {
    // Single character tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS,  SEMICOLON, SLASH, STAR,

    // One or two character tokens
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, GREATER,
    GREATER_EQUAL, GREATER_OR_EQUAL, LESS, LESS_EQUAL,

    // Literals
    INDENTIFIER, STRING, NUMBER,

    // Keywords
    AND, CLASS, ELSE, FALSE, FN, FOR, NONE, OR, PRINT,
    RETURN, SUPER, THIS, TRUE, LET, WHILE,

    EOF,
}