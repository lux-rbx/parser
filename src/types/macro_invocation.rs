use luau_parser_derive::{Print, Range};
use lux_lexer::token::Token;

use super::Bracketed;

/// The arguments passed to a [`MacroInvocation`].
#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd, Eq, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MacroInvocationArguments(pub Vec<Token>);

/// A macro invocation.
///
/// ```lua
/// macro_name!(...tokens)
/// ```
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MacroInvocation {
    /// The name of the macro that'll be invoked
    pub macro_name: Token,

    /// The tokens passed to the macro.
    pub arguments: Bracketed<MacroInvocationArguments>,
}
