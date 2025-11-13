#[cfg(test)]
mod tests {
    use crate::parser::types::*;
    use crate::parser::tokeniser::Token;
    use crate::parser::impls::*;

    use logos::Logos;

    // Helper: run the lexer and drop any errors
    fn lex(src: &str) -> Vec<Token> {
        Token::lexer(src)
            .filter_map(|r| r.ok())
            .collect()
    }

    #[test]
    fn tokenize_simple_definition() {
        let src = "Definition main : result unit := massert 3.";
        let tokens = lex(src);
        // for (i, t) in tokens.iter().enumerate() {
        //     println!("Token[{i}]: {:?}", t);
        // }

        assert!(tokens.contains(&Token::Definition));
        assert!(tokens.contains(&Token::Ident("main".into())));
        assert!(tokens.contains(&Token::Colon));
        assert!(tokens.contains(&Token::Walrus));
        assert!(tokens.contains(&Token::Dot));
    }

    #[test]
    fn tokenize_arguments() {
        let src = "Definition foo (x : U32) (y : bool) : T := expr.";
        let tokens = lex(src);

        assert!(tokens.contains(&Token::LParen));
        assert!(tokens.contains(&Token::RParen));

        assert!(tokens.contains(&Token::Ident("x".into())));
        assert!(tokens.contains(&Token::Ident("y".into())));

        // U32 and bool are Idents now (correct)
        assert!(tokens.contains(&Token::Ident("U32".into())));
        assert!(tokens.contains(&Token::Ident("bool".into())));
    }

    #[test]
    fn tokenizes_multiline_bodies() {
        let src = r#"
            Definition foo : T :=
              something (a b c)
              more stuff
              end.
        "#;

        let tokens = lex(src);

        assert!(tokens.contains(&Token::Ident("foo".into())));
        assert!(tokens.contains(&Token::Dot));
    }

    #[test]
    fn parse_zero_argument_definition() {
        let src = "Definition main : result unit := massert (3).";
        let tokens = lex(src);
        // for (i, t) in tokens.iter().enumerate() {
        //     println!("Token[{i}]: {:?}", t);
        // }

        let (_, def) = parse_definition(&tokens).expect("Should parse");

        assert_eq!(def.name, "main");
        assert_eq!(def.args.len(), 0);
        assert_eq!(def.return_type, "result unit");
        assert_eq!(def.body, "massert (3)");
    }

    #[test]
    fn parse_function_with_arguments() {
        let src = "Definition foo (x : U32) (y : bool) : result unit := something.";
        let tokens = lex(src);

        let (_, def) = parse_definition(&tokens).expect("Should parse");

        assert_eq!(def.name, "foo");
        assert_eq!(def.args.len(), 2);

        assert_eq!(def.args[0].name, "x");
        assert_eq!(def.args[0].ty, "U32");

        assert_eq!(def.args[1].name, "y");
        assert_eq!(def.args[1].ty, "bool");

        assert_eq!(def.return_type, "result unit");
        assert_eq!(def.body, "something");
    }

    #[test]
    fn parse_multiline_definition() {
        let src = r#"
            Definition foo (n : nat)
                           (b : bool)
            : result unit :=
                do_something n b
                >>= more_stuff.
        "#;

        let tokens = lex(src);

        let (_, def) = parse_definition(&tokens).expect("Should parse multiline");

        assert_eq!(def.name, "foo");
        assert_eq!(def.args.len(), 2);
        assert_eq!(def.args[0].ty, "nat");
        assert_eq!(def.args[1].ty, "bool");

        assert_eq!(def.return_type, "result unit");
        assert_eq!(def.body, "do_something n b >>= more_stuff");
    }

    #[test]
    fn parse_multiple_definitions_in_file() {
        let src = r#"
            Definition a : unit := tt.
            Definition b (x : nat) : nat := x.
            Definition c (y : bool) (z : nat) : unit := tt.
        "#;

        let defs = parse_coq_file(src);

        assert_eq!(defs.len(), 3);

        assert_eq!(defs[0].name, "a");
        assert_eq!(defs[1].name, "b");
        assert_eq!(defs[2].name, "c");

        assert_eq!(defs[2].args.len(), 2);
    }

    #[test]
    fn parse_ignores_comments_and_other_text() {
        let src = r#"
            (** comment *)
            Require Import Foo.
            Import Bar.

            Definition main : unit := tt.
        "#;

        let defs = parse_coq_file(src);

        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].name, "main");
    }
}
