//! Test for Readme.md
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

fn gettext(s: &str) -> String {
    s.to_string()
}

#[test]
fn test_readme_1() {
    // ---
    let mut rng = fastrand::Rng::new();
    let word = if rng.f64() > 0.5 {
        gettext("poor")
    } else {
        gettext("rich")
    };
    let message = if rng.f64() > 0.5 {
        gettext("You are {}.").replace("{}", &word)
    } else {
        gettext("You purchased a {} ship.").replace("{}", &word)
    };
    // ---

    assert!(word == "poor" || word == "rich");
    assert!(
        message == "You are poor."
            || message == "You are rich."
            || message == "You purchased a poor ship."
            || message == "You purchased a rich ship."
    );
}

#[test]
fn test_readme_2() {
    let money = 1000;
    let cost = 1000000;

    // ---
    let s1 = if money < 10000 {
        gettext("poor")
    } else {
        gettext("rich")
    };
    let mut ph1: tphrase::Generator = gettext("main = You are {ECONOMICAL_SITUATION}.")
        .parse()
        .unwrap();
    let r1 = ph1.generate_with_context(&tphrase::ExtContext::from([(
        "ECONOMICAL_SITUATION".to_string(),
        s1,
    )]));
    // ...snip...
    let s2 = if cost < 10000 {
        gettext("poor")
    } else {
        gettext("rich")
    };
    let mut ph2: tphrase::Generator =
        gettext("main = You purchased a {ECONOMICAL_SITUATION} ship.")
            .parse()
            .unwrap();
    let r2 = ph2.generate_with_context(&tphrase::ExtContext::from([(
        "ECONOMICAL_SITUATION".to_string(),
        s2,
    )]));
    // ---

    assert!(r1 == "You are poor.");
    assert!(r2 == "You purchased a rich ship.");
}

#[test]
fn test_readme_3() {
    let money = 1000;
    let cost = 1000000;

    // ---
    let s1 = if money < 10000 {
        gettext("poor")
    } else {
        gettext("rich")
    };
    let r1 = gettext("You are {}.").replace("{}", &s1);
    // ...snip...
    let s2 = if cost < 10000 {
        gettext("poor")
    } else {
        gettext("rich")
    };
    let r2 = gettext("You purchased a {} ship.").replace("{}", &s2);
    // ---

    assert!(r1 == "You are poor.");
    assert!(r2 == "You purchased a rich ship.");
}

#[test]
fn test_readme_4() {
    let money = 1000;
    let cost = 1000000;

    // ---
    let r1 = if money < 10000 {
        gettext("You are poor.")
    } else {
        gettext("You are rich.")
    };
    // ...snip...
    let r2 = if cost < 10000 {
        gettext("You purchased a poor ship.")
    } else {
        gettext("You purchased a rich ship.")
    };
    // ---

    assert!(r1 == "You are poor.");
    assert!(r2 == "You purchased a rich ship.");
}
