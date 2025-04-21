//! Parse functions
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

pub(crate) mod data;

/// The class to feed characters to `parser()`.
/// It's able to look ahead one codepoint.
struct CharFeeder<'a, I: Iterator<Item = char>> {
    /// Wrapped iterator.
    it: &'a mut I,
    /// The codepoint buffer.
    c: [char; 2],
    /// The number of the valid codepoints in `c`.
    num_c: usize,
    /// The line number at the current position.
    line: usize,
    /// The column number at the current position.
    column: usize,
}

impl<'a, I: Iterator<Item = char>> CharFeeder<'a, I> {
    /// The number of the lookahead codepoints.
    const LOOK_AHEAD: usize = 1;

    /// Create a new CharFeeder.
    ///
    /// # Parameter
    /// - `it`: The char iterator to wrap.
    fn new(it: &'a mut I) -> Self {
        let mut s = CharFeeder {
            it,
            c: ['\0'; 2],
            num_c: 0,
            line: 1,
            column: 1,
        };
        for i in 0..=Self::LOOK_AHEAD {
            match s.it.next() {
                None => s.c[i] = '\0',
                Some(x) => {
                    s.c[i] = x;
                    s.num_c += 1
                }
            };
        }
        return s;
    }

    /// The codepoint at the current position.
    ///
    /// # Return
    /// The codepoint at the current position. '\0' if `is_end()` is true.
    fn c(self: &Self) -> char {
        self.c[0]
    }

    /// The codepoint at the next position.
    ///
    /// # Return
    /// The codepoint at the next position. '\0' if the current position is the last of the input.
    fn next_c(self: &Self) -> char {
        self.c[1]
    }

    /// Is the position at the end?
    fn is_end(self: &Self) -> bool {
        self.num_c == 0
    }

    /// The line number of the current position.
    fn line_number(self: &Self) -> usize {
        self.line
    }

    /// The column number of the current position.
    fn column_number(self: &Self) -> usize {
        self.column
    }

    /// Step forward.
    fn next(self: &mut Self) {
        if self.is_end() {
            return;
        }
        if self.c() == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.c[0] = self.c[1];
        match self.it.next() {
            None => {
                self.c[1] = '\0';
                self.num_c -= 1;
            }
            Some(x) => {
                self.c[1] = x;
            }
        }
    }
}

use self::data::ProductionRule;
use self::data::Syntax;
use self::data::Text;
use self::data::TextOptions;
use crate::Substitutor;
use std::cell::RefCell;
use std::rc::Rc;

/// String in `Err` is the human readable error message.
type ParseResult<T> = Result<T, String>;

/// Create the instance of `Err` for the parse error.
fn parse_error<T, I: Iterator<Item = char>>(it: &CharFeeder<I>, err_msg: &str) -> ParseResult<T> {
    Err(format!(
        "Line#{}, Column#{}: {}",
        it.line_number(),
        it.column_number(),
        err_msg
    ))
}

/// Parse a phrase syntax to create the instance of the `Syntax`.
///
/// # Parameter
/// - `p`: The iterator of the source text.
///
/// # Return
/// The human readable error message when `Err`.
///
/// # Eample
/// ```rust
/// let syntax = tphrase::parse(&mut r#"main = Hello, World!"#.chars()).unwrap();
/// let mut ph: tphrase::Generator = tphrase::Generator::new();
/// let _ = ph.add(syntax);
/// assert_eq!(ph.generate(), "Hello, World!");
/// ```
///
/// `Err` in the result holds some human readable error messages.
/// ```rust
/// let syntax_result: Result<tphrase::Syntax, _> = tphrase::parse(&mut r#"
///     main = "Hello, " {WORLD}
///     WORLD
///       = world
/// "#.chars());
/// let err_msgs = syntax_result.err().unwrap();
/// assert_eq!(err_msgs.len(), 3);
/// assert_eq!(err_msgs[0], "Line#2, Column#22: The end of the text or \"\\n\" is expected.");
/// assert_eq!(err_msgs[1], "Line#3, Column#10: \"=\" or \":=\" is expected.");
/// assert_eq!(err_msgs[2], "Line#4, Column#7: A nonterminal \"[A-Za-z0-9_.]+\" is expected.");
/// ```
pub fn parse<S: Substitutor, I: Iterator<Item = char>>(
    p: &mut I,
) -> Result<Syntax<S>, Vec<String>> {
    let mut syntax = Syntax::new();
    let mut err_msg = Vec::new();
    let mut it = CharFeeder::new(p);

    while !it.is_end() {
        if let Err(e) = parse_assignment(&mut it, &mut syntax) {
            err_msg.push(e);
            // Recovering from the error
            let mut cont_line = false;
            while !it.is_end() {
                let c = it.c();
                if c == '\n' {
                    if cont_line {
                        cont_line = false;
                    } else {
                        break;
                    }
                } else if c != ' ' && c != '\t' {
                    cont_line = c == '|' || c == '~' || c == '=';
                }
                it.next();
            }
        }
    }
    syntax.fix_local_nonterminal(&mut err_msg);
    if err_msg.is_empty() {
        return Ok(syntax);
    } else {
        return Err(err_msg);
    }
}

/// Parse a phrase syntax to create the instance of the `Syntax`.
///
/// # Parameter
/// - `s`: The source text.
///
/// # Return
/// The human readable error message when `Err`.
///
/// # Eample
/// ```rust
/// let syntax = tphrase::parse_str(r#"main = Hello, World!"#).unwrap();
/// let mut ph: tphrase::Generator = tphrase::Generator::new();
/// let _ = ph.add(syntax);
/// assert_eq!(ph.generate(), "Hello, World!");
/// ```
///
/// `Err` in the result holds some human readable error messages.
/// ```rust
/// let syntax_result: Result<tphrase::Syntax, _> = tphrase::parse_str(r#"
///     main = "Hello, " {WORLD}
///     WORLD
///       = world
/// "#);
/// let err_msgs = syntax_result.err().unwrap();
/// assert_eq!(err_msgs.len(), 3);
/// assert_eq!(err_msgs[0], "Line#2, Column#22: The end of the text or \"\\n\" is expected.");
/// assert_eq!(err_msgs[1], "Line#3, Column#10: \"=\" or \":=\" is expected.");
/// assert_eq!(err_msgs[2], "Line#4, Column#7: A nonterminal \"[A-Za-z0-9_.]+\" is expected.");
/// ```
pub fn parse_str<S: Substitutor>(s: &str) -> Result<Syntax<S>, Vec<String>> {
    parse(&mut s.chars())
}

/// Skip spaces and newlines.
///
/// # Parameter
/// - `it`: The character feeder.
/// - `en_nl`: Skip also the newlines if `en_nl` is true.
fn skip_space_nl_opt<I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
    en_nl: bool,
) -> ParseResult<()> {
    while !it.is_end() {
        let c = it.c();
        if c == '{' && it.next_c() == '*' {
            it.next();
            it.next();
            while !it.is_end() && it.c() != '}' {
                it.next();
            }
            if it.is_end() {
                return parse_error(it, "The end of the comment is expected.");
            }
        } else if !(c == ' ' || c == '\t' || (en_nl && c == '\n')) {
            break;
        }
        it.next();
    }
    return Ok(());
}

/// Skip spaces.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Related EBNF
/// ```EBNF
/// space_opt = [ { space } ] ;
/// space = " " | "\t" | ( "{*", [ { ? [^}] ? } ], "}" ) ;
/// ```
fn skip_space<I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<()> {
    skip_space_nl_opt(it, false)
}

/// Skip spaces and newlines.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Related EBNF
/// ```EBNF
/// space_nl_opt = [ { space | nl } ] ;
/// nl = "\n" ;
/// ```
fn skip_space_nl<I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<()> {
    skip_space_nl_opt(it, true)
}

/// Skip spaces and a newline.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Related EBNF
/// ```EBNF
/// space_one_nl_opt = space_opt, [ nl, space_opt ] ;
/// ```
fn skip_space_one_nl<I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<()> {
    skip_space(it)?;
    if it.c() == '\n' {
        it.next();
        skip_space(it)?;
    }
    return Ok(());
}

/// Can it be a part of a nonterminal?
///
/// # Parameter
/// - `c`: The character to be tested.
///
/// # Return
/// `true` if it can be a part of a nonterminal.
fn is_nonterminal_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}

/// Parse an assignment.
///
/// # Parameter
/// - `it`: The character feeder.
/// - `syntax`: The syntax into which the assignment is added.
///
/// # Related EBNF
/// ```EBNF
/// start = space_nl_opt, [ { assignment, space_nl_opt } ], $ ;
/// assignment = nonterminal, space_opt, [ weight, space_opt ], operator, space_one_nl_opt, production_rule, ( nl | $ ) ; (* One of spaces before weight is necessary because nonterminal consumes the numeric character and the period. *)
/// ```
fn parse_assignment<S: Substitutor, I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
    syntax: &mut Syntax<S>,
) -> ParseResult<()> {
    skip_space_nl(it)?;
    if it.is_end() {
        return Ok(());
    }
    let nonterminal = parse_nonterminal(it)?;
    skip_space(it)?;
    let weight = parse_weight(it)?;
    skip_space(it)?;
    let op_type = parse_operator(it)?;
    skip_space_one_nl(it)?;
    let mut rule = parse_production_rule(it, '\0')?;
    rule.set_weight(weight);
    if it.is_end() || it.c() == '\n' {
        if op_type == ':' {
            rule.equalize_chance(true);
        }
        if let Err(err_msg) = syntax.add_production_rule(&nonterminal, rule) {
            return parse_error(&it, &err_msg);
        }
    } else {
        return parse_error(&it, "The end of the text or \"\\n\" is expected.");
    }
    return Ok(());
}

/// Parse a nonterminal.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The nonterminal.
///
/// # Related EBNF
/// ```EBNF
/// nonterminal = { ? [A-Za-z0-9_.] ? } ;
/// ```
fn parse_nonterminal<I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<String> {
    let mut nonterminal = String::new();
    while !it.is_end() {
        let c = it.c();
        if is_nonterminal_char(c) {
            nonterminal.push(c);
            it.next();
        } else {
            break;
        }
    }
    if nonterminal.is_empty() {
        return parse_error(&it, "A nonterminal \"[A-Za-z0-9_.]+\" is expected.");
    }
    return Ok(nonterminal);
}

/// Is it a character of the decimal number?
///
/// # Parameter
/// - `c`: The character to be tested.
///
/// # Return
/// `true` if it's a decimal number.
fn is_decimal_number_char(c: char) -> bool {
    '0' <= c && c <= '9'
}

/// Parse a weight number.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The weight number if weight is specified, or None.
///
/// # Related EBNF
/// ```EBNF
/// weight = ( ( { ? [0-9] ? }, [ "." ] ) | ( ".", ? [0-9] ? ) ), [ { ? [0-9] ? } ] ;
/// ```
fn parse_weight<I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<Option<f64>> {
    let mut s = String::new();
    let mut c = it.c();
    if c == '.' {
        it.next();
        c = it.c();
        if is_decimal_number_char(c) {
            s.push('.');
            s.push(c);
            it.next();
            c = it.c();
        } else {
            return parse_error(&it, "A number is expected. (\".\" is not a number.)");
        }
    } else if is_decimal_number_char(c) {
        while {
            s.push(c);
            it.next();
            c = it.c();
            is_decimal_number_char(c)
        } {}
        if c == '.' {
            s.push(c);
            it.next();
            c = it.c();
        }
    } else {
        return Ok(None);
    }
    while is_decimal_number_char(c) {
        s.push(c);
        it.next();
        c = it.c();
    }
    return Ok(Some(s.parse().unwrap()));
}

/// Parse an operator.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// ":" if the operator is ":=", or "=".
///
/// # Related EBNF
/// ```EBNF
/// operator = "=" | ":=" ;
/// ```
fn parse_operator<I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<char> {
    let c = it.c();
    if c == '=' {
        it.next();
        return Ok('=');
    } else if c == ':' {
        it.next();
        if it.c() == '=' {
            it.next();
            return Ok(':');
        } else {
            return parse_error(it, "\"=\" is expected.");
        }
    } else {
        return parse_error(it, "\"=\" or \":=\" is expected.");
    }
}

/// Parse a production rule.
///
/// # Parameter
/// - `it`: The character feeder.
/// - `term_char`: The expected character after the production rule. If it's '\0', no special character is expected.
///
/// # Return
/// The production rule.
///
/// # Related EBNF
/// ```EBNF
/// production_rule = options, gsubs ;
/// ```
fn parse_production_rule<S: Substitutor, I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
    term_char: char,
) -> ParseResult<ProductionRule<S>> {
    let options = parse_options(it)?;
    let gsubs = parse_gsubs(it)?;
    let rule = ProductionRule::new(options, gsubs);
    if term_char != '\0' {
        skip_space_nl(it)?;
        if it.c() == term_char {
            it.next();
        } else {
            let mut s = "\"".to_string();
            s.push(term_char);
            s += "\" is expected.";
            return parse_error(it, &s);
        }
    }
    return Ok(rule);
}

/// Parse an options.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The options.
///
/// # Related EBNF
/// ```EBNF
/// options = text, space_opt, [ { "|", space_one_nl_opt, text, space_opt } ] ;
/// ```
fn parse_options<S: Substitutor, I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
) -> ParseResult<TextOptions<S>> {
    let mut options = TextOptions::new();
    options.add_text(parse_text(it)?);
    skip_space(it)?;
    while it.c() == '|' {
        it.next();
        skip_space_one_nl(it)?;
        options.add_text(parse_text(it)?);
        skip_space(it)?;
    }
    return Ok(options);
}

/// Parse a text.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The text.
///
/// # Related EBNF
/// ```EBNF
/// text = text_begin, [ text_body, [ text_postfix ] ] |
///        '"', [ { ? [^"{] ? | expansion } ], '"', space_opt, [ weight ] |
///        "'", [ { ? [^'{] ? | expansion } ], "'", space_opt, [ weight ] |
///        "`", [ { ? [^`{] ? | expansion } ], "`", space_opt, [ weight ] ;
/// text_begin = ? [^ \t\n"'`|~{}] ? | expansion ; (* "}" is the next to the text when it's in {= ...}. *)
/// expansion = "{", [ { ? [^}] ? } ], "}" ;
/// weight = ( ( { ? [0-9] ? }, [ "." ] ) | ( ".", ? [0-9] ? ) ), [ { ? [0-9] ? } ] ;
/// ```
fn parse_text<S: Substitutor, I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
) -> ParseResult<Text<S>> {
    return match it.c() {
        '\0' | ' ' | '\t' | '\n' | '|' | '~' | '}' => parse_error(it, "A text is expected."),
        '"' | '\'' | '`' => parse_quoted_text(it),
        _ => parse_non_quoted_text(it),
    };
}

/// Parse a quoted text.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The text.
///
/// # Related EBNF
/// ```EBNF
/// text = ...
///     '"', [ { ? [^"{] ? | expansion } ], '"', space_opt, [ number ] |
///     "'", [ { ? [^'{] ? | expansion } ], "'", space_opt, [ number ] |
///     "`", [ { ? [^`{] ? | expansion } ], "`", space_opt, [ number ] ;
/// ```
fn parse_quoted_text<S: Substitutor, I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
) -> ParseResult<Text<S>> {
    let mut text = Text::new();
    let mut s = String::new();
    let quote = it.c();
    it.next();
    while !it.is_end() && it.c() != quote {
        if it.c() == '{' {
            parse_expansion(it, &mut text, &mut s)?;
        } else {
            s.push(it.c());
            it.next();
        }
    }
    if it.is_end() {
        let mut msg = "The end of the".to_string();
        msg.push(quote);
        msg += "quoted text";
        msg.push(quote);
        msg += " is expected.";
        return parse_error(it, &msg);
    }
    if !s.is_empty() {
        text.add_string(s);
    }
    it.next();
    skip_space(it)?;
    text.set_weight(parse_weight(it)?);
    return Ok(text);
}

/// Parse a non quoted text.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The text.
///
/// # Related EBNF
/// ```EBNF
/// text = ...
///     text_begin, [ text_body, [ text_postfix ] ] |
///     ... ;
/// text_body = { ? [^\n|~{}] ? | expansion } ;
/// text_postfix = ? space_opt(?=($|[\n|~}])) ? ; (* text_postfix greedily matches with space_opt preceding the end of the text, newline, "|", "~", or "}", but it consumes only space_opt. *)
/// ```
fn parse_non_quoted_text<S: Substitutor, I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
) -> ParseResult<Text<S>> {
    // The caller ensures it.c() == text_begin or EOT.
    let mut text = Text::new();
    let mut s = String::new();
    let mut spaces = String::new(); // The candidate for "text_postfix" (trailing spaces)

    loop {
        let c = it.c();
        match c {
            '\0' | '\n' | '|' | '~' | '}' => {
                if !s.is_empty() {
                    text.add_string(s);
                }
                break;
            }
            ' ' | '\t' => {
                spaces.push(c);
                it.next();
            }
            '{' => {
                if it.next_c() == '*' {
                    // The comment block can match "text_postfix" rule, so don't clear "spaces" if it's a comment block.
                    it.next();
                    it.next();
                    while !it.is_end() && it.c() != '}' {
                        it.next();
                    }
                    if it.is_end() {
                        return parse_error(it, "The end of the comment is expected.");
                    }
                    it.next();
                } else {
                    s += &spaces;
                    spaces.clear();
                    parse_expansion(it, &mut text, &mut s)?;
                }
            }
            _ => {
                s += &spaces;
                s.push(c);
                spaces.clear();
                it.next();
            }
        };
    }
    return Ok(text);
}

/// Parse an expansion.
///
/// # Parameter
/// - `it`: The character feeder.
/// - `text`: The text into which the parts are added.
/// - `s`: The unsolved string.
///
/// # Note
/// Accomplish the definitive conversions here. (If the string enclosed by "{" and "}" may be a nonterminal, it's a non-definitive conversion.)
///
/// # Related EBNF
/// ```EBNF
/// expansion = "{", [ { ? [^}] ? } ], "}" ;
/// ```
fn parse_expansion<S: Substitutor, I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
    text: &mut Text<S>,
    s: &mut String,
) -> ParseResult<()> {
    it.next();
    let c = it.c();
    if it.next_c() == '}' {
        // "{(}" and "{)}"
        let r_opt = match c {
            '(' => Some('{'),
            ')' => Some('}'),
            _ => None,
        };
        if let Some(r) = r_opt {
            it.next();
            it.next();
            s.push(r);
            return Ok(());
        };
    }

    if c == '=' || (c == ':' && it.next_c() == '=') {
        // Anonymous production rule
        if c == ':' {
            it.next();
        }
        it.next();
        skip_space_nl(it)?;
        text.add_string(s.clone());
        s.clear();
        let mut rule = parse_production_rule(it, '}')?;
        if c == ':' {
            rule.equalize_chance(true);
        }
        text.add_anonymous_rule(Rc::new(RefCell::new(rule)));
        return Ok(());
    } else {
        let is_comment = c == '*';
        let mut is_nonterminal = c != '}' && !is_comment;
        let mut name = String::new();
        while !it.is_end() {
            let c2 = it.c();
            it.next();
            if c2 == '}' {
                if is_nonterminal {
                    if !s.is_empty() {
                        text.add_string(s.clone());
                        s.clear();
                    }
                    text.add_expansion(name);
                } else if !is_comment {
                    *s += &name;
                }
                return Ok(());
            } else {
                is_nonterminal = is_nonterminal && is_nonterminal_char(c2);
                if !is_comment {
                    name.push(c2);
                }
            }
        }
    }
    return parse_error(it, "The end of the brace expansion is expected.");
}

/// Parse a gsubs.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The gsubs.
///
/// # Related EBNF
/// ```EBNF
/// gsubs = [ { "~", space_one_nl_opt, sep, { pat }, sep2, [ { pat } ], sep2, [ gsub_limit ], space_opt } ] ; (* 'sep2' is the same character of 'sep'. *)
/// sep = ? [^ \t\n{] ? ; (* '{' may be the beginning of the comment block. *)
/// ```
fn parse_gsubs<S: Substitutor, I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<S> {
    let mut gsubs = S::new();
    while it.c() == '~' {
        it.next();
        skip_space_one_nl(it)?;
        let sep = it.c();
        if it.is_end() {
            return parse_error(it, "Unexpected EOT.");
        } else if sep == '{' {
            return parse_error(it, "\"{\" isn't allowable as a separator.");
        }
        it.next();

        let pattern = parse_pattern(it, sep, false)?;
        let repl = parse_pattern(it, sep, true)?;
        let limit = parse_gsub_limit(it)?;
        if let Err(msg) = gsubs.add(&pattern, repl, limit) {
            let mut err_msg = "Gsub error: ".to_string();
            err_msg += &msg;
            return parse_error(it, &err_msg);
        }
        skip_space(it)?;
    }
    return Ok(gsubs);
}

/// Parse a gsub limit.
///
/// # Parameter
/// - `it`: The character feeder.
///
/// # Return
/// The limit.
///
/// # Related EBNF
/// ```EBNF
/// gsub_limit = "g" | { ? [0-9]+ ? } ;
/// ```
fn parse_gsub_limit<I: Iterator<Item = char>>(it: &mut CharFeeder<I>) -> ParseResult<usize> {
    let mut c = it.c();
    if c == 'g' {
        it.next();
        return Ok(0); // 0 means no limit.
    } else {
        let mut s = String::new();
        while is_decimal_number_char(c) {
            s.push(c);
            it.next();
            c = it.c();
        }
        if s.is_empty() {
            return Ok(1);
        } else {
            let n_opt = s.parse::<usize>();
            if let Ok(n) = n_opt {
                return Ok(n);
            } else {
                return parse_error(it, "Error in gsub limit. (It may be too big number.)");
            }
        }
    }
}

/// Parse a pattern or a repl.
///
/// # Parameter
/// - `it`: The character feeder.
/// - `sep`: The separator character.
/// - `allow_empty`: Is the empty string allowed?
///
/// # Return
/// The pattern or the repl.
///
/// # Related EBNF
/// ```EBNF
/// pat = ? all characters ? - sep2 ; (* 'sep2' is the character precedes 'pat' in the parent 'gsubs'. *)
/// ```
fn parse_pattern<I: Iterator<Item = char>>(
    it: &mut CharFeeder<I>,
    sep: char,
    allow_empty: bool,
) -> ParseResult<String> {
    let mut pat = String::new();
    while !it.is_end() && it.c() != sep {
        pat.push(it.c());
        it.next();
    }
    if !allow_empty && pat.is_empty() {
        return parse_error(it, "A nonempty pattern is expected.");
    }
    if it.is_end() {
        return parse_error(it, "Unexpected EOT.");
    }
    it.next();
    return Ok(pat);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_feeder() {
        let v = "abcd";
        let mut chars = v.chars();
        let mut it = CharFeeder::new(&mut chars);
        assert_eq!(it.is_end(), false);
        assert_eq!(it.c(), 'a');
        assert_eq!(it.next_c(), 'b');
        assert_eq!(it.line_number(), 1);
        assert_eq!(it.column_number(), 1);
        it.next();
        assert_eq!(it.c(), 'b');
        assert_eq!(it.next_c(), 'c');
        assert_eq!(it.line_number(), 1);
        assert_eq!(it.column_number(), 2);
        it.next();
        assert_eq!(it.c(), 'c');
        assert_eq!(it.next_c(), 'd');
        assert_eq!(it.line_number(), 1);
        assert_eq!(it.column_number(), 3);
        it.next();
        assert_eq!(it.c(), 'd');
        assert_eq!(it.is_end(), false);
        assert_eq!(it.line_number(), 1);
        assert_eq!(it.column_number(), 4);
        it.next();
        assert_eq!(it.is_end(), true);
    }
}
