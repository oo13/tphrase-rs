//! Test for Some Error Structs
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

extern crate tphrase;
use tphrase::*;

#[test]
fn test_substitution_add_error() {
    let err = SubstitutorAddError::new("Message.".to_string());

    // Debug
    assert_eq!(
        format!("{:?}", err),
        "SubstitutorAddError { error_message: \"Message.\" }"
    );

    // Display
    assert_eq!(
        format!("{}", err),
        "substitutor error in add(): \"Message.\""
    );
    assert_eq!(err.to_string(), "substitutor error in add(): \"Message.\"");
}

#[test]
fn test_syntax_remove_error() {
    let err = SyntaxRemoveError::new();

    // Debug
    assert_eq!(format!("{:?}", err), "SyntaxRemoveError");

    // Display
    assert_eq!(format!("{}", err), "error in remove()");
    assert_eq!(err.to_string(), "error in remove()");
}
