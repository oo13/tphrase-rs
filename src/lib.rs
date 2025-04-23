//! # Introduction
//! TPhrase for Rust can generate various phrases with a syntax expressed by a text, which can be translated to generate a phrase in other languages. (The way to translate is such as gettext, but it's outside the scope of TPhrase for Rust.)
//!
//! # Example
//! ## A simple example
//! `gettext()` is a translating function.
//!
//! ```rust
//! # use std::error::Error;
//! # fn gettext(s: &str) -> &str { s }
//! # fn main() -> Result<(), tphrase::CompileError> {
//! let mut ph: tphrase::Generator = gettext(r#"
//!     main = {HELLO}, {WORLD}!
//!     HELLO = Hi | Greetings | Hello | Good morning
//!     WORLD = world | guys | folks
//! "#).parse()?;
//! let s = ph.generate();
//!
//! assert_eq!(ph.number_of_syntax(), 1);
//! assert_eq!(ph.combination_number(), 12);
//! assert_eq!(ph.weight(), 12.0);
//! // Assertion failure if it's translated.
//! assert!(s == "Hi, world!" ||
//!         s == "Hi, guys!" ||
//!         s == "Hi, folks!" ||
//!         s == "Greetings, world!" ||
//!         s == "Greetings, guys!" ||
//!         s == "Greetings, folks!" ||
//!         s == "Hello, world!" ||
//!         s == "Hello, guys!" ||
//!         s == "Hello, folks!" ||
//!         s == "Good morning, world!" ||
//!         s == "Good morning, guys!" ||
//!         s == "Good morning, folks!");
//! # Ok(())
//! # }
//! ```
//!
//! ## Text Substitution, and Generation with an External Context
//!
//! ```rust
//! # use std::error::Error;
//! # fn gettext(s: &str) -> &str { s }
//! # fn main() -> Result<(), tphrase::CompileError> {
//! let mut ph: tphrase::Generator = gettext(r#"
//!     HELLO = Hi | Greetings | Hello | Good morning
//!     WORLD = world | "{GENDER}-siblings" 2 | folks ~
//!           /female-siblings/sisters/ ~
//!           /male-siblings/brothers/
//!     main = {HELLO}, {WORLD}!
//! "#).parse()?;
//! let s = ph.generate_with_context(&tphrase::ExtContext::from([
//!     ("GENDER".to_string(), "female".to_string()),
//! ]));
//!
//! assert_eq!(ph.number_of_syntax(), 1);
//! assert_eq!(ph.combination_number(), 12);
//! assert_eq!(ph.weight(), 16.0);
//! // Assertion failure if it's translated.
//! assert!(s == "Hi, world!" ||
//!         s == "Hi, sisters!" ||
//!         s == "Hi, folks!" ||
//!         s == "Greetings, world!" ||
//!         s == "Greetings, sisters!" ||
//!         s == "Greetings, folks!" ||
//!         s == "Hello, world!" ||
//!         s == "Hello, sisters!" ||
//!         s == "Hello, folks!" ||
//!         s == "Good morning, world!" ||
//!         s == "Good morning, sisters!" ||
//!         s == "Good morning, folks!");
//! # Ok(())
//! # }
//! ```
//!
//! "{GENDER}-siblings" is followed by 2 so the weight of "{GENDER}-siblings" is 2. The quotation is necessary if it's followed by a weight.
//!
//! If you will make it translatable, the external contexts should be the range in the predefined variations and use in order to restrict the context, instead of to introduce extensibility, that is, you should tell the translator the possible combinations before translating.
//!
//! ## Multiple Phrase Syntaxes
//!
//! ```rust
//! # use std::error::Error;
//! # fn gettext(s: &str) -> &str { s }
//! # fn main() -> Result<(), tphrase::CompileError> {
//! let syntax1: tphrase::Syntax = gettext(r#"
//!     main = I hope this modules is useful!
//! "#).parse()?;
//! let syntax2: tphrase::Syntax = gettext(r#"
//!     main = This module is a libre software. You can help out by contributing {BUGS}.
//!     BUGS= bug reports | typo fixes | "revisions of the documents" 2
//! "#).parse()?;
//!
//! let mut ph: tphrase::Generator = tphrase::Generator::new();
//! let _ = ph.add(syntax1)?;
//! let _ = ph.add(syntax2)?;
//! let s = ph.generate();
//!
//! assert_eq!(ph.number_of_syntax(), 2);
//! assert_eq!(ph.combination_number(), 4);
//! assert_eq!(ph.weight(), 5.0);
//! // Assertion failure if it's translated.
//! assert!(s == "I hope this modules is useful!" ||
//!         s == "This module is a libre software. You can help out by contributing bug reports." ||
//!         s == "This module is a libre software. You can help out by contributing typo fixes." ||
//!         s == "This module is a libre software. You can help out by contributing revisions of the documents.");
//! # Ok(())
//! # }
//! ```
//! A phrase generator can have completely independent phrase syntaxes.
//!
//! The chance to generate "I hope..." is 20%, "This module..." is 80% because latter has 4 weights. It equalizes the chance to select each phrase syntax if [`Generator::equalize_chance()`] is called with `true`.
//!
//! ## Separate Syntaxes to Parse
//!
//! ```rust
//! # use std::error::Error;
//! # fn gettext(s: &str) -> &str { s }
//! # fn main() -> Result<(), tphrase::CompileError> {
//! let greet_syntax: tphrase::Syntax = gettext(r#"
//!     HELLO = Hi | Greetings | Hello | Good morning
//! "#).parse()?;
//! let world_syntax: tphrase::Syntax = gettext(r#"
//!     WORLD = world | guys | folks
//! "#).parse()?;
//! let mut main_syntax: tphrase::Syntax = gettext(r#"
//!     main = {HELLO}, {WORLD}!
//! "#).parse()?;
//!
//! main_syntax.add(greet_syntax)?;
//! main_syntax.add(world_syntax)?;
//!
//! let mut ph: tphrase::Generator = tphrase::Generator::new();
//! let _ = ph.add(main_syntax)?;
//! let s = ph.generate();
//!
//! assert_eq!(ph.number_of_syntax(), 1);
//! assert_eq!(ph.combination_number(), 12);
//! assert_eq!(ph.weight(), 12.0);
//! // Assertion failure if it's translated.
//! assert!(s == "Hi, world!" ||
//!         s == "Hi, guys!" ||
//!         s == "Hi, folks!" ||
//!         s == "Greetings, world!" ||
//!         s == "Greetings, guys!" ||
//!         s == "Greetings, folks!" ||
//!         s == "Hello, world!" ||
//!         s == "Hello, guys!" ||
//!         s == "Hello, folks!" ||
//!         s == "Good morning, world!" ||
//!         s == "Good morning, guys!" ||
//!         s == "Good morning, folks!");
//! # Ok(())
//! # }
//! ```
//!
//! Parsing separating syntaxes may make easy to understand the parse error message and edit the translation file. If the syntax has more than a few decades lines, you should consider dividing into the multiple syntaxes (In above example, it's too short).
//!
//! It can use to create a common library with some assignments, but the number of the combinations should be low enough for the translators to accept them.
//!
//! # Syntax of the Phrase Syntax
//! ## Overview
//! The phrase syntax consists of assignments. The order of the assignments doesn't affect the generated text. The recursive reference is not allowed. The multiple definition for a nonterminal occurs an error.
//!
//! It needs a definition of the nonterminal where is the start condition to generate the phrase. It's "main" by default, and Rust coders can change it.
//!
//! ## Spaces
//! The spaces can consist of U+0020 SPACE, U+0009 TAB, and the comment blocks "{* ... }".
//!
//! The operators "=", ":=", "|", and "~" can be preceded by the spaces, and succeeded by spaces and one newline. (If it allowed multiple newlines, some syntax errors cause puzzling error messages.)
//!
//! ## Assignment
//! The assignment defines a nonterminal assigned to a production rule.
//!
//! `nonterminal = production_rule` or `nonterminal := production_rule`
//!
//!  The nonterminal can consist the alphabet, numeric, period, and low line characters ("[A-Za-z0-9_.]"). The nonterminal starts with "_" is a local nonterminal that is visible only from the same compile unit.
//!
//! The assignments must be separated by one newline at least. The last assignment doesn't need a following newline. There can be any number of the spaces and newlines between the assignments.
//!
//! The assignment operator ":=" means the production rule equalizes the chance to select each text. "=" means the chance depends on the number of the possible texts to generate and the weight set by user.
//!
//! If you don't use ":=" and the weight, the chance to generate all possible phrases is equal:
//! ```tphrase
//! main = {A1} | {A2}
//! A1 = 0 | 1 | 2
//! A2 = {A21} | {A22}
//! A21 = 3 | 4
//! A22 = 5 | 6 | 7 | 8 | 9
//! ```
//! The each chance to generate from "0" to "9" is 10%. The chance to select {A1} is 30%, {A2} is 70%, {A21} is 20%, {A22} is 50%.
//!
//! If you use ":=":
//! ```tphrase
//! main = {A1} | {A2}
//! A1 = 0 | 1 | 2
//! A2 := {A21} | {A22}
//! A21 = 3 | 4
//! A22 = 5 | 6 | 7 | 8 | 9
//! ```
//! The chance to select {A21} is 35%, {A22} is 35%. {A1} and {A2} aren't affected by ":=". (cf. The weight propagates the higher layers.)
//!
//! A weight of a nonterminal can be specified between the nonterminal and the assignment operator. For example:
//! ```tphrase
//! SUB 1 = A | B
//! ```
//! It's equivalent to this:
//! ```tphrase
//! SUB = "A" 0.5 | "B" 0.5
//! ```
//! The weight of the nonterminal is the sum of the weight of the options in the production rule by default.
//!
//! ## Production rule
//!
//! The production rule consist of options and gsubs. For example: `text1 | text2 | text3 ~ /pat1/repl1/ ~ /pat2/repl2/g`
//!
//! ## Options
//! The options are texts separated "|". For example: `text1 | text2 | text3`
//!
//! ## Text
//! The text is the candidate for the result of the production rule.
//!
//! If a text enclose quotation ('"', "'", or "`"), the text can have any character except the quotation. If it's followed by a number, the number is the weight of the chance to select the text. There can be the spaces between the quoted text and the weight number. By default, the weight of the text is the product of the weight of the expansions in the text. (The weight of the string except the expansion is one.)
//!
//! ```tphrase
//! A = text1 | "text2" 2
//! ```
//! The chance of "text1" is 33%, "text2" is 67%.
//! ```tphrase
//! A = text1 | {B}
//! B = text2 | "{C}" 2
//! C = 1 | 2 | 3
//! ```
//! The chance of "text1" is 25%, "text2" is 25%, "{C}" is 50%. The chance of "{C}" is lower than no weight.
//!
//! The text doesn't need to enclose quotations ('"', "'", or "`") if it meets these requirements:
//!    1. The text is not empty.
//!    1. The text has neither newline, "|", "~", nor "}" except for a part of an expansion.
//!    1. The beginning of the text is other than the spaces and quotations. (The spaces, including the comment block, preceding the text are not a part of the text. The expansion is a part of the text even if the expansion results the spaces or the empty string.)
//!    1. The end of the text is not the spaces. (The spaces, including the comment block, succeeding the text are not a part of the text. The expansion is a part of the text even if the expansion results the spaces or the empty string.)
//!    1. The text is not followed by a weight number. (The number is a part of the text.)
//!
//! The text may have expansions, which is a string enclosed by "{" and "}". The text can contain "{" only as the beginning of the expansion, and the expansion can include any character except "}". The rule is prior to the above rules, for example &quot; {&quot;} &quot; is a valid text.
//!
//! ## Expansion
//! The string enclosed by "{" and "}" is the expansion, which will be expanded into a text. "{" and "}" can enclose any character except "}". If the string enclosed "{" and "}" has only alphabet, numeric, period, and low line characters ("[A-Za-z0-9_.]"), the enclosed string is a nonterminal. The nonterminal starts with "_" is a local nonterminal.
//!
//! 1. If the nonterminal is assigned to a production rule, the expansion will be expanded in the generated text.
//! 1. The local unsolved nonterminal occurs an error.
//! 1. If the external context specifies the substitution for the global unsolved nonterminal, it's applied.
//! 1. "{(}" and "{)}" will be expanded into "{" and "}".
//! 1. If the beginning of the expansion is "{*", the expansion will be expanded into the empty string. (It's effectively a comment block.)
//! 1. If the beginning of the expansion is "{=" or "{:=", the content (except the first "=" or ":=") is considered as a production rule. For example, "{= A|B|C}" will be expanded into the result of the production rule "A|B|C". The syntax of the content is expressed by EBNF: `content = space_nl_opt, production_rule, space_nl_opt ;` "{:=" is, of course, the equalized select version of "{=".
//! 1. The other expansion will be expanded into itself removed outer "{" and "}". (I recommend that the nonterminal is noticeable to find it easily unless you will leave it unsolved.)
//!
//! ## Gsub (Global substitution)
//! Gsub is the function to substitute the resulting string selected from the options. You can specify any number (including zero) of gsubs that substitute the string. 1st gsub specifies the substitution of the selected text out of the options, and then the result of the preceding substitution is substituted by the next gsub's.
//!
//! A gsub specification follows "~", the first character except spaces means the separator in the specification. The separator character can be any codepoint except spaces, newline, and "{", and can differ from a separator in the other specification.
//!
//! For example:
//! ```tphrase
//! ~ /A B/C D/g ~ !/!|!11 ~ $X Y$1 2$
//! ```
//!
//! The pattern parameter succeeds the first separator in the specification and can have any characters except the separator, but the pattern must not be the empty string.
//!
//! The separator succeeds the pattern, and the replacement parameter follows the separator. The replacement can have any characters except the separator, and can be the empty string.
//!
//! The separator succeeds the replacement, and the number parameter follows the separator. The number is the integer or "g" that means all ("0" is equivalent to "g"). You can omit the number and it means "1".
//!
//! The parameters "pattern", "replacement", "gsub_limit" are compatible with the regex in the regex::Regex by default. The Rust coders can customize the gsub function.
//!
//! ## EBNF
//!
//! ```EBNF
//! start = space_nl_opt, [ { assignment, space_nl_opt } ], $ ;
//! space = " " | "\t" | ( "{*", [ { ? [^}] ? } ], "}" ) ;
//! nl = "\n" ;
//! space_nl_opt = [ { space | nl } ] ;
//!
//! assignment = nonterminal, space_opt, [ weight, space_opt ], operator, space_one_nl_opt, production_rule, ( nl | $ ) ; (* One of spaces before weight is necessary because nonterminal consumes the numeric character and the period. *)
//! nonterminal = { ? [A-Za-z0-9_.] ? } ;
//! weight = ( ( { ? [0-9] ? }, [ "." ] ) | ( ".", ? [0-9] ? ) ), [ { ? [0-9] ? } ] ;
//! operator = "=" | ":=" ;
//! space_opt = [ { space } ] ;
//! space_one_nl_opt = space_opt, [ nl, space_opt ] ;
//!
//! production_rule = options, gsubs ;
//!
//! options = text, space_opt, [ { "|", space_one_nl_opt, text, space_opt } ] ;
//! text = text_begin, [ text_body, [ text_postfix ] ] |
//!        '"', [ { ? [^"{] ? | expansion } ], '"', space_opt, [ weight ] |
//!        "'", [ { ? [^'{] ? | expansion } ], "'", space_opt, [ weight ] |
//!        "`", [ { ? [^`{] ? | expansion } ], "`", space_opt, [ weight ] ;
//! text_begin = ? [^ \t\n"'`|~{}] ? | expansion ; (* "}" is the next to the text when it's in {= ...}. *)
//! text_body = { ? [^\n|~{}] ? | expansion } ;
//! text_postfix = ? space_opt(?=($|[\n|~}])) ? ; (* text_postfix greedily matches with space_opt preceding the end of the text, newline, "|", "~", or "}", but it consumes only space_opt. *)
//! expansion = "{", [ { ? [^}] ? } ], "}" ;
//!
//! gsubs = [ { "~", space_one_nl_opt, sep, { pat }, sep2, [ { pat } ], sep2, [ gsub_limit ], space_opt } ] ; (* 'sep2' is the same character of 'sep'. *)
//! sep = ? [^ \t\n{] ? ; (* '{' may be the beginning of the comment block. *)
//! pat = ? all characters ? - sep2 ; (* 'sep2' is the character precedes 'pat' in the parent 'gsubs'. *)
//! gsub_limit = "g" | { ? [0-9] ? } ;
//! ```
//!
//! # License
//! TPhrase for Rust is licensed under either of [MIT](http://opensource.org/licenses/MIT) or [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) at your option.
//!
//! Copyright © 2025 OOTA, Masato
//!
//! This file is part of TPhrase for Rust.
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//!
//! OR
//!
//! Licensed under the Apache License, Version 2.0 (the "License"); you may not use TPhrase for Rust except in compliance with the License. You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

mod compile_error;
mod fastrand_rng;
mod generator;
mod parser;
mod random_number_generator;
mod regex_substitutor;
mod substitutor;
mod utils;

pub use compile_error::CompileError;
pub use fastrand_rng::FastrandRng;
pub use generator::Generator;
pub use generator::SyntaxId;
pub use generator::SyntaxRemoveError;
pub use parser::data::Syntax;
pub use parser::parse;
pub use parser::parse_str;
pub use random_number_generator::RandomNumberGenerator;
pub use regex_substitutor::RegexGsub;
pub use substitutor::Substitutor;
pub use substitutor::SubstitutorAddError;
pub(crate) use utils::{select_and_generate_text, TextGenerator};
pub use utils::{trunc_syntax, trunc_syntax_str};

/// The default random number generator of [`Generator`].
pub type DefaultRng = FastrandRng;
/// The default substitutor of [`Generator`].
pub type DefaultSubst = RegexGsub;
/// The type of the external context.
pub type ExtContext = std::collections::HashMap<String, String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ph: Generator = "
            main = {HELLO}, {WORLD} ~
                  /Hell([, ])/Hello$1/
            HELLO = Hell
            WORLD = World!"
            .parse()
            .unwrap();
        assert_eq!(ph.generate(), "Hello, World!");
    }
}

#[doc = include_str!("../Readme.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
