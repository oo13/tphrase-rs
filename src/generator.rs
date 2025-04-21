//! Generator
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

use crate::parser::data::Syntax;
use crate::select_and_generate_text;
use crate::ExtContext;
use crate::RandomGenerator;
use crate::Substitutor;

/// The type of Syntax ID. Used when removing a syntax from a generator.
pub type SyntaxID = usize;

/// A translatable phrase generator.
///
/// Add some syntaxes and generate phrases.
///
/// # Example
/// ```rust
/// let syntax = tphrase::parse_str(r#"main = Hello, World!"#).unwrap();
/// let mut ph: tphrase::Generator = tphrase::Generator::new();
/// let _ = ph.add(syntax);
/// assert_eq!(ph.generate(), "Hello, World!");
/// ```
///
/// You can easily create from a string to a phrase generator that has only one syntax.
/// ```rust
/// let mut ph: tphrase::Generator = "main = Hello, World!".parse().unwrap();
/// assert_eq!(ph.generate(), "Hello, World!");
/// ```
///
/// The phrase generator can be added multiple syntaxes.
/// ```rust
/// let mut ph: tphrase::Generator = "main = Hello, World!".parse().unwrap();
/// let syntax: tphrase::Syntax = "main = How are you?".parse().unwrap();
/// let _ = ph.add(syntax);
/// let s = ph.generate();
/// assert!(s == "Hello, World!" ||
///         s == "How are you?");
/// ```
///
/// `Err` from `add()` and `from_str()` holds some human readable error messages.
/// ```rust
/// let mut ph_result: Result<tphrase::Generator, _> = "start = Hello, World!".parse();
/// let err_msgs = ph_result.err().unwrap();
/// assert_eq!(err_msgs.len(), 1);
/// assert_eq!(err_msgs[0], "The nonterminal \"main\" doesn't exist.");
/// ```
#[derive(Clone)]
pub struct Generator<
    R: RandomGenerator = crate::DefaultRandomGenerator,
    S: Substitutor = crate::DefaultSubstitutor,
> {
    /// The syntaxes in the instance.
    syntaxes: Vec<Syntax<S>>,
    /// `weights[i]` is the sum of `weights[i-1]` and the weight to select `syntaxes[i]`.
    weights: Vec<f64>,
    /// Is the chance equalized?
    equalized_chance: bool,
    /// The syntax ID.
    ids: Vec<SyntaxID>,
    /// Random generator.
    rng: R,
}
impl<R: RandomGenerator, S: Substitutor> std::str::FromStr for Generator<R, S> {
    type Err = Vec<String>;
    /// `from_str(s)` is equivalent to `new()` and `add(parse_str(s)?)`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let syntax: Syntax<S> = s.parse()?;
        let mut ph: Generator<R, S> = Generator::new();
        ph.add(syntax)?;
        return Ok(ph);
    }
}
impl<R: RandomGenerator, S: Substitutor> Generator<R, S> {
    /// Create an empty generator.
    pub fn new() -> Self {
        Self {
            syntaxes: Vec::new(),
            weights: Vec::new(),
            equalized_chance: false,
            ids: Vec::new(),
            rng: R::new(),
        }
    }

    /// Generate a phrase.
    ///
    /// # Return
    /// A phrase.
    ///
    /// # Note
    /// - The empty generator creates "nil".
    /// - `self` is mut because `rng` is mut.
    pub fn generate(self: &mut Self) -> String {
        let no_context = super::ExtContext::new();
        return self.generate_with_context(&no_context);
    }

    /// Generate a phrase using an external context.
    ///
    /// # Parameter
    /// - `ext_context`: The external context that has some nonterminals and the substitutions.
    ///
    /// # Return
    /// A phrase.
    ///
    /// # Note
    /// - The empty generator creates "nil".
    /// - `self` is mut because `rng` is mut.
    pub fn generate_with_context(self: &mut Self, ext_context: &ExtContext) -> String {
        select_and_generate_text(
            &self.syntaxes,
            &self.weights,
            self.equalized_chance,
            &ext_context,
            &mut self.rng,
        )
    }

    /// Add a phrase syntax.
    ///
    /// # Parameter
    /// - `syntax`: The phrase syntax to be added.
    ///
    /// # Return
    /// ID for the syntax added into the instance, or a human readable error message if no phrase syntax is added.
    ///
    /// # Note
    /// - Only the phrase syntax that contains the nonterminal "main" can be added.
    /// - The recursive reference to a nonterminal is not allowed.
    /// - The syntax ID is unique only in `self`.
    pub fn add(self: &mut Self, syntax: Syntax<S>) -> Result<SyntaxID, Vec<String>> {
        return self.add_with_start_condition(syntax, "main");
    }

    /// Add a phrase syntax.
    ///
    /// # Parameter
    /// - `syntax`: The phrase syntax to be added.
    /// - `start_condition`: The name of the nonterminal where is the start condition.
    ///
    /// # Return
    /// ID for the syntax added into the instance, or a human readable error message if no phrase syntax is added.
    ///
    /// # Note
    /// - Only the phrase syntax that contains the start condition can be added.
    /// - The recursive reference to a nonterminal is not allowed.
    /// - The syntax ID is unique only in `self`.
    pub fn add_with_start_condition(
        self: &mut Self,
        mut syntax: Syntax<S>,
        start_condition: &str,
    ) -> Result<SyntaxID, Vec<String>> {
        syntax.bind_syntax(start_condition)?;
        let new_weight = syntax.weight();
        self.syntaxes.push(syntax);
        self.weights.push(self.weight() + new_weight);
        let id = match self.ids.last() {
            Some(x) => {
                if *x < std::usize::MAX {
                    *x + 1
                } else {
                    let mut err_msg = Vec::new();
                    err_msg.push("Too many syntaxes.".to_string());
                    return Err(err_msg);
                }
            }
            None => 1,
        };
        self.ids.push(id);
        return Ok(id);
    }

    /// Remove a phrase syntax.
    ///
    /// # Parameter
    /// - `id`: ID for the phrase syntax.
    ///
    /// # Return
    /// `Ok` if the phrase syntax is deleted. `Err` if ID doesn't exist in `self`.
    ///
    /// # Note
    /// - This is an O(n) function, because it's assumed that the function is not used frequently.
    /// -  The ID for the removed phrase syntax may be reused by `add()`.
    pub fn remove(self: &mut Self, syntax_id: SyntaxID) -> Result<(), ()> {
        let i = match self.ids.binary_search(&syntax_id) {
            Ok(x) => x,
            Err(_) => return Err(()),
        };
        self.ids.remove(i);
        self.syntaxes.remove(i);
        self.weights.pop();
        let mut sum: f64 = 0.0;
        if i >= 1 {
            sum = self.weights[i - 1];
        }
        for j in i..self.syntaxes.len() {
            sum += self.syntaxes[j].weight();
            self.weights[j] = sum;
        }
        return Ok(());
    }

    /// Clear the syntaxes and create an empty phrase generator.
    pub fn clear(self: &mut Self) {
        self.syntaxes.clear();
        self.weights.clear();
        self.ids.clear();
    }

    /// Equalize the chance to select each phrase syntax.
    ///
    /// # Parameter
    /// - `enable`: equalized if `enable` is true. If not, the chance depends on the weight of the phrase syntax. (Default)
    pub fn equalize_chance(self: &mut Self, enable: bool) {
        self.equalized_chance = enable;
    }

    /// The number of the syntaxes in the instance.
    ///
    /// # Return
    /// The number of the syntaxes in the instance.
    pub fn number_of_syntax(self: &Self) -> usize {
        self.syntaxes.len()
    }

    /// The sum of the weight of the syntaxes in the instance.
    ///
    /// # Return
    /// The sum of the weight of the syntaxes in the instance.
    pub fn weight(self: &Self) -> f64 {
        if let Some(x) = self.weights.last() {
            *x
        } else {
            0.0
        }
    }

    /// The number of the possible phrases generated by the instance.
    ///
    /// # Return
    /// The the number of the possible phrases generated by the instance.
    pub fn combination_number(self: &Self) -> usize {
        self.syntaxes
            .iter()
            .fold(0, |acc, x| acc + x.combination_number())
    }
}
