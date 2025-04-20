//! Test for parsing
//!
//! Copyright © 2025 OOTA, Masato
//!
//! This file is part of TPhrase-rs.
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//!
//! OR
//!
//! Licensed under the Apache License, Version 2.0 (the "License"); you may not use TPhrase-rs except in compliance with the License. You may obtain a copy of the License at
//!
//! http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

extern crate tphrase;
use tphrase::*;

mod utils;
use utils::*;

#[test]
fn test_parse_hello_world() {
    let mut ph: Generator = r#"main=Hello World."#.parse().unwrap();
    assert_eq!(ph.generate(), "Hello World.");
}

#[test]
fn test_parse_spaces_before_equal() {
    let mut ph: Generator = r#"main =Hello World."#.parse().unwrap();
    assert_eq!(ph.generate(), "Hello World.");
}

#[test]
fn test_parse_spaces_after_equal() {
    let mut ph: Generator = r#"main= Hello World."#.parse().unwrap();
    assert_eq!(ph.generate(), "Hello World.");
}

#[test]
fn test_parse_newline_after_equal() {
    let mut ph: Generator = r#"main=
Hello World."#
        .parse()
        .unwrap();
    assert_eq!(ph.generate(), "Hello World.");
}

#[test]
fn test_parse_assignment_equal_chance() {
    let mut ph: Generator = r#"main := Hello World."#.parse().unwrap();
    assert_eq!(ph.generate(), "Hello World.");
}

#[test]
fn test_parse_assignment_trailing_spaces() {
    let mut ph: Generator = r#"main = Hello World.    {* --}
        {* --- }
"#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "Hello World.");
}

#[test]
fn test_parse_assignment_after_spaces() {
    let mut ph: Generator = r#"
        {* --- }


         main = Hello World."#
        .parse()
        .unwrap();
    assert_eq!(ph.generate(), "Hello World.");
}

#[test]
fn test_parse_assignments_top_down() {
    let mut ph: Generator = r#"
        main = {sub}
        sub = A
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A");
}

#[test]
fn test_parse_assignments_bottom_up() {
    let mut ph: Generator = r#"
        sub = A
        main = {sub}
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A");
}

#[test]
fn test_parse_spaces() {
    let mut ph: Generator<ZeroNG> = r#"
        {* comment } main 	{* comment } =  	{* comment }
            {* comment } text1 	{* comment } | 	{* comment }
            {* comment } "text2" 	{* comment } ~  	{* comment }
            {* comment } /A/Z/ 	{* comment }
            {* comment } 
            {* comment } sub 	{* comment } :=  	{* comment }
            {* comment } 'text3' 	{* comment } | 	{* comment }
            {* comment } `text4` 	{* comment }
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "text1");
}

#[test]
fn test_parse_production_rule_simple() {
    let mut ph: Generator<ZeroNG> = r#"
        main = text1 | text2 | text3 ~ /pat1/repl1/ ~ /pat2/repl2/g
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "text1");
}

#[test]
fn test_parse_text_quoted() {
    let mut ph: Generator<ZeroNG> = r#"
        main = text1 | "text2" 2
        sub = 'text1' 2 | `text2`
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "text1");
    assert_eq!(ph.weight(), 3.0);
}

#[test]
fn test_parse_text_quoted_with_real_number_1() {
    let mut ph: Generator<ZeroNG> = r#"
        main = text1 | "text2" 2.1
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "text1");
    assert_eq!(ph.weight(), 3.1);
}

#[test]
fn test_parse_text_quoted_with_real_number_2() {
    let mut ph: Generator<ZeroNG> = r#"
        main = text1 | "text2" .32
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "text1");
    assert_eq!(ph.weight(), 1.32);
}

#[test]
fn test_parse_text_quoted_with_all_decimals() {
    let mut ph: Generator<ZeroNG> = r#"
        main = text1 | "text2" 12345678901.
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "text1");
    assert_eq!(ph.weight(), 12345678902.0);
}

#[test]
fn test_parse_text_quoted_with_number_decimal_only() {
    let ph: Result<Generator, _> = r#"
        main = text1 | "text2" .
    "#
    .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("A number is expected. (\".\" is not a number.)"));
}

#[test]
fn test_parse_text_non_quoted() {
    let mut ph: Generator<ZeroNG> = r#"
        main = 	text1 	|  
            te|xt2
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "text1");
    assert_eq!(ph.combination_number(), 3);
}

#[test]
fn test_parse_text_empty() {
    let mut ph: Generator<ZeroNG> = r#"
        main = 	'' | "" | `` | {} | '' | {*
        comment }"" |
            '{* comment }' |
    ``"#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "");
    assert_eq!(ph.combination_number(), 8);
}

#[test]
fn test_parse_expansion_prior_rule() {
    let mut ph: Generator = r#"
        main = "  {"
{'`|~ 	}  "
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "  \"\n{'`|~ \t  ");
}

#[test]
fn test_parse_expansion_nonterminal_1() {
    let mut ph: Generator = r#"
        main = "-+{AAA}+="
        AAA = ZZZ
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+ZZZ+=");
}

#[test]
fn test_parse_expansion_nonterminal_2() {
    let mut ph: Generator = r#"
        main = "-+{1}+="
        1 = ZZZ
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+ZZZ+=");
}

#[test]
fn test_parse_expansion_nonterminal_3() {
    let mut ph: Generator = r#"
        main = "-+{_}+="
        _ = ZZZ
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+ZZZ+=");
}

#[test]
fn test_parse_expansion_braces() {
    let mut ph: Generator = r#"
        main = "-+{(}+={)}|-"
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+{+=}|-");
}

#[test]
fn test_parse_expansion_comment() {
    let mut ph: Generator = r#"
        main = "-+{*comment}+="
        comment = ZZZ
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-++=");
}

#[test]
fn test_parse_expansion_anonymous_rule_1() {
    let mut ph: Generator<ZeroNG> = r#"
        main = "-+{= A | B | C }+="
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+A+=");
}

#[test]
fn test_parse_expansion_anonymous_rule_2() {
    let mut ph: Generator<ZeroNG> = r#"
        main = "-+{:=1|2|3~/1/9/~|2|8|}+="
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+9+=");
}

#[test]
fn test_parse_expansion_anonymous_rule_3() {
    let mut ph: Generator<ZeroNG> = r#"
        main = "-+{=
           A | B | C
        }+="
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+A+=");
}

#[test]
fn test_parse_expansion_unsolved() {
    let mut ph: Generator<ZeroNG> = r#"
        main = "-+{AAA}+="
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "-+AAA+=");
}

#[test]
fn test_parse_gsub_simple() {
    let mut ph: Generator<ZeroNG> = r#"
        main = 1 | 2 | 3~/A/C/g
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "1");
}

#[test]
fn test_parse_gsub_separator() {
    let mut ph: Generator<ZeroNG> = r#"
        main = 1 | 2 | 3~|A|C|g~/B/D/ ~ "C"""#
        .parse()
        .unwrap();
    assert_eq!(ph.generate(), "1");
}

#[test]
fn test_parse_gsub_with_character_except_g() {
    let ph: Result<Generator, _> = r#"
        main = 1 | 2 | 3 ~ ~A~B~h"#
        .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The end of the text or \"\\n\" is expected."));
}

#[test]
fn test_parse_gsub_with_real_number() {
    let ph: Result<Generator, _> = r#"
        main = 1 | 2 | 3 ~ ~A~B~1.1"#
        .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The end of the text or \"\\n\" is expected."));
}

#[test]
fn test_parse_gsub_with_integer_number() {
    let mut ph: Generator<ZeroNG> = r#"
        main = 1 | 2 | 3 ~
            /@//0 ~
            /A//1 ~
            /B//2 ~
            /C//3 ~
            /D//4 ~
            /E//5 ~
            /F//6 ~
            /G//7 ~
            /H//8 ~
            /I//9"#
        .parse()
        .unwrap();
    assert_eq!(ph.generate(), "1");
}

#[test]
fn test_parse_gsub_with_too_big_number() {
    let ph: Result<Generator, _> = r#"
        main = 1 | 2 | 3 ~
            /@//0 ~
            /A//1 ~
            /B//2 ~
            /C//3 ~
            /D//4 ~
            /E//5 ~
            /F//6 ~
            /G//7 ~
            /H//8 ~
            /I//99999999999999999999999999999"#
        .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("Error in gsub limit. (It may be too big number.)"));
}

#[test]
fn test_parse_gsub_unicode_separator() {
    let mut ph: Generator<ZeroNG> = r#"
        main = 1 | 2 | 3 ~ あAあBあ"#
        .parse()
        .unwrap();
    assert_eq!(ph.generate(), "1");
}

#[test]
fn test_parse_error_in_the_last_line() {
    let ph: Result<Generator, _> = r#"
        main = 1 | 2 | 3 ~ /A//+"#
        .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The end of the text or \"\\n\" is expected."));
}

#[test]
fn test_parse_recursive_expansion_error() {
    let ph: Result<Generator, _> = r#"
        main = {A}
        A = {B}
        B = {C}
        C = {B}
    "#
    .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("Recursive expansion of \"B\" is detected."));
}

#[test]
fn test_parse_no_local_nonterminal_error() {
    let ph: Result<Generator, _> = r#"
        main = {A}
        A = {_B}
        B = C
    "#
    .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The local nonterminal \"_B\" is not found."));
}

#[test]
fn test_parse_nonterminal_with_weight_1() {
    let mut ph: Generator<ZeroNG> = r#"
        main 10 = A | B | C
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A");
    assert_eq!(ph.weight(), 10.0);
}

#[test]
fn test_parse_nonterminal_with_weight_2() {
    let mut ph: Generator<ZeroNG> = r#"
        main 10.5= A | B | C
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A");
    assert_eq!(ph.weight(), 10.5);
}

#[test]
fn test_parse_nonterminal_characters() {
    let mut ph: Generator<ZeroNG> = r#"
        main = {0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.}
        0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_. = 9
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "9");
}

#[test]
fn test_parse_redefined_nonterminal_error() {
    let ph: Result<Generator, _> = r#"
        main = {A}
        A = 1 | 2 | 3
        A = 4 | 5 | 6
    "#
    .parse();
    assert!(ph.is_err());
    if let Err(err_msg) = ph {
        assert_eq!(err_msg.len(), 1);
        assert!(err_msg[0].contains("The nonterminal \"A\" is already defined."))
    }
}

#[test]
fn test_parse_unclosed_comment_1() {
    let ph: Result<Generator, _> = r#"
        {*
    "#
    .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The end of the comment is expected."));
}

#[test]
fn test_parse_unclosed_comment_2() {
    let ph: Result<Generator, _> = r#"
        main = A
        {*
    "#
    .parse();
    assert!(ph.is_err());
    let err_msg = ph.err().unwrap();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The end of the comment is expected."));
}
