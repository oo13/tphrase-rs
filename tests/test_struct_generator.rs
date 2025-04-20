//! Test for struct Generator
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
fn test_struct_generator_from_str() {
    let mut ph: Generator = std::str::FromStr::from_str(
        r#"
        main = Hello, World!
    "#,
    )
    .unwrap();

    assert_eq!(ph.generate(), "Hello, World!");
    assert_eq!(ph.combination_number(), 1);
    assert_eq!(ph.weight(), 1.0);
    assert_eq!(ph.number_of_syntax(), 1);
}

#[test]
fn test_struct_generator_str_parse() {
    let mut ph: Generator = r#"
        main = Hello, World!
    "#
    .parse()
    .unwrap();

    assert_eq!(ph.generate(), "Hello, World!");
    assert_eq!(ph.combination_number(), 1);
    assert_eq!(ph.weight(), 1.0);
    assert_eq!(ph.number_of_syntax(), 1);
}

#[test]
fn test_struct_generator_clone_and_equalize_chance() {
    let mut ph1: Generator = r#"
        main = {= X | Y | Z } | {A} | {B}

        A = A1 | A2 | A3 ~ /a/b/

        B = B1 | B2 | B3
    "#
    .parse()
    .unwrap();
    let syntax: Syntax = r#"
        main = {= V | W } | {C} ~ /c/d/

        C = C1 | C2 | C3
    "#
    .parse()
    .unwrap();
    ph1.add(syntax).unwrap();

    let mut ph2: Generator = ph1.clone();

    ph1.equalize_chance(true);

    let dist1 = TextDistribution::from([
        ("X".to_string(), 1.0 / (2.0 * 9.0)),
        ("Y".to_string(), 1.0 / (2.0 * 9.0)),
        ("Z".to_string(), 1.0 / (2.0 * 9.0)),
        ("A1".to_string(), 1.0 / (2.0 * 9.0)),
        ("A2".to_string(), 1.0 / (2.0 * 9.0)),
        ("A3".to_string(), 1.0 / (2.0 * 9.0)),
        ("B1".to_string(), 1.0 / (2.0 * 9.0)),
        ("B2".to_string(), 1.0 / (2.0 * 9.0)),
        ("B3".to_string(), 1.0 / (2.0 * 9.0)),
        ("V".to_string(), 1.0 / (2.0 * 5.0)),
        ("W".to_string(), 1.0 / (2.0 * 5.0)),
        ("C1".to_string(), 1.0 / (2.0 * 5.0)),
        ("C2".to_string(), 1.0 / (2.0 * 5.0)),
        ("C3".to_string(), 1.0 / (2.0 * 5.0)),
    ]);
    let dist2 = TextDistribution::from([
        ("X".to_string(), 1.0 / 14.0),
        ("Y".to_string(), 1.0 / 14.0),
        ("Z".to_string(), 1.0 / 14.0),
        ("A1".to_string(), 1.0 / 14.0),
        ("A2".to_string(), 1.0 / 14.0),
        ("A3".to_string(), 1.0 / 14.0),
        ("B1".to_string(), 1.0 / 14.0),
        ("B2".to_string(), 1.0 / 14.0),
        ("B3".to_string(), 1.0 / 14.0),
        ("V".to_string(), 1.0 / 14.0),
        ("W".to_string(), 1.0 / 14.0),
        ("C1".to_string(), 1.0 / 14.0),
        ("C2".to_string(), 1.0 / 14.0),
        ("C3".to_string(), 1.0 / 14.0),
    ]);

    assert!(check_distribution(&mut ph1, 100000, &dist1, 0.01));
    assert!(check_distribution(&mut ph2, 100000, &dist2, 0.01));

    assert_eq!(ph1.combination_number(), 14);
    assert_eq!(ph1.weight(), 14.0);
    assert_eq!(ph1.number_of_syntax(), 2);

    assert_eq!(ph2.combination_number(), 14);
    assert_eq!(ph2.weight(), 14.0);
    assert_eq!(ph2.number_of_syntax(), 2);
}

#[test]
fn test_struct_generator_new() {
    let mut ph: Generator = Generator::new();

    assert_eq!(ph.generate(), "nil");
    assert_eq!(ph.combination_number(), 0);
    assert_eq!(ph.weight(), 0.0);
    assert_eq!(ph.number_of_syntax(), 0);
}

#[test]
fn test_struct_generator_generate() {
    let mut ph: Generator<ZeroNG> = r#"
        main = A | B | C
    "#
    .parse()
    .unwrap();

    assert_eq!(ph.generate(), "A");
}

#[test]
fn test_struct_generator_generate_with_context() {
    let mut ph: Generator<ZeroNG> = r#"
        main = {A} | B | C
    "#
    .parse()
    .unwrap();

    assert_eq!(
        ph.generate_with_context(&ExtContext::from([("A".to_string(), "a".to_string()),])),
        "a"
    );
}

#[test]
fn test_struct_generator_add() {
    let mut ph: Generator<ZeroNG> = r#"
        main = {= X | Y | Z } | {A} | {B}
        A = A1 | A2 | A3
        B = B1 | B2 | B3
    "#
    .parse()
    .unwrap();
    let syntax: Syntax = r#"
        main = {= V | W } | {C}
        C = C1 | C2 | C3
    "#
    .parse()
    .unwrap();
    let id: SyntaxID = ph.add(syntax).unwrap();
    assert_eq!(id, 2);
    assert_eq!(ph.generate(), "X");
    assert_eq!(ph.combination_number(), 14);
    assert_eq!(ph.weight(), 14.0);
    assert_eq!(ph.number_of_syntax(), 2);
}

#[test]
fn test_struct_generator_add_with_start_condition() {
    let syntax: Syntax = r#"
        main = MAIN
        alt = ALT
    "#
    .parse()
    .unwrap();

    let mut ph: Generator = Generator::new();
    let id: SyntaxID = ph.add_with_start_condition(syntax, "alt").unwrap();
    assert_eq!(id, 1);
    assert_eq!(ph.generate(), "ALT");
    assert_eq!(ph.combination_number(), 1);
    assert_eq!(ph.weight(), 1.0);
    assert_eq!(ph.number_of_syntax(), 1);
}

#[test]
fn test_struct_generator_add_and_error() {
    let mut ph: Generator<ZeroNG> = r#"
        main = {= X | Y | Z } | {A} | {B}
        A = A1 | A2 | A3
        B = B1 | B2 | B3
    "#
    .parse()
    .unwrap();
    let syntax1: Syntax = r#"
        main = {= V | W } | {C}
        C = C1 | C2 | C3
    "#
    .parse()
    .unwrap();
    assert!(ph.add_with_start_condition(syntax1, "MAIN").is_err());
    let syntax2: Syntax = r#"
        C = C4
    "#
    .parse()
    .unwrap();
    let id2: SyntaxID = ph.add_with_start_condition(syntax2, "C").unwrap();
    assert_eq!(id2, 2);
    assert_eq!(ph.generate(), "X");
    assert_eq!(ph.combination_number(), 10);
    assert_eq!(ph.weight(), 10.0);
    assert_eq!(ph.number_of_syntax(), 2);
}

#[test]
fn test_struct_generator_remove_first_phrase() {
    let mut ph: Generator<Point9NG> = Generator::new();

    let syntax1: Syntax = r#"main = "1" 2 | 2 | 3"#.parse().unwrap();
    let syntax2: Syntax = r#"main = A | "B" 3 | C"#.parse().unwrap();
    let syntax3: Syntax = r#"main = あ | い | "う" 4"#.parse().unwrap();
    let id1: SyntaxID = ph.add(syntax1).unwrap();
    let id2: SyntaxID = ph.add(syntax2).unwrap();
    let id3: SyntaxID = ph.add(syntax3).unwrap();
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);

    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 9);
    assert_eq!(ph.weight(), 15.0);
    assert_eq!(ph.number_of_syntax(), 3);

    let _ = ph.remove(id1).unwrap();

    assert!(ph.remove(id1).is_err());
    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 6);
    assert_eq!(ph.weight(), 11.0);
    assert_eq!(ph.number_of_syntax(), 2);

    let _ = ph.remove(id2).unwrap();

    assert!(ph.remove(id2).is_err());
    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 3);
    assert_eq!(ph.weight(), 6.0);
    assert_eq!(ph.number_of_syntax(), 1);

    let _ = ph.remove(id3).unwrap();

    assert!(ph.remove(id3).is_err());
    assert_eq!(ph.generate(), "nil");
    assert_eq!(ph.combination_number(), 0);
    assert_eq!(ph.weight(), 0.0);
    assert_eq!(ph.number_of_syntax(), 0);
}

#[test]
fn test_struct_generator_remove_last_phrase() {
    let mut ph: Generator<Point9NG> = Generator::new();

    let syntax1: Syntax = r#"main = "1" 2 | 2 | 3"#.parse().unwrap();
    let syntax2: Syntax = r#"main = A | "B" 3 | C"#.parse().unwrap();
    let syntax3: Syntax = r#"main = あ | い | "う" 4"#.parse().unwrap();
    let id1: SyntaxID = ph.add(syntax1).unwrap();
    let id2: SyntaxID = ph.add(syntax2).unwrap();
    let id3: SyntaxID = ph.add(syntax3).unwrap();
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);

    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 9);
    assert_eq!(ph.weight(), 15.0);
    assert_eq!(ph.number_of_syntax(), 3);

    let _ = ph.remove(id3).unwrap();

    assert!(ph.remove(id3).is_err());
    assert_eq!(ph.generate(), "C");
    assert_eq!(ph.combination_number(), 6);
    assert_eq!(ph.weight(), 9.0);
    assert_eq!(ph.number_of_syntax(), 2);

    let _ = ph.remove(id2).unwrap();

    assert!(ph.remove(id2).is_err());
    assert_eq!(ph.generate(), "3");
    assert_eq!(ph.combination_number(), 3);
    assert_eq!(ph.weight(), 4.0);
    assert_eq!(ph.number_of_syntax(), 1);

    let _ = ph.remove(id1).unwrap();

    assert!(ph.remove(id1).is_err());
    assert_eq!(ph.generate(), "nil");
    assert_eq!(ph.combination_number(), 0);
    assert_eq!(ph.weight(), 0.0);
    assert_eq!(ph.number_of_syntax(), 0);
}

#[test]
fn test_struct_generator_remove_middle_phrase() {
    let mut ph: Generator<Point9NG> = Generator::new();

    let syntax1: Syntax = r#"main = "1" 2 | 2 | 3"#.parse().unwrap();
    let syntax2: Syntax = r#"main = A | "B" 3 | C"#.parse().unwrap();
    let syntax3: Syntax = r#"main = あ | い | "う" 4"#.parse().unwrap();
    let id1: SyntaxID = ph.add(syntax1).unwrap();
    let id2: SyntaxID = ph.add(syntax2).unwrap();
    let id3: SyntaxID = ph.add(syntax3).unwrap();
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);

    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 9);
    assert_eq!(ph.weight(), 15.0);
    assert_eq!(ph.number_of_syntax(), 3);

    let _ = ph.remove(id2).unwrap();

    assert!(ph.remove(id2).is_err());
    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 6);
    assert_eq!(ph.weight(), 10.0);
    assert_eq!(ph.number_of_syntax(), 2);

    let _ = ph.remove(id1).unwrap();

    assert!(ph.remove(id1).is_err());
    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 3);
    assert_eq!(ph.weight(), 6.0);
    assert_eq!(ph.number_of_syntax(), 1);

    let _ = ph.remove(id3).unwrap();

    assert!(ph.remove(id3).is_err());
    assert_eq!(ph.generate(), "nil");
    assert_eq!(ph.combination_number(), 0);
    assert_eq!(ph.weight(), 0.0);
    assert_eq!(ph.number_of_syntax(), 0);
}

#[test]
fn test_struct_generator_remove_and_add_phrase() {
    let mut ph: Generator<Point9NG> = Generator::new();

    let syntax1: Syntax = r#"main = 1"#.parse().unwrap();
    let syntax2: Syntax = r#"main = A | B"#.parse().unwrap();
    let syntax3: Syntax = r#"main = あ | い | う"#.parse().unwrap();
    let id1: SyntaxID = ph.add(syntax1).unwrap();
    let id2: SyntaxID = ph.add(syntax2).unwrap();
    let id3: SyntaxID = ph.add(syntax3).unwrap();
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);

    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 6);
    assert_eq!(ph.weight(), 6.0);
    assert_eq!(ph.number_of_syntax(), 3);

    let _ = ph.remove(id2).unwrap();

    assert!(ph.remove(id2).is_err());
    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 4);
    assert_eq!(ph.weight(), 4.0);
    assert_eq!(ph.number_of_syntax(), 2);

    let syntax4: Syntax = r#"main = 11 | 12 | 13 | 14"#.parse().unwrap();
    let id4: SyntaxID = ph.add(syntax4).unwrap();
    assert_eq!(id4, 4);

    assert_eq!(ph.generate(), "14");
    assert_eq!(ph.combination_number(), 8);
    assert_eq!(ph.weight(), 8.0);
    assert_eq!(ph.number_of_syntax(), 3);

    let _ = ph.remove(id4).unwrap();

    assert!(ph.remove(id4).is_err());
    assert_eq!(ph.generate(), "う");
    assert_eq!(ph.combination_number(), 4);
    assert_eq!(ph.weight(), 4.0);
    assert_eq!(ph.number_of_syntax(), 2);

    let syntax5: Syntax = r#"main = AA | BB | CC | DD | EE"#.parse().unwrap();
    let id5: SyntaxID = ph.add(syntax5).unwrap();
    assert_eq!(id5, 4);

    assert_eq!(ph.generate(), "EE");
    assert_eq!(ph.combination_number(), 9);
    assert_eq!(ph.weight(), 9.0);
    assert_eq!(ph.number_of_syntax(), 3);
}

#[test]
fn test_struct_generator_clear() {
    let mut ph: Generator<ZeroNG> = r#"
        main = {= X | Y | Z } | {A} | {B}
        A = A1 | A2 | A3

        B = B1 | B2 | B3
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "X");
    assert_eq!(ph.combination_number(), 9);
    assert_eq!(ph.weight(), 9.0);
    assert_eq!(ph.number_of_syntax(), 1);

    ph.clear();
    assert_eq!(ph.generate(), "nil");
    assert_eq!(ph.combination_number(), 0);
    assert_eq!(ph.weight(), 0.0);
    assert_eq!(ph.number_of_syntax(), 0);

    let syntax: Syntax = r#"
        main = {= V | W } | {C}
        C = C1 | C2 | C3
    "#
    .parse()
    .unwrap();
    let _ = ph.add(syntax).unwrap();
    assert_eq!(ph.generate(), "V");
    assert_eq!(ph.combination_number(), 5);
    assert_eq!(ph.weight(), 5.0);
    assert_eq!(ph.number_of_syntax(), 1);
}
