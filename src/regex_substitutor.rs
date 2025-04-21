//! RegexGsub
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

use super::Substitutor;
use std::borrow::Cow;

/// The type representing one gsub instruction.
#[derive(Clone)]
struct Replacer {
    matcher: regex::Regex,
    replace: String,
    limit: usize,
}

impl Replacer {
    fn new(pattern: &str, repl: String, limit: usize) -> Result<Self, String> {
        match regex::Regex::new(pattern) {
            Err(e) => {
                let err_msg = match e {
                    regex::Error::Syntax(s) => s,
                    regex::Error::CompiledTooBig(_) => "Compiled Too Big".to_string(),
                    _ => "Unknown Error".to_string(),
                };
                return Result::Err(err_msg);
            }
            Ok(re) => {
                return Ok(Self {
                    matcher: re,
                    replace: repl,
                    limit,
                });
            }
        };
    }
    fn replace<'a>(self: &Self, s: &'a Cow<'a, str>) -> Cow<'a, str> {
        self.matcher.replacen(s, self.limit, &self.replace)
    }
}

/// A type of `Substitutor` using `regex::Regex`. The default substitutor of `Generator`.
#[derive(Clone)]
pub struct RegexGsub {
    params: Vec<Replacer>,
}

impl Substitutor for RegexGsub {
    fn new() -> Self {
        Self { params: Vec::new() }
    }
    fn gsub<'a>(self: &Self, s: &'a str) -> Cow<'a, str> {
        let mut r = Cow::from(s);
        for param in self.params.iter() {
            if let Cow::Owned(s2) = param.replace(&r) {
                *r.to_mut() = s2;
            }
        }
        r
    }
    fn add(self: &mut Self, pattern: &str, repl: String, limit: usize) -> Result<(), String> {
        match Replacer::new(pattern, repl, limit) {
            Ok(r) => {
                self.params.push(r);
                Result::Ok(())
            }
            Err(s) => Result::Err(s),
        }
    }
}
