//! Test for generating phrases
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
fn test_generate_no_options() {
    let mut ph: Generator = r#"
        main = ""
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "");
}

#[test]
fn test_generate_no_weight_options() {
    let mut ph: Generator<LinearNG3> = r#"
        main = A | B | C
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A");
    assert_eq!(ph.generate(), "B");
    assert_eq!(ph.generate(), "C");
}

#[test]
fn test_generate_weighted_options() {
    let mut ph: Generator<LinearNG6> = r#"
        main = A | "B" 2 | "C" 3
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A");
    assert_eq!(ph.generate(), "B");
    assert_eq!(ph.generate(), "B");
    assert_eq!(ph.generate(), "C");
    assert_eq!(ph.generate(), "C");
    assert_eq!(ph.generate(), "C");
}

#[test]
fn test_generate_weighted_and_equalized_options() {
    let mut ph: Generator<LinearNG6> = r#"
        main := A | "B" 2 | "C" 3
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A");
    assert_eq!(ph.generate(), "A");
    assert_eq!(ph.generate(), "B");
    assert_eq!(ph.generate(), "B");
    assert_eq!(ph.generate(), "C");
    assert_eq!(ph.generate(), "C");
}

#[test]
fn test_generate_options_distribution() {
    let mut ph: Generator = r#"
        main = {A1} | {A2}
        A1 = 0 | 1 | 2
        A2 = {A21} | {A22}
        A21 = 3 | 4
        A22 = 5 | 6 | 7 | 8 | 9
    "#
    .parse()
    .unwrap();
    let dist = TextDistribution::from([
        ("0".to_string(), 0.1),
        ("1".to_string(), 0.1),
        ("2".to_string(), 0.1),
        ("3".to_string(), 0.1),
        ("4".to_string(), 0.1),
        ("5".to_string(), 0.1),
        ("6".to_string(), 0.1),
        ("7".to_string(), 0.1),
        ("8".to_string(), 0.1),
        ("9".to_string(), 0.1),
    ]);
    assert!(check_distribution(&mut ph, 100000, &dist, 0.01));
}

#[test]
fn test_generate_options_distribution_equalized() {
    let mut ph: Generator = r#"
        main = {A1} | {A2}
        A1 = 0 | 1 | 2
        A2 := {A21} | {A22}
        A21 = 3 | 4
        A22 = 5 | 6 | 7 | 8 | 9
    "#
    .parse()
    .unwrap();
    let dist = TextDistribution::from([
        ("0".to_string(), 0.1),
        ("1".to_string(), 0.1),
        ("2".to_string(), 0.1),
        ("3".to_string(), 0.175),
        ("4".to_string(), 0.175),
        ("5".to_string(), 0.07),
        ("6".to_string(), 0.07),
        ("7".to_string(), 0.07),
        ("8".to_string(), 0.07),
        ("9".to_string(), 0.07),
    ]);
    assert!(check_distribution(&mut ph, 100000, &dist, 0.01));
}

#[test]
fn test_generate_options_distribution_weighted() {
    let mut ph: Generator = r#"
        main = text1 | {B}
        B = text2 | "{C}" 2
        C = 1 | 2 | 3
    "#
    .parse()
    .unwrap();
    let dist = TextDistribution::from([
        ("text1".to_string(), 0.25),
        ("text2".to_string(), 0.25),
        ("1".to_string(), 0.1667),
        ("2".to_string(), 0.1667),
        ("3".to_string(), 0.1667),
    ]);
    assert!(check_distribution(&mut ph, 100000, &dist, 0.01));
}

#[test]
fn test_generate_options_distribution_many_items() {
    let mut ph: Generator = r#"
        main =
        "00" 5 | "01" | "02" | "03" | "04" | "05" | "06" | "07" | "08" | "09" |
        "10" | "11" 3 | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" |
        "20" | "21" | "22" 4 | "23" | "24" | "25" | "26" | "27" | "28" | "29" |
        "30" | "31" | "32" | "33" 2 | "34" | "35" | "36" | "37" | "38" | "39"
    "#
    .parse()
    .unwrap();
    const D: f64 = 1.0 / 50.0;
    let dist = TextDistribution::from([
        ("00".to_string(), 5.0 * D),
        ("01".to_string(), D),
        ("02".to_string(), D),
        ("03".to_string(), D),
        ("04".to_string(), D),
        ("05".to_string(), D),
        ("06".to_string(), D),
        ("07".to_string(), D),
        ("08".to_string(), D),
        ("09".to_string(), D),
        ("10".to_string(), D),
        ("11".to_string(), 3.0 * D),
        ("12".to_string(), D),
        ("13".to_string(), D),
        ("14".to_string(), D),
        ("15".to_string(), D),
        ("16".to_string(), D),
        ("17".to_string(), D),
        ("18".to_string(), D),
        ("19".to_string(), D),
        ("20".to_string(), D),
        ("21".to_string(), D),
        ("22".to_string(), 4.0 * D),
        ("23".to_string(), D),
        ("24".to_string(), D),
        ("25".to_string(), D),
        ("26".to_string(), D),
        ("27".to_string(), D),
        ("28".to_string(), D),
        ("29".to_string(), D),
        ("30".to_string(), D),
        ("31".to_string(), D),
        ("32".to_string(), D),
        ("33".to_string(), 2.0 * D),
        ("34".to_string(), D),
        ("35".to_string(), D),
        ("36".to_string(), D),
        ("37".to_string(), D),
        ("38".to_string(), D),
        ("39".to_string(), D),
    ]);
    assert!(check_distribution(&mut ph, 100000, &dist, 0.01));
}

#[test]
fn test_generate_anonymous_rule() {
    let mut ph: Generator<LinearNG3> = r#"
        main = 1{= A | B | C }2
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "1A2");
    assert_eq!(ph.generate(), "1B2");
    assert_eq!(ph.generate(), "1C2");
}

#[test]
fn test_generate_anonymous_rule_weighted() {
    let mut ph: Generator<LinearNG6> = r#"
        main = 1{= A | "B" 2 | "C" 3}2
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "1A2");
    assert_eq!(ph.generate(), "1B2");
    assert_eq!(ph.generate(), "1B2");
    assert_eq!(ph.generate(), "1C2");
    assert_eq!(ph.generate(), "1C2");
    assert_eq!(ph.generate(), "1C2");
}

#[test]
fn test_generate_anonymous_rule_weighted_and_equalized() {
    let mut ph: Generator<LinearNG6> = r#"
        main = 1{:= A | "B" 2 | "C" 3}2
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "1A2");
    assert_eq!(ph.generate(), "1A2");
    assert_eq!(ph.generate(), "1B2");
    assert_eq!(ph.generate(), "1B2");
    assert_eq!(ph.generate(), "1C2");
    assert_eq!(ph.generate(), "1C2");
}

#[test]
fn test_generate_special_expansion() {
    let mut ph: Generator = r#"
        main = "A{(}B{"}C{|}D{~}E{)}F{{}G{*comment}H{
}"
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A{B\"C|D~E}F{GH\n");
}

#[test]
fn test_generate_with_external_context() {
    let mut ph: Generator = r#"
        main = {A} {B} {C}
        A = head
        C = tail
    "#
    .parse()
    .unwrap();
    assert_eq!(
        ph.generate_with_context(&ExtContext::from([
            ("B".to_string(), "body".to_string()),
            ("C".to_string(), "foot".to_string()),
        ])),
        "head body tail"
    );
}

#[test]
fn test_generate_gsub() {
    let mut ph: Generator = r#"
        main = "The quick brown fox jumps over the lazy dog." ~ /jumps/jumped/ ~ |dog|dogs|
    "#
    .parse()
    .unwrap();
    assert_eq!(
        ph.generate(),
        "The quick brown fox jumped over the lazy dogs."
    );
}

#[test]
fn test_generate_gsub_captured() {
    let mut ph: Generator = r#"
        main = "tail head" ~ /([a-z]+) ([a-z]+)/$2 $1/
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "head tail");
}

#[test]
fn test_generate_gsub_global() {
    let mut ph: Generator = r#"
        main = "oooooooooooooooooooo
@@@@@@@@@@@@@@@@@@@@ $$$$$$$$$$$$$$$$$$$$" ~ /o/0/ ~|@|a|g ~'\$'S'
    "#
    .parse()
    .unwrap();
    assert_eq!(
        ph.generate(),
        "0ooooooooooooooooooo\naaaaaaaaaaaaaaaaaaaa S$$$$$$$$$$$$$$$$$$$"
    );
}

#[test]
fn test_generate_gsub_number() {
    let mut ph: Generator = r#"
        main = "oooooooooooooooooooo
@@@@@@@@@@@@@@@@@@@@ $$$$$$$$$$$$$$$$$$$$" ~ /o/0/11 ~|@|a|g ~'\$'S'
    "#
    .parse()
    .unwrap();
    assert_eq!(
        ph.generate(),
        "00000000000ooooooooo\naaaaaaaaaaaaaaaaaaaa S$$$$$$$$$$$$$$$$$$$"
    );
}

#[test]
fn test_generate_expansion_external_context_and_gsub() {
    let mut ph: Generator = r#"
        main = {A} {B} {C} ~ /head/HEAD/ ~ /tail/TAIL/ ~ /body/BODY/
        A = head
        C = tail
    "#
    .parse()
    .unwrap();
    assert_eq!(
        ph.generate_with_context(&ExtContext::from([("B".to_string(), "body".to_string()),])),
        "HEAD BODY TAIL"
    );
}

#[test]
fn test_generate_sharing_syntax() {
    let common: Syntax = r#"
        sub = {sub2}
    "#
    .parse()
    .unwrap();
    let mut main1: Syntax = r#"
        main = {sub}
        sub2 = 1
    "#
    .parse()
    .unwrap();
    let mut main2: Syntax = r#"
        main = {sub}
        sub2 = 2
    "#
    .parse()
    .unwrap();
    let _ = main1.add(common.clone()).unwrap();
    let _ = main2.add(common).unwrap();
    let mut ph1: Generator = Generator::new();
    let mut ph2: Generator = Generator::new();
    let _ = ph1.add(main1).unwrap();
    let _ = ph2.add(main2).unwrap();
    assert_eq!(ph1.generate(), "1");
    assert_eq!(ph2.generate(), "2");
}

#[test]
fn test_generate_sharing_syntax_distribution() {
    let common: Syntax = r#"
        sub = {sub2}
    "#
    .parse()
    .unwrap();
    let mut main1: Syntax = r#"
        main = {sub}
        sub2 = 1 | 2 | 3 | 4
    "#
    .parse()
    .unwrap();
    let mut main2: Syntax = r#"
        main = {sub}
        sub2 = A | B
    "#
    .parse()
    .unwrap();
    let _ = main1.add(common.clone()).unwrap();
    let _ = main2.add(common).unwrap();
    let mut ph1: Generator = Generator::new();
    let mut ph2: Generator = Generator::new();
    let _ = ph1.add(main1).unwrap();
    let _ = ph2.add(main2).unwrap();
    let dist1 = TextDistribution::from([
        ("1".to_string(), 0.25),
        ("2".to_string(), 0.25),
        ("3".to_string(), 0.25),
        ("4".to_string(), 0.25),
    ]);
    let dist2 = TextDistribution::from([("A".to_string(), 0.5), ("B".to_string(), 0.5)]);
    assert!(check_distribution(&mut ph1, 100000, &dist1, 0.01));
    assert!(check_distribution(&mut ph2, 100000, &dist2, 0.01));
}

#[test]
fn test_generate_sharing_anonymous_rule() {
    let common: Syntax = r#"
        sub = {= {sub2}}
    "#
    .parse()
    .unwrap();
    let mut main1: Syntax = r#"
        main = {sub}
        sub2 = 1
    "#
    .parse()
    .unwrap();
    let mut main2: Syntax = r#"
        main = {sub}
        sub2 = 2
    "#
    .parse()
    .unwrap();
    let _ = main1.add(common.clone()).unwrap();
    let _ = main2.add(common).unwrap();
    let mut ph1: Generator = Generator::new();
    let mut ph2: Generator = Generator::new();
    let _ = ph1.add(main1).unwrap();
    let _ = ph2.add(main2).unwrap();
    assert_eq!(ph1.generate(), "1");
    assert_eq!(ph2.generate(), "2");
}

#[test]
fn test_generate_sharing_anonymous_rule_distribution() {
    let common: Syntax = r#"
        sub = {= {sub2}}
    "#
    .parse()
    .unwrap();
    let mut main1: Syntax = r#"
        main = {sub}
        sub2 = 1 | 2 | 3 | 4
    "#
    .parse()
    .unwrap();
    let mut main2: Syntax = r#"
        main = {sub}
        sub2 = A | B
    "#
    .parse()
    .unwrap();
    let _ = main1.add(common.clone()).unwrap();
    let _ = main2.add(common).unwrap();
    let mut ph1: Generator = Generator::new();
    let mut ph2: Generator = Generator::new();
    let _ = ph1.add(main1).unwrap();
    let _ = ph2.add(main2).unwrap();
    let dist1 = TextDistribution::from([
        ("1".to_string(), 0.25),
        ("2".to_string(), 0.25),
        ("3".to_string(), 0.25),
        ("4".to_string(), 0.25),
    ]);
    let dist2 = TextDistribution::from([("A".to_string(), 0.5), ("B".to_string(), 0.5)]);
    assert!(check_distribution(&mut ph1, 100000, &dist1, 0.01));
    assert!(check_distribution(&mut ph2, 100000, &dist2, 0.01));
}

#[test]
fn test_generate_overwrite_nonterminal() {
    let sub: Syntax = r#"
        sub = A
    "#
    .parse()
    .unwrap();
    let mut main: Syntax = r#"
        main = {sub}
        sub = B
    "#
    .parse()
    .unwrap();
    let add_result = main.add(sub);
    let mut ph: Generator = Generator::new();
    let _ = ph.add(main).unwrap();
    assert_eq!(ph.generate(), "A");
    assert!(add_result.is_err());
    let err_msg = add_result.unwrap_err();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The nonterminal \"sub\" is already defined. Overwrited by newer."));
}

#[test]
fn test_generate_dont_overwrite_local_nonterminal() {
    let sub: Syntax = r#"
        _sub = A
    "#
    .parse()
    .unwrap();
    let mut main: Syntax = r#"
        main = {_sub}
        _sub = B
    "#
    .parse()
    .unwrap();
    let _ = main.add(sub).unwrap();
    let mut ph: Generator = Generator::new();
    let _ = ph.add(main).unwrap();
    assert_eq!(ph.generate(), "B");
}

#[test]
fn test_generate_sharing_local_nonterminal() {
    let common: Syntax = r#"
        sub = {_sub2}
        _sub2 = {sub3}
    "#
    .parse()
    .unwrap();
    let mut main1: Syntax = r#"
        main = {sub}
        sub3 = 1
    "#
    .parse()
    .unwrap();
    let mut main2: Syntax = r#"
        main = {sub}
        sub3 = 2
    "#
    .parse()
    .unwrap();
    let _ = main1.add(common.clone()).unwrap();
    let _ = main2.add(common).unwrap();
    let mut ph1: Generator = Generator::new();
    let mut ph2: Generator = Generator::new();
    let _ = ph1.add(main1).unwrap();
    let _ = ph2.add(main2).unwrap();
    assert_eq!(ph1.generate(), "1");
    assert_eq!(ph2.generate(), "2");
}

#[test]
fn test_generate_sharing_rule() {
    let mut ph: Generator<ZeroNG> = r#"
        main = {A} | {B} | {C}
        A = A1 A2 {COMMON} | A3 {AB} A4 | {AC} A5 A6
        B = B1 B2 {BA} | B3 {COMMON} B4 | {BC} B5 B6
        C = C1 C2 {CA} | C3 {CB} C4 | {COMMON} C5 C6
        AB = AB1
        AC = AC1
        BA = BA1 | "BA2" 2
        BC = BC1 | BC2
        CA = CA1 | CA2 | "CA3" 3
        CB = CB1 | CB2 | CB3
        COMMON = "1" 2 | {AB} | "2" 3 | {AC} | "3" 4 | {BA} | 4 | {BC} | 5 | {CA} | 6 | {CB} | 7
    "#
    .parse()
    .unwrap();
    assert_eq!(ph.generate(), "A1 A2 1");
    assert_eq!(ph.combination_number(), 19 + 2 + 19 + 4 + 19 + 6);
    assert_eq!(ph.weight(), (28 + 2 + 28 + 5 + 28 + 8) as f64);
}

#[test]
fn test_generate_sharing_rule_and_sharing_syntax() {
    let mut syntax: Syntax = r#"
        main = {A} | {B} | {C}
        A = A1 A2 {COMMON} | A3 {AB} A4 | {AC} A5 A6
        B = B1 B2 {BA} | B3 {COMMON} B4 | {BC} B5 B6
        C = C1 C2 {CA} | C3 {CB} C4 | {COMMON} C5 C6
        AB = AB1
        AC = AC1
        BA = BA1 | "BA2" 2
        BC = BC1 | BC2
        CA = CA1 | CA2 | "CA3" 3
        CB = CB1 | CB2 | CB3
        COMMON = "1" 2 | {AB} | "2" 3 | {AC} | "3" 4 | {BA} | 4 | {BC} | 5 | {CA} | 6 | {CB} | 7
    "#
    .parse()
    .unwrap();
    let mut ph1: Generator<ZeroNG> = Generator::new();
    let _ = ph1.add(syntax.clone()).unwrap();
    let add_result = syntax.add("CB = ''".parse().unwrap());
    let mut ph2: Generator<ZeroNG> = Generator::new();
    let _ = ph2.add(syntax).unwrap();
    assert_eq!(ph1.generate(), "A1 A2 1");
    assert_eq!(ph2.generate(), "A1 A2 1");
    assert_eq!(ph1.combination_number(), 19 + 2 + 19 + 4 + 19 + 6);
    assert_eq!(ph2.combination_number(), 17 + 2 + 17 + 4 + 17 + 4);
    assert_eq!(ph1.weight(), (28 + 2 + 28 + 5 + 28 + 8) as f64);
    assert_eq!(ph2.weight(), (26 + 2 + 26 + 5 + 26 + 6) as f64);
    assert!(add_result.is_err());
    let err_msg = add_result.unwrap_err();
    assert_eq!(err_msg.len(), 1);
    assert!(err_msg[0].contains("The nonterminal \"CB\" is already defined. Overwrited by newer."));
}

#[test]
fn test_generate_nonterminal_with_weight() {
    let mut ph: Generator = r#"
        main 1 = A | B | C | D | E
    "#
    .parse()
    .unwrap();
    let syntax: Syntax = r#"
        main 1 = 1
    "#
    .parse()
    .unwrap();
    let _ = ph.add(syntax).unwrap();
    let dist = TextDistribution::from([
        ("A".to_string(), 0.1),
        ("B".to_string(), 0.1),
        ("C".to_string(), 0.1),
        ("D".to_string(), 0.1),
        ("E".to_string(), 0.1),
        ("1".to_string(), 0.5),
    ]);
    assert!(check_distribution(&mut ph, 100000, &dist, 0.01));
    assert_eq!(ph.combination_number(), 6);
    assert_eq!(ph.weight(), 2.0);
    assert_eq!(ph.number_of_syntax(), 2);
}
