//! Test for utility functions
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

#[test]
fn test_trunc_syntax() {
    let s = "main = \"0123456789\"".to_string();
    assert_eq!(trunc_syntax(&mut s.chars(), 40), "main = \"0123456789\"");
    assert_eq!(
        trunc_syntax(&mut s[..].chars(), 40),
        "main = \"0123456789\""
    );
}

#[test]
fn test_trunc_syntax_str() {
    assert_eq!(
        trunc_syntax_str("main = \"0123456789\"", 40),
        "main = \"0123456789\""
    );
    assert_eq!(
        trunc_syntax_str(
            "
        main = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    ",
            30
        ),
        "main = 0 | 1 | 2 | 3 | 4 | 5 |..."
    );
    assert_eq!(
        trunc_syntax_str(
            "
        main = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    ",
            25
        ),
        "main = 0 | 1 | 2 | 3 | 4..."
    );
    assert_eq!(
        trunc_syntax_str(
            "
        main = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    ",
            15
        ),
        "main = 0 | 1 |..."
    );
    assert_eq!(
        trunc_syntax_str("main = Hello, World!", 10),
        "main = Hello,..."
    );
    assert_eq!(trunc_syntax_str("main = A      A!", 10), "main = A...");
    assert_eq!(
        trunc_syntax_str("  main = A   A   A!", 14),
        "main = A   A..."
    );
    assert_eq!(
        trunc_syntax_str(
            "
        main = {HELLO}, {WORLD}!
        HELLO = Hello
        WORLD = World
    ",
            10
        ),
        "main = {HELLO},..."
    );
}

#[test]
fn test_truncate_preceding_spaces() {
    assert_eq!(
        trunc_syntax_str(
            "
\t          main = \"0123456789\"",
            40
        ),
        "main = \"0123456789\""
    );
}

#[test]
fn test_truncate_succeeding_spaces() {
    assert_eq!(
        trunc_syntax_str("main = \"0123456789\"\t", 40),
        "main = \"0123456789\""
    );
}

#[test]
fn test_truncate_characters() {
    assert_eq!(
        trunc_syntax_str(
            "
        |||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||",
            40
        ),
        "||||||||||||||||||||||||||||||||||||||||..."
    );
    assert_eq!(
        trunc_syntax_str(
            "
        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
            40
        ),
        "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~..."
    );
    assert_eq!(
        trunc_syntax_str(
            "
        =================================================================",
            40
        ),
        "========================================..."
    );
    assert_eq!(
        trunc_syntax_str(
            "
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a",
            40
        ),
        "a a a a a a a a a a a a a a a a a a a a..."
    );
    assert_eq!(trunc_syntax_str("
        a\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta", 40), "a\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta\ta...");
}

#[test]
fn test_not_truncate_characters() {
    assert_eq!(
        trunc_syntax_str(
            "
        -----------------------------------------------------------------",
            40
        ),
        "-----------------------------------------------------------------"
    );
    assert_eq!(
        trunc_syntax_str(
            "
        jugemujugemugokounosurikirekaijarisuigyonosuigyoumatuunraimatuhuuraimatu",
            40
        ),
        "jugemujugemugokounosurikirekaijarisuigyonosuigyoumatuunraimatuhuuraimatu"
    );
}

#[test]
fn test_truncate_nl() {
    assert_eq!(
        trunc_syntax_str(
            "
                    1 2 3
",
            40
        ),
        "1 2 3..."
    );
}
