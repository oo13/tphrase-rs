//! Utility functions for test
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

use tphrase::*;

#[derive(Clone, Debug)]
pub struct ZeroNG {}
impl RandomNumberGenerator for ZeroNG {
    fn new() -> Self {
        Self {}
    }
    fn next(self: &mut Self) -> f64 {
        0.0
    }
}

#[derive(Clone, Debug)]
pub struct Point9NG {}
impl RandomNumberGenerator for Point9NG {
    fn new() -> Self {
        Self {}
    }
    fn next(self: &mut Self) -> f64 {
        0.9
    }
}

pub trait LinearNGParam {
    fn new() -> Self;
    fn max() -> f64;
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LinearNG<P: LinearNGParam> {
    n: P,
    i: f64,
}
impl<P: LinearNGParam> RandomNumberGenerator for LinearNG<P> {
    fn new() -> Self {
        Self {
            n: P::new(),
            i: 0.0,
        }
    }
    fn next(self: &mut Self) -> f64 {
        if self.i >= P::max() {
            0.0
        } else {
            self.i += 1.0;
            (self.i - 0.5) / P::max()
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinearNGParam3 {}
impl LinearNGParam for LinearNGParam3 {
    fn new() -> Self {
        Self {}
    }
    fn max() -> f64 {
        3.0
    }
}

#[derive(Clone, Debug)]
pub struct LinearNGParam6 {}
impl LinearNGParam for LinearNGParam6 {
    fn new() -> Self {
        Self {}
    }
    fn max() -> f64 {
        6.0
    }
}

#[allow(dead_code)]
pub type LinearNG3 = LinearNG<LinearNGParam3>;
#[allow(dead_code)]
pub type LinearNG6 = LinearNG<LinearNGParam6>;

#[allow(dead_code)]
pub type TextDistribution = std::collections::HashMap<String, f64>;

#[allow(dead_code)]
pub fn check_distribution(
    ph: &mut Generator,
    num: usize,
    dist: &TextDistribution,
    allowance: f64,
) -> bool {
    let mut count = std::collections::HashMap::<String, usize>::new();
    for _ in 0..num {
        let s = ph.generate();
        match dist.get(&s) {
            Some(_) => *count.entry(s).or_insert(1) += 1,
            None => {
                println!("The result \"{}\" is not expected.", s);
                return false;
            }
        }
    }

    let mut is_match = true;
    for (s, probability) in dist.iter() {
        let mut op: f64 = 0.0;
        if let Some(x) = count.get(s) {
            op = (*x as f64) / (num as f64);
        }
        if (op - probability).abs() > allowance {
            println!(
                "The probability ({}) of the result \"{}\" is not match the expected value {}.",
                op, s, probability
            );
            is_match = false;
        }
    }
    return is_match;
}
