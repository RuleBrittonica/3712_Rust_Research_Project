use crate::parser::types::{CoqArgument, CoqDefinition};
use crate::parser::tokeniser::Token;
use logos::Logos;
use nom::{Err, IResult};
use nom::error::ErrorKind;

pub fn parse_coq_file(src: &str) -> Vec<CoqDefinition> {
    let tokens: Vec<Token> = Token::lexer(src)
        .filter_map(|res| res.ok())
        .collect();

    let mut defs = Vec::new();
    let mut input: &[Token] = &tokens;

    while !input.is_empty() {
        match parse_definition(input) {
            Ok((rest, def)) => {
                defs.push(def);
                input = rest;
            }
            Err(_) => input = &input[1..],
        }
    }

    defs
}

pub fn parse_definition<'a>(tokens: &'a [Token]) -> IResult<&'a [Token], CoqDefinition> {
    use Token::*;

    let mut input = tokens;

    // 1. Definition <name>
    match input.split_first() {
        Some((Definition, rest)) => input = rest,
        _ => return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Tag))),
    }

    let name = match input.split_first() {
        Some((Ident(n), rest)) => {
            input = rest;
            n.clone()
        }
        _ => return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Tag))),
    };

    // 2. Parse arguments: (x : T)
    let mut args: Vec<CoqArgument> = Vec::new();

    loop {
        match input {
            [LParen, Ident(arg), Colon, rest @ ..] => {
                input = rest;

                let mut ty = String::new();
                // Collect until RParen
                loop {
                    match input {
                        [Ident(s), rest2 @ ..] |
                        [Number(s), rest2 @ ..] |
                        [Symbol(s), rest2 @ ..] => {
                            if !ty.is_empty() { ty.push(' '); }
                            ty.push_str(s);
                            input = rest2;
                        }
                        [RParen, rest2 @ ..] => {
                            input = rest2;
                            break;
                        }
                        [_other, rest2 @ ..] => input = rest2,
                        [] => break,
                    }
                }

                args.push(CoqArgument {
                    name: arg.clone(),
                    ty: ty.trim().to_string(),
                });
            }
            _ => break,
        }
    }

    // 3. Parse return type: : <stuff> :=
    match input.split_first() {
        Some((Colon, rest)) => input = rest,
        _ => return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Tag))),
    }

    let mut return_type = String::new();
    loop {
        match input.split_first() {
            Some((Walrus, rest)) => {
                input = rest;
                break; /* finished return type */
            }

            Some((Ident(s), rest)) |
            Some((Number(s), rest)) |
            Some((Symbol(s), rest)) => {
                if !return_type.is_empty() { return_type.push(' '); }
                return_type.push_str(s);
                input = rest;
            }

            Some((_tok, rest)) => input = rest,

            None => return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Eof))),
        }
    }

    // 4. Body = original tokens until "."
    let mut body_chunks = Vec::new();

    loop {
        match input.split_first() {
            Some((Dot, rest)) => {
                input = rest;
                break;
            }
            Some((tok, rest)) => {
                body_chunks.push(tok.to_string());
                input = rest;
            }
            None => return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Eof))),
        }
    }

    let body = body_chunks
        .join(" ")
        .replace("( ", "(")
        .replace(" )", ")")
        .trim()
        .to_string();

    Ok((input, CoqDefinition { name, args, return_type, body }))
}
