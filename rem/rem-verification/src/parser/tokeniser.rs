use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
#[derive(Logos)]
pub enum Token {
    #[token("Definition")]
    Definition,

    // Identifiers like: main, foo, result, unit, massert, mutate_x, Good, etc.
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string(), priority = 3)]
    Ident(String),

    #[token("(", priority = 3)]
    LParen,
    #[token(")", priority = 3)]
    RParen,

    #[token(":=", priority = 3)]
    Walrus,

    #[token(":", priority = 3)]
    Colon,

    #[token(".", priority = 3)]
    Dot,

    #[token("->", priority = 3)]
    Arrow,

    #[token(",", priority = 3)]
    Comma,

    // Numbers (we don’t care about value, just keep text)
    #[regex(r"[0-9]+", |lex| lex.slice().to_string(), priority = 2)]
    Number(String),

    // Any other non-whitespace, non-alnum symbol sequence
    // e.g. "%i32", "s=", "<-", ";", etc.
    // Symbols but excluding punctuation we tokenize explicitly
    #[regex(r"[^\w\s\.\,\(\)\:]+", |lex| lex.slice().to_string(), priority = 1)]
    Symbol(String),

    // Whitespace & actual lexer errors
    #[regex(r"[ \t\r\n\f]+", logos::skip, priority = 4)]
    Error,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Definition => write!(f, "Definition"),
            Ident(s) => write!(f, "{}", s),
            Number(s) => write!(f, "{}", s),
            Symbol(s) => write!(f, "{}", s),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            Walrus => write!(f, ":="),
            Colon => write!(f, ":"),
            Dot => write!(f, "."),
            Arrow => write!(f, "->"),
            Comma => write!(f, ","),
            Error => Ok(()),
        }
    }
}
