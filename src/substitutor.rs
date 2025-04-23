//! Substitutor
//
// Copyright © 2025 OOTA, Masato
//
// This file is part of TPhrase for Rust.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// OR
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use TPhrase for Rust except in compliance with the License. You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

/// The substitutor implementing the gsub function in tphrase.
/// You can replace the default into your version. [`Generator`] doesn't have [`Clone`] and [`Debug`] traits if your instance of [`Substitutor`] doesn't have [`Clone`] and [`Debug`] traits.
///
/// [`Generator`]: struct.Generator.html
pub trait Substitutor {
    /// Create a substitutor. Used in [`parse()`], [`parse_str()`], [`Generator::from_str()`], and [`Syntax::from_str()`].
    ///
    /// [`parse()`]: fn.parse.html
    /// [`parse_str()`]: fn.parse_str.html
    /// [`Generator::from_str()`]: struct.Generator.html#method.from_str
    /// [`Syntax::from_str()`]: struct.Syntax.html#method.from_str
    fn new() -> Self;
    /// Substitute str. Used in [`Generator::generate()`].
    ///
    /// [`Generator::generate()`]: struct.Generator.html#method.generate
    fn gsub<'a>(self: &Self, s: &'a str) -> std::borrow::Cow<'a, str>;
    /// Add an instruction of gsub function. Used in parsing.
    ///
    /// The string in [`Err`] is the error message to show users.
    fn add(
        self: &mut Self,
        pattern: &str,
        repl: String,
        limit: usize,
    ) -> Result<(), SubstitutorAddError>;
}

/// The type that represents the error when adding the parameters to a [`Substitutor`].
#[derive(Clone, Default, Debug)]
pub struct SubstitutorAddError {
    error_message: String,
}
impl SubstitutorAddError {
    /// Create a new instance.
    ///
    /// # Note
    /// - Against the common manner in Rust, the beginning of `msg` should be capital letter and the end is the period.
    pub fn new(msg: String) -> Self {
        Self { error_message: msg }
    }
    /// The error message.
    pub fn error_message(self: &Self) -> &String {
        &self.error_message
    }
}
impl std::fmt::Display for SubstitutorAddError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "substitutor error in add()")?;
        if !self.error_message.is_empty() {
            write!(f, ": \"{}\"", self.error_message)?;
        }
        Ok(())
    }
}
impl std::error::Error for SubstitutorAddError {}
