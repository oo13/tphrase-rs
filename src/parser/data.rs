//! Parsed data and Syntax
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

use crate::select_and_generate_text;
use crate::ExtContext;
use crate::RandomGenerator;
use crate::Substitutor;
use crate::TextGenerator;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
type Assignments<S> = HashMap<String, Rc<RefCell<ProductionRule<S>>>>;

/// A part of the text.
enum Part<S: Substitutor> {
    /// The part is a literal text.
    Literal(String),
    /// The part is an expansion. `Option` is `None` when the `Part` doesn't bind to a `Syntax`.
    Expansion(String, Option<Rc<RefCell<ProductionRule<S>>>>),
    /// The part is an anonymous rule.
    AnonymousRule(Rc<RefCell<ProductionRule<S>>>),
}
impl<S: Substitutor> Clone for Part<S> {
    /// # Note
    /// `clone()` has a side-effect that the `Part` unbinds the `Syntax`. `Syntax::clone()` handles it.
    fn clone(self: &Self) -> Self {
        match self {
            Part::Literal(s) => Part::Literal(s.clone()),
            Part::Expansion(s, _) => Part::Expansion(s.clone(), None),
            Part::AnonymousRule(r) => {
                Part::AnonymousRule(Rc::new(RefCell::new(r.borrow().clone())))
            }
        }
    }
}

/// The data structure representing the text.
pub(super) struct Text<S: Substitutor> {
    /// The parts of the text.
    parts: Vec<Part<S>>,
    /// The number of the combination.
    comb: usize,
    /// The weight of the text.
    weight: f64,
    /// Was the weight manually set?
    weight_by_user: bool,
}
impl<S: Substitutor> Clone for Text<S> {
    fn clone(self: &Self) -> Self {
        Self {
            parts: self.parts.clone(),
            comb: self.comb,
            weight: self.weight,
            weight_by_user: self.weight_by_user,
        }
    }
}
impl<S: Substitutor> TextGenerator for Text<S> {
    fn generate<R: RandomGenerator>(self: &Self, ext_context: &ExtContext, rng: &mut R) -> String {
        let mut r = "".to_string();
        for p in self.parts.iter() {
            match p {
                Part::Literal(s) => r += &s,
                Part::Expansion(s, e_opt) => {
                    if let Some(e) = e_opt {
                        r += &e.borrow().generate(ext_context, rng);
                    } else if let Some(ext_str) = ext_context.get(s) {
                        r += ext_str;
                    } else {
                        r += &s;
                    }
                }
                Part::AnonymousRule(e) => {
                    r += &e.borrow().generate(ext_context, rng);
                }
            };
        }
        return r;
    }
}
impl<S: Substitutor> Text<S> {
    /// Create an empty text.
    pub(super) fn new() -> Self {
        Self {
            parts: Vec::new(),
            comb: 1,
            weight: 1.0,
            weight_by_user: false,
        }
    }

    /// Add a string that is a part of the text.
    ///
    /// # Parameter
    /// - `s`: The string.
    pub(super) fn add_string(self: &mut Self, s: String) {
        self.parts.push(Part::Literal(s));
    }

    /// Add an expansion name that is a part of the text.
    ///
    /// # Parameter
    /// - `name`: The expansion name.
    pub(super) fn add_expansion(self: &mut Self, s: String) {
        self.parts.push(Part::Expansion(s, None))
    }

    /// Add an anonymous rule that is a part of the text.
    ///
    /// # Parameter
    /// - `r`: The anonymous rule.
    pub(super) fn add_anonymous_rule(self: &mut Self, r: Rc<RefCell<ProductionRule<S>>>) {
        self.parts.push(Part::AnonymousRule(r));
    }

    /// Set the weight of the text manually.
    ///
    /// # Parameter
    /// - `w`: The weight. The default value is used if weight is `None`.
    ///
    /// # Note
    /// It disable the automatic calculation of the weight if `w` is not `None`.
    pub(super) fn set_weight(self: &mut Self, w: Option<f64>) {
        if let Some(x) = w {
            self.weight = x;
            self.weight_by_user = true;
        } else {
            self.weight_by_user = false;
        }
    }

    /// Bind the instance on a syntax.
    ///
    /// # Parameter
    /// - `assingments`: The assignments in the `Syntax` to be bound on.
    /// - `epoch`: The current binding epoch.
    /// - `err_msg`: The error messages are added if some errors are detected.
    ///
    /// # Error
    /// An error message is added to `err_msg` if this instance detects a recursive expansion.
    fn bind_syntax(
        self: &mut Self,
        assignments: &Assignments<S>,
        epoch: usize,
        err_msg: &mut Vec<String>,
    ) {
        let mut tmp_weight: f64 = 1.0;
        self.comb = 1;
        for p in self.parts.iter_mut() {
            match p {
                Part::AnonymousRule(r) => {
                    r.borrow_mut().bind_syntax(assignments, epoch, err_msg);
                }
                Part::Expansion(s, _) => {
                    if let Some(r) = assignments.get(s) {
                        match r.try_borrow_mut() {
                            Ok(mut rule) => {
                                rule.bind_syntax(assignments, epoch, err_msg);
                                *p = Part::Expansion(s.clone(), Some(Rc::clone(r)));
                            }
                            Err(_) => {
                                let mut msg = "Recursive expansion of \"".to_string();
                                msg += &s;
                                msg += "\" is detected.";
                                err_msg.push(msg);
                            }
                        }
                    }
                }
                _ => (),
            };

            let mut tmp_comb = self.comb;
            let mut update_wc = |r: &Rc<RefCell<ProductionRule<S>>>| {
                tmp_comb *= r.borrow().combination_number();
                tmp_weight *= r.borrow().weight();
            };
            match p {
                Part::AnonymousRule(r) => update_wc(r),
                Part::Expansion(_, r_opt) => {
                    if let Some(r) = r_opt {
                        update_wc(r);
                    }
                }
                _ => (),
            };
            self.comb = tmp_comb;
        }
        if !self.weight_by_user {
            self.weight = tmp_weight;
        }
    }

    /// Fix the reference to the local nonterminal.
    ///
    /// # Parameter
    /// - `syntax`: The syntax to be fixed.
    /// - `err_msg`: The error messages are added if some errors are detected.
    ///
    /// # Error
    /// An error is caused if the local nonterminal that is referred by a production rule doesn't exists.
    fn fix_local_nonterminal(self: &mut Self, syntax: &Syntax<S>, err_msg: &mut Vec<String>) {
        for p in self.parts.iter_mut() {
            if let Part::Expansion(s, _) = p {
                if Syntax::<S>::is_local_nonterminal(s) {
                    if let Some(r) = syntax.production_rule(s) {
                        *p = Part::AnonymousRule(r);
                    } else {
                        let mut msg = "The local nonterminal \"".to_string();
                        msg += &s;
                        msg += "\" is not found.";
                        err_msg.push(msg);
                    }
                }
            }
        }
    }

    /// The weight of the texts.
    ///
    /// # Return
    /// The weight.
    ///
    /// # Note
    /// The return value is meaningless when the instance doesn't binds to a `Syntax`.
    fn weight(self: &Self) -> f64 {
        self.weight
    }

    /// The number of the possible texts generated by the instance.
    ///
    /// # Return
    /// The the number of the possible texts generated by the instance.
    fn combination_number(self: &Self) -> usize {
        self.comb
    }
}

/// The data structure representing the set of the text options.
pub(super) struct TextOptions<S: Substitutor> {
    /// The set of the text options.
    texts: Vec<Text<S>>,
    /// `weights[i]` is the sum of `weights[i-1]` and the weight to select `texts[i]`.
    weights: Vec<f64>,
    /// Is the chance equalized?
    equalized_chance: bool,
}
impl<S: Substitutor> Clone for TextOptions<S> {
    fn clone(self: &Self) -> Self {
        Self {
            texts: self.texts.clone(),
            weights: self.weights.clone(),
            equalized_chance: self.equalized_chance,
        }
    }
}
impl<S: Substitutor> TextGenerator for TextOptions<S> {
    fn generate<R: RandomGenerator>(self: &Self, ext_context: &ExtContext, rng: &mut R) -> String {
        select_and_generate_text(
            &self.texts,
            &self.weights,
            self.equalized_chance,
            &ext_context,
            rng,
        )
    }
}
impl<S: Substitutor> TextOptions<S> {
    /// Create an empty `Options`.
    pub(super) fn new() -> Self {
        Self {
            texts: Vec::new(),
            weights: Vec::new(),
            equalized_chance: false,
        }
    }

    /// The weight of the texts.
    ///
    /// # Return
    /// The weight.
    ///
    /// # Note
    /// The return value is meaningless when the instance doesn't binds to a `Syntax`.
    fn weight(self: &Self) -> f64 {
        if let Some(x) = self.weights.last() {
            *x
        } else {
            0.0
        }
    }

    /// The number of the possible texts generated by the instance.
    ///
    /// # Return
    /// The the number of the possible texts generated by the instance.
    fn combination_number(self: &Self) -> usize {
        let mut sum: usize = 0;
        for t in self.texts.iter() {
            sum += t.combination_number();
        }
        return sum;
    }

    /// Add a text.
    ///
    /// # Parameter
    /// - `s`: The text.
    pub(super) fn add_text(self: &mut Self, s: Text<S>) {
        self.texts.push(s);
        self.weights.push(self.weight() + 1.0);
    }

    /// Equalize the chance to select each text.
    ///
    /// # Parameter
    /// - `enable`: equalized if `enable` is `true`. If not, the chance depends on the weight of the text. (Default)
    fn equalize_chance(self: &mut Self, enable: bool) {
        self.equalized_chance = enable;
    }

    /// Bind the instance on a syntax.
    ///
    /// # Parameter
    /// - `assingments`: The assignments in the `Syntax` to be bound on.
    /// - `epoch`: The current binding epoch.
    /// - `err_msg`: The error messages are added if some errors are detected.
    ///
    /// # Error
    /// An error message is added to `err_msg` if this instance detects a recursive expansion.
    fn bind_syntax(
        self: &mut Self,
        assignments: &Assignments<S>,
        epoch: usize,
        err_msg: &mut Vec<String>,
    ) {
        let mut sum: f64 = 0.0;
        for (i, t) in self.texts.iter_mut().enumerate() {
            t.bind_syntax(assignments, epoch, err_msg);
            sum += t.weight();
            self.weights[i] = sum;
        }
    }

    /// Fix the reference to the local nonterminal.
    ///
    /// # Parameter
    /// - `syntax`: The syntax to be fixed.
    /// - `err_msg`: The error messages are added if some errors are detected.
    ///
    /// # Error
    /// An error is caused if the local nonterminal that is referred by a production rule doesn't exists.
    fn fix_local_nonterminal(self: &mut Self, syntax: &mut Syntax<S>, err_msg: &mut Vec<String>) {
        for t in self.texts.iter_mut() {
            t.fix_local_nonterminal(syntax, err_msg);
        }
    }
}

/// The data structure representing the production rule.
pub(super) struct ProductionRule<S: Substitutor> {
    /// The options in the production rule.
    options: TextOptions<S>,
    /// The gsubs in the production rule.
    gsubs: Rc<S>,
    /// The binding epoch.
    binding_epoch: usize,
    /// The weight specified by the phrase syntax.
    weight: Option<f64>,
}
impl<S: Substitutor> Clone for ProductionRule<S> {
    fn clone(self: &Self) -> Self {
        Self {
            options: self.options.clone(),
            gsubs: Rc::clone(&self.gsubs),
            binding_epoch: 0,
            weight: None,
        }
    }
}
impl<S: Substitutor> TextGenerator for ProductionRule<S> {
    fn generate<R: RandomGenerator>(self: &Self, ext_context: &ExtContext, rng: &mut R) -> String {
        self.gsubs
            .gsub(&self.options.generate(ext_context, rng))
            .to_string()
    }
}
impl<S: Substitutor> ProductionRule<S> {
    /// Create an empty `ProductionRule`.
    pub(super) fn new(options: TextOptions<S>, gsubs: S) -> Self {
        Self {
            options,
            gsubs: Rc::new(gsubs),
            binding_epoch: 0,
            weight: None,
        }
    }

    /// The weight of the texts.
    ///
    /// # Return
    /// The weight.
    ///
    /// # Note
    /// The return value is meaningless when the instance doesn't binds to a `Syntax`.
    fn weight(self: &Self) -> f64 {
        match self.weight {
            None => self.options.weight(),
            Some(x) => x,
        }
    }

    /// The number of the possible texts generated by the instance.
    ///
    /// # Return
    /// The the number of the possible texts generated by the instance.
    fn combination_number(self: &Self) -> usize {
        self.options.combination_number()
    }

    /// Set the weight of the production rule.
    ///
    /// # Parameter
    /// - `weight`: The weight of the production rule. The default value is used if `weight` is `None`. The default weight is the value propagated from the `TextOptions`.
    pub(super) fn set_weight(self: &mut Self, weight: Option<f64>) {
        self.weight = weight;
    }

    /// Equalize the chance to select each text.
    ///
    /// # Parameter
    /// - `enable`: equalized if `enable` is `true`. If not, the chance depends on the weight of the text. (Default)
    pub(super) fn equalize_chance(self: &mut Self, enable: bool) {
        self.options.equalize_chance(enable);
    }

    /// Bind the instance on a syntax.
    ///
    /// # Parameter
    /// - `assingments`: The assignments in the `Syntax` to be bound on.
    /// - `epoch`: The current binding epoch.
    /// - `err_msg`: The error messages are added if some errors are detected.
    ///
    /// # Error
    /// An error message is added to `err_msg` if this instance detects a recursive expansion.
    fn bind_syntax(
        self: &mut Self,
        assignments: &Assignments<S>,
        epoch: usize,
        err_msg: &mut Vec<String>,
    ) {
        // No need to check the recursion because RefCell detects it.
        if self.binding_epoch == epoch {
            // Already bound
            return;
        }

        self.options.bind_syntax(assignments, epoch, err_msg);
        self.binding_epoch = epoch;
    }

    /// Fix the reference to the local nonterminal.
    ///
    /// # Parameter
    /// - `syntax`: The syntax to be fixed.
    /// - `err_msg`: The error messages are added if some errors are detected.
    ///
    /// # Error
    /// An error is caused if the local nonterminal that is referred by a production rule doesn't exists.
    fn fix_local_nonterminal(self: &mut Self, syntax: &mut Syntax<S>, err_msg: &mut Vec<String>) {
        self.options.fix_local_nonterminal(syntax, err_msg);
    }

    /// Reset the binding epoch.
    fn reset_binding_epoch(self: &mut Self) {
        self.binding_epoch = 0;
    }
}

/// The data structure representing (a part of) the phrase syntax.
///
/// Add some phrase syntaxes to a `Syntax`, and add it to a `Generator` to use.
///
/// # Example
/// `parse()` and `parse_str()` can create a `Syntax`.
/// ```rust
/// let syntax = tphrase::parse_str(r#"main = Hello, World!"#).unwrap();
/// let mut ph: tphrase::Generator = tphrase::Generator::new();
/// let _ = ph.add(syntax);
/// assert_eq!(ph.generate(), "Hello, World!");
/// ```
///
/// You can easily create from a string to a phrase syntax.
/// ```rust
/// let syntax: tphrase::Syntax = "main = How are you?".parse().unwrap();
/// let mut ph: tphrase::Generator = tphrase::Generator::new();
/// let _ = ph.add(syntax);
/// assert_eq!(ph.generate(), "How are you?");
/// ```
///
/// You can add a part of a phrase syntax to a `Syntax`.
/// ```rust
/// let hello: tphrase::Syntax = "HELLO = Hello".parse().unwrap();
/// let world: tphrase::Syntax = "WORLD = World".parse().unwrap();
/// let mut syntax: tphrase::Syntax = "main = {HELLO}, {WORLD}!".parse().unwrap();
/// syntax.add(hello).unwrap();
/// syntax.add(world).unwrap();
/// let mut ph: tphrase::Generator = tphrase::Generator::new();
/// let _ = ph.add(syntax);
/// assert_eq!(ph.generate(), "Hello, World!");
/// ```
///
/// `Err` in the result holds some human readable error messages.
/// ```rust
/// let syntax_result: Result<tphrase::Syntax, _> = r#"
///     main = "Hello, " {WORLD}
///     WORLD
///       = world
/// "#.parse();
/// let err_msgs = syntax_result.err().unwrap();
/// assert_eq!(err_msgs.len(), 3);
/// assert_eq!(err_msgs[0], "Line#2, Column#22: The end of the text or \"\\n\" is expected.");
/// assert_eq!(err_msgs[1], "Line#3, Column#10: \"=\" or \":=\" is expected.");
/// assert_eq!(err_msgs[2], "Line#4, Column#7: A nonterminal \"[A-Za-z0-9_.]+\" is expected.");
/// ```
pub struct Syntax<S: Substitutor = crate::DefaultSubstitutor> {
    /// The assignments in the syntax.
    assignments: Assignments<S>,
    /// The reference to the start condition.
    start_rule: Option<Rc<RefCell<ProductionRule<S>>>>,
    /// The name of the start condition.
    start_condition: String,
    /// The binding epoch.
    binding_epoch: usize,
}
impl<S: Substitutor> Clone for Syntax<S> {
    fn clone(self: &Self) -> Self {
        let mut a = Self {
            assignments: Assignments::new(),
            start_rule: None,
            start_condition: self.start_condition.clone(),
            binding_epoch: 0,
        };
        for (k, v) in self.assignments.iter() {
            a.assignments
                .insert(k.clone(), Rc::new(RefCell::new(v.borrow().clone())));
        }
        if self.start_rule.is_some() {
            let _ = a.bind_syntax(&self.start_condition); // It should not generate any errors.
        }
        return a;
    }
}
impl<S: Substitutor> TextGenerator for Syntax<S> {
    fn generate<R: RandomGenerator>(self: &Self, ext_context: &ExtContext, rng: &mut R) -> String {
        if self.is_generatable() {
            self.start_rule
                .as_ref()
                .unwrap()
                .borrow()
                .generate(ext_context, rng)
        } else {
            "nil".to_string()
        }
    }
}
impl<S: Substitutor> std::str::FromStr for Syntax<S> {
    type Err = Vec<String>;
    /// `from_str(s)` is equivalent to `parse_str(s)`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::parse_str(s)
    }
}
impl<S: Substitutor> Syntax<S> {
    /// Create an empty phrase syntax.
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
            start_rule: None,
            start_condition: String::new(),
            binding_epoch: 0,
        }
    }

    /// The sum of the weight of the texts.
    ///
    /// # Return
    /// The sum of the weight.
    ///
    /// # Note
    /// The return value is meaningless if `is_generatable()` is `false`. (The return type isn't `Option` because the function isn't public for the crate user.)
    pub(crate) fn weight(self: &Self) -> f64 {
        if self.is_generatable() {
            self.start_rule.as_ref().unwrap().borrow().weight()
        } else {
            0.0
        }
    }

    /// The number of the possible texts generated by the instance.
    ///
    /// # Return
    /// The the number of the possible texts generated by the instance.
    ///
    /// # Note
    /// The return value is meaningless if `is_generatable()` is `false`. (The return type isn't `Option` because the function isn't public for the crate user.)
    pub(crate) fn combination_number(self: &Self) -> usize {
        if self.is_generatable() {
            self.start_rule
                .as_ref()
                .unwrap()
                .borrow()
                .combination_number()
        } else {
            0
        }
    }

    /// Is is a local nonterminal?
    ///
    /// # Parameter
    /// - `nonterminal`: The target nonterminal.
    ///
    /// # Return
    /// It's a local nonterminal.
    fn is_local_nonterminal(nonterminal: &str) -> bool {
        nonterminal.chars().next().is_some_and(|c| c == '_')
    }

    /// The production rule assigned to the nonterminal.
    ///
    /// # Parameter
    /// - `nonterminal`: The target nonterminal.
    ///
    /// # Return
    /// The production rule.
    fn production_rule(self: &Self, nonterminal: &str) -> Option<Rc<RefCell<ProductionRule<S>>>> {
        self.assignments.get(nonterminal).cloned()
    }

    /// Is the instance able to generate a phrase?
    ///
    /// # Return
    /// The instance is able to generate a phrase.
    ///
    /// # Note
    /// It means the instance has the production rule assigned to the start condition and is successfully bound.
    fn is_generatable(self: &Self) -> bool {
        self.start_rule.is_some()
    }

    /// Add a pair of a nonterminal and a production rule.
    ///
    /// # Return
    /// - `nonterminal`: The nonterminal.
    /// - `rule`: The production rule to be assigned to the nonterminal.
    ///
    /// # Return
    /// `Ok` if no errors are detected. `Err` has the human readable error message.
    /// # Note
    /// - It has a side effect to make the instance the unbound state (although the object that was bound on `self` remains bound on it).
    /// - If `self` already contains nonterminal, then (1) the nonterminal and the rule don't add to `self`, (2) `Err` is returned.
    pub(super) fn add_production_rule(
        self: &mut Self,
        nonterminal: &str,
        rule: ProductionRule<S>,
    ) -> Result<(), String> {
        self.disable_generating();
        let it = self.assignments.get(nonterminal);
        if it.is_none() {
            self.assignments
                .insert(nonterminal.to_string(), Rc::new(RefCell::new(rule)));
            Ok(())
        } else {
            let mut err = "The nonterminal \"".to_string();
            err += nonterminal;
            err += "\" is already defined.";
            Err(err)
        }
    }

    /// Add a set of the assignments.
    ///
    /// # Return
    /// - `syntax`: The syntax with the assignments to be added.
    ///
    /// # Return
    /// `Ok` if no errors are detected. `Err` has the human readable error message.
    ///
    /// # Note
    /// - If syntax has the nonterminal that this already contains, then: (1) the nonterminal in syntax OVERWRITES it, (2) `Err` is returned.
    /// - It has a side effect to make the instance the unbound state (although the crate user has no occasion to use the bound `Syntax` directly.)
    pub fn add(self: &mut Self, mut syntax: Syntax<S>) -> Result<(), Vec<String>> {
        self.disable_generating();
        let mut err_msg = Vec::new();
        for (k, v) in syntax.assignments.drain() {
            if self.assignments.get(&k).is_some() {
                let mut err = "The nonterminal \"".to_string();
                err += &k;
                err += "\" is already defined. Overwrited by newer.";
                err_msg.push(err);
            }
            self.assignments.insert(k, v);
        }
        if err_msg.is_empty() {
            return Ok(());
        } else {
            return Err(err_msg);
        }
    }

    /// Disable the instance for generating a phrase.
    fn disable_generating(self: &mut Self) {
        self.start_rule = None;
        self.start_condition.clear();
    }

    /// Try to bind the expansions on the nonterminals in this.
    ///
    /// # Parameter
    /// - `start_condition`: The nonterminal where is the start condition.
    ///
    /// # Return
    /// `Ok` if no errors are detected. `Err` has the human readable error messages.
    ///
    /// # Note
    /// - Only the nonterminals that are directly or indirectly referred by the start condition are tried binding.
    /// - An error is caused if the recursive reference to a nonterminal exists.
    /// - An error is cause if the nonterminal start_condition doesn't exist.
    pub(crate) fn bind_syntax(self: &mut Self, start_condition: &str) -> Result<(), Vec<String>> {
        self.disable_generating();
        let mut err_msg = Vec::new();
        if self.assignments.get(start_condition).is_none() {
            let mut err = "The nonterminal \"".to_string();
            err += start_condition;
            err += "\" doesn't exist.";
            err_msg.push(err);
            return Err(err_msg);
        }

        self.binding_epoch += 1;
        // It's generally 0 or 1 because the functions of struct Syntax and Generator don't call bind_syntax() to the syntax that already bound. (The three variations (initial, current, not current) are enough to distinguish the binding epoch unless start_condition is changed.)
        if self.binding_epoch == std::usize::MAX {
            for (_, v) in self.assignments.iter() {
                v.borrow_mut().reset_binding_epoch();
            }
            self.binding_epoch = 1;
        }

        let start_rule = Rc::clone(self.assignments.get(start_condition).as_ref().unwrap());
        start_rule.borrow_mut().bind_syntax(
            &mut self.assignments,
            self.binding_epoch,
            &mut err_msg,
        );
        let is_bound = err_msg.len() == 0;
        if is_bound {
            self.start_rule = Some(start_rule);
            self.start_condition = start_condition.to_string();
            return Ok(());
        } else {
            return Err(err_msg);
        }
    }

    /// Fix the reference to the local nonterminal.
    ///
    /// # Parameter
    /// - `err_msg`: The error messages are added if some errors are detected.
    ///
    /// # Error
    /// An error is caused if the local nonterminal that is referred by a production rule doesn't exists.
    pub(super) fn fix_local_nonterminal(self: &mut Self, err_msg: &mut Vec<String>) {
        let mut rc_v: Vec<Rc<RefCell<ProductionRule<S>>>> = Vec::new();
        for (_, v) in self.assignments.iter() {
            rc_v.push(Rc::clone(v));
        }
        for v in rc_v.iter() {
            v.borrow_mut().fix_local_nonterminal(self, err_msg);
        }
        self.assignments
            .retain(|k, _| !Self::is_local_nonterminal(k));
    }
}
