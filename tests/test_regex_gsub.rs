//! Test for RegexGsub
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

use std::borrow::Cow;

#[test]
fn test_simple_replace() {
    let mut gsub = RegexGsub::new();
    assert!(matches!(gsub.add("abc", "def".to_string(), 0), Ok(_)));
    assert_eq!(gsub.gsub("abcdef"), "defdef");
}

#[test]
fn test_multiple_rules() {
    let mut gsub = RegexGsub::new();
    assert!(matches!(gsub.add("abc", "def".to_string(), 0), Ok(_)));
    assert_eq!(gsub.gsub("abcdef"), "defdef");
    assert!(matches!(gsub.add("e", "E".to_string(), 1), Ok(_)));
    assert_eq!(gsub.gsub("abcdef"), "dEfdef");
}

#[test]
fn test_regex_compile_error() {
    let mut gsub = RegexGsub::new();
    assert!(matches!(gsub.add("[[[]", "Z".to_string(), 0), Err(_)));
}

#[test]
fn test_cow() {
    let mut gsub = RegexGsub::new();
    assert!(matches!(gsub.add("abc", "def".to_string(), 0), Ok(_)));
    assert!(matches!(gsub.gsub("abcdef"), Cow::Owned(_)));

    /* Don't copy if not match. */
    assert!(matches!(gsub.gsub("zbcdef"), Cow::Borrowed(_)));
}
