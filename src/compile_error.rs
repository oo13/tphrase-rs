//! CompileError
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

use std::error;
use std::fmt;

/// The type that represents the error when parsing and binding a phrase syntax.
///
/// # Example
/// ```rust
/// let syntax_result: Result<tphrase::Syntax, _> = r#"
///     main = "Hello, " {WORLD}
///     WORLD
///       = world
/// "#.parse();
/// assert!(syntax_result.is_err());
/// if let Err(mut err) = syntax_result {
///     // The type of 'err' is tphrase::CompileError.
///     assert_eq!(err.error_messages().len(), 3);
///
///     // Debug
///     assert_eq!(format!("{:?}", err),
///                r#"CompileError { ["Line#2, Column#22: The end of the text or \"\\n\" is expected.", "Line#3, Column#10: \"=\" or \":=\" is expected.", "Line#4, Column#7: A nonterminal \"[A-Za-z0-9_.]+\" is expected."] }"#);
///
///     // Display
///     assert_eq!(
///         format!("{}", err),
///         format!("{}{}{}{}",
///             "compile error:\n",
///             "Line#2, Column#22: The end of the text or \"\\n\" is expected.\n",
///             "Line#3, Column#10: \"=\" or \":=\" is expected.\n",
///             "Line#4, Column#7: A nonterminal \"[A-Za-z0-9_.]+\" is expected."));
///     assert_eq!(
///         err.to_string(),
///         format!("{}{}{}{}",
///             "compile error:\n",
///             "Line#2, Column#22: The end of the text or \"\\n\" is expected.\n",
///             "Line#3, Column#10: \"=\" or \":=\" is expected.\n",
///             "Line#4, Column#7: A nonterminal \"[A-Za-z0-9_.]+\" is expected."));
///
///     // Display with a specific separators.
///     err.set_separators(
///         " ".to_string(),
///         "\"".to_string(),
///         "\"".to_string(),
///         ", ".to_string(),
///     );
///     assert_eq!(
///         format!("{}", err),
///         r#"compile error: "Line#2, Column#22: The end of the text or "\n" is expected.", "Line#3, Column#10: "=" or ":=" is expected.", "Line#4, Column#7: A nonterminal "[A-Za-z0-9_.]+" is expected.""#
///     );
///
///     // Display without the details.
///     err.omit_details(true);
///     assert_eq!(format!("{}", err), "compile error");
///
///     // Contents of the datailed error messages.
///     assert_eq!(err.error_messages().len(), 3);
///     assert_eq!(err.error_messages()[0], r#"Line#2, Column#22: The end of the text or "\n" is expected."#);
///     assert_eq!(err.error_messages()[1], r#"Line#3, Column#10: "=" or ":=" is expected."#);
///     assert_eq!(err.error_messages()[2], r#"Line#4, Column#7: A nonterminal "[A-Za-z0-9_.]+" is expected."#);
/// }
/// ```
#[derive(Clone, Default)]
pub struct CompileError {
    /// Detailed error messages.
    ///
    /// # Note
    /// - The length may be more than one if the multiple parse errors are detected.
    /// - Against the common manner in Rust, the beginning of the detailed error message is capital letter and the end is the period.
    error_messages: Vec<String>,
    /// Does the output with [`std::fmt::Display`] omit the detailed error messages?
    omit_details: bool,
    /// The string inserted in the beginning of the entire detailed error message.
    begin_of_message: String,
    /// The string inserted in the beginning of each detailed error message.
    quote: String,
    /// The string inserted in the end of each detailed error message.
    unquote: String,
    /// The string inserted between the detailed error messages.
    delimiter: String,
}
impl fmt::Debug for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CompileError {{ {:?} }}", self.error_messages)
    }
}
impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "compile error")?;
        if !self.omit_details && !self.error_messages.is_empty() {
            write!(f, ":{}", self.begin_of_message)?;
            let mut is_first = true;
            for err_msg in self.error_messages.iter() {
                if !is_first {
                    write!(f, "{}", self.delimiter)?;
                }
                is_first = false;
                write!(f, "{}{}{}", self.quote, err_msg, self.unquote)?;
            }
        }
        Ok(())
    }
}
impl error::Error for CompileError {}
impl CompileError {
    /// Create a new instance.
    pub(crate) fn new() -> Self {
        Self {
            error_messages: Vec::new(),
            omit_details: false,
            begin_of_message: "\n".to_string(),
            quote: "".to_string(),
            unquote: "".to_string(),
            delimiter: "\n".to_string(),
        }
    }
    /// The separators using the output with [`std::fmt::Display`].
    ///
    /// # Parameter
    /// - `begin_of_message`: The string inserted in the beginning of the entire detailed error message.
    /// - `quote`: The string inserted in the beginning of each detailed error message.
    /// - `unquote`: The string inserted in the end of each detailed error message.
    /// - `delimiter`: The string inserted between the detailed error messages.
    pub fn set_separators(
        self: &mut Self,
        begin_of_message: String,
        quote: String,
        unquote: String,
        delimiter: String,
    ) {
        self.begin_of_message = begin_of_message;
        self.quote = quote;
        self.unquote = unquote;
        self.delimiter = delimiter;
    }
    /// Does the output with [`std::fmt::Display`] omit the detailed error messages?
    ///
    /// # Parameter
    /// - `enable`: Omit the details if `enable` is `true`.
    pub fn omit_details(self: &mut Self, enable: bool) {
        self.omit_details = enable;
    }
    /// Add an error message to the detailed error messages.
    pub(crate) fn add_error_message(self: &mut Self, s: String) {
        self.error_messages.push(s);
    }
    /// Add error messages to the detailed error messages.
    pub(crate) fn add_error_messages(self: &mut Self, s: Vec<String>) {
        self.error_messages = s;
    }
    /// The detailed error messages.
    pub fn error_messages(self: &Self) -> &Vec<String> {
        &self.error_messages
    }
}
