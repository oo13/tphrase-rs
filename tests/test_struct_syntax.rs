//! Test for struct Syntax
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

extern crate tphrase;
use tphrase::*;

#[test]
fn test_struct_syntax_new() {
    let syntax: Syntax = Syntax::new();

    // Check the error.
    let mut ph: Generator = Generator::new();
    let result = ph.add(syntax);
    assert!(result.is_err());
    let msgs = result.unwrap_err();
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0], "The nonterminal \"main\" doesn't exist.");
    assert_eq!(ph.generate(), "nil");
    assert_eq!(ph.combination_number(), 0);
    assert_eq!(ph.weight(), 0.0);
    assert_eq!(ph.number_of_syntax(), 0);
}

#[test]
fn test_struct_syntax_from_str() {
    let syntax: Result<Syntax, _> = std::str::FromStr::from_str(
        r#"
        main = Hello, World!
    "#,
    );

    // Check no errors.
    let mut ph: Generator = Generator::new();
    let _ = ph.add(syntax.unwrap()).unwrap();
    assert_eq!(ph.generate(), "Hello, World!");
}

#[test]
fn test_struct_syntax_str_parse() {
    let syntax: Result<Syntax, _> = r#"
        main = Hello, World!
    "#
    .parse();

    // Check no errors.
    let mut ph: Generator = Generator::new();
    let _ = ph.add(syntax.unwrap()).unwrap();
    assert_eq!(ph.generate(), "Hello, World!");
}

#[test]
fn test_struct_syntax_parse() {
    let syntax: Result<Syntax, _> = parse(
        &mut r#"
        main = Hello, World!
    "#
        .chars(),
    );

    // Check no errors.
    let mut ph: Generator = Generator::new();
    let _ = ph.add(syntax.unwrap()).unwrap();
    assert_eq!(ph.generate(), "Hello, World!");
}

#[test]
fn test_struct_syntax_parse_str() {
    let syntax: Result<Syntax, _> = parse_str(
        r#"
        main = Hello, World!
    "#,
    );

    // Check no errors.
    let mut ph: Generator = Generator::new();
    let _ = ph.add(syntax.unwrap()).unwrap();
    assert_eq!(ph.generate(), "Hello, World!");
}

#[test]
fn test_struct_syntax_clone() {
    let syntax: Syntax = r#"
        main = {HELLO}, {= World!}
        HELLO = Hello
    "#
    .parse()
    .unwrap();
    let syntax2 = syntax.clone();

    // Check no errors.
    let mut ph: Generator = Generator::new();
    let _ = ph.add(syntax2).unwrap();
    assert_eq!(ph.generate(), "Hello, World!");
}

#[test]
fn test_struct_syntax_add() {
    let mut syntax1: Syntax = r#"
        main = {HELLO}, {= World!}
    "#
    .parse()
    .unwrap();
    let syntax2: Syntax = r#"
        HELLO = Hello
    "#
    .parse()
    .unwrap();
    syntax1.add(syntax2).unwrap();

    // Check no errors.
    let mut ph: Generator = Generator::new();
    let _ = ph.add(syntax1).unwrap();
    assert_eq!(ph.generate(), "Hello, World!");
}
