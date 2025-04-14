//! All `impl` blocks for [`MacroInvocation`]

use lux_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};

use crate::{
    force_parse_bracketed,
    types::{MacroInvocation, MacroInvocationArguments, Parse, ParseWithArgs},
};

impl Parse for MacroInvocationArguments {
    fn parse(mut token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let mut arguments = Vec::new();
        let mut state = lexer.save_state();

        while !matches!(
            token.token_type,
            TokenType::EndOfFile | TokenType::Symbol(Symbol::ClosingParenthesis)
        ) {
            arguments.push(token);

            state = lexer.save_state();
            token = lexer.next_token();
        }

        lexer.set_state(state);

        Some(Self(arguments))
    }
}

impl Parse for MacroInvocation {
    fn parse(macro_name: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(macro_name.token_type, TokenType::MacroIdentifier(_)) {
            return None;
        }

        let arguments = force_parse_bracketed!(
            lexer,
            errors,
            "Expected <expr>",
            (
                TokenType::Symbol(Symbol::OpeningParenthesis),
                TokenType::Symbol(Symbol::OpeningParenthesis)
            ),
            Symbol::ClosingParenthesis,
        );

        Some(Self {
            macro_name,
            arguments,
        })
    }
}
