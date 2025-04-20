# TPhrase-rs, Translatable phrase generator for rust

This library is one of the translatable phrase generators. See the manual for details.

There are [Lua](https://github.com/oo13/translatable_phrase_generator) and [C++](https://github.com/oo13/tphrase) versions.

## Manual
Use `cargo doc` or read [Lua version](https://github.com/oo13/translatable_phrase_generator/blob/main/manual.md).

## What's the translatable phrase generator

1. The generator can generate a phrase (like a word, a name, a sentence, and a paragraph) with a syntax rule expressed by a text that can be replaced (=translated) in the run-time.
1. The syntax rule can express the phrase in many languages other than English.

For instance, [Context-free grammar of Wesnoth](https://wiki.wesnoth.org/Context-free_grammar) is a translatable phrase generator. [The phrase node in Endless Sky](https://github.com/endless-sky/endless-sky/wiki/CreatingPhrases) might be, except that it cannot replace the text of the syntax rule.

## Motive for Creating Another One
[Context-free grammar of Wesnoth](https://wiki.wesnoth.org/Context-free_grammar) can theoretically describe a phrase syntax rule in many languages, but a syntax rule that can be simply expressed a combination of words in English might not be expressed by a combination in other languages because they have inflection and so on; a syntax rule with three production rules have ten options in English may be translated into a syntax rule with one production rule have a thousand options in a language. It's hard to maintain a "combination explosion" like this. (In other hand, the expandable to all options in a single production rule means translatable. If a method doesn't allow the translator to specify all options, it may be impossible to translate potentially.)

Also a word in English cannot always translate into the same word of a language in the different sentence, and it should be translated a various word depended on the context. It causes a trouble similar to inflection.

As far as I had experienced about translating the phrase node in my translatable version of Endless Sky, the substitution is effective to handle the inflection and the like depended on the context, and the sophisticated substitution may make easy to handle the changing word order.

[Context-free grammar of Wesnoth](https://wiki.wesnoth.org/Context-free_grammar)  has no substituting functions. That's why I would create another phrase generator that wasn't compatible with it.

## Features (in comparison with Context-free grammar of Wesnoth)
- gsub (global substitution) function.
- Generator with a parameter to restrict the context.
- Equal chance to generate all possible texts by default, and the creator can change it.
- The comment blocks to maintain the translation easily.
- Don't care some white spaces for readability.

## Example of Phrase Syntax
### Avoid combination explosion
Japanese translation of Arach ship's name in Endless Sky (excerpt):
```
ARACH_START= マg | グラb | ブロg | ブロp | ブラb | モg | モb {* | ... }
ARACH_MIDDLE = aラg | aバg | グラg | グロp | aロp | プルーt {* | ... }
ARACH_NEXT = ・{ARACH_START}
ARACH_EMPTY = ""
main = {ARACH_START}{ARACH_MIDDLE}{=
         {ARACH_MIDDLE} | {ARACH_EMPTY} | {ARACH_NEXT}
       }{=
         {ARACH_MIDDLE} | {ARACH_EMPTY}
       } ~
       /ga/ガ/g ~
       /ba/バ/g ~
       /pa/パ/g ~
       /ta/タ/g ~
       /g/グ/g ~
       /b/ブ/g ~
       /p/プ/g ~
       /t/トゥ/g
```
Gsubs handle the characters that must be replaced by the combination with preceding and succeeding words.

### Inflection
```
ARTICLES = a | the | that | its
NOUNS = @apple | banana | @orange | @avocado | watermelon
main = {ARTICLES} {NOUNS} ~
       /a @/an / ~
       /@//
```

### Translate a single word into some different words
An example that an English word cannot translates into the same word in Japanese.
English version:
```
ECONOMICAL_SITUATION = poor | rich
main = You are {ECONOMICAL_SITUATION}. |
       You purchased a {ECONOMICAL_SITUATION} ship.
```

Japanese version:
```
ECONOMICAL_SITUATION = 貧乏 | 裕福
main = あなたは{ECONOMICAL_SITUATION}です。 |
       あなたは{ECONOMICAL_SITUATION}な船を購入しました。 ~
       /貧乏な船/粗末な船/ ~
       /裕福な船/豪華な船/
```
If you use a simple substitution instead of a translatable phrase generator, it cannot translate. For example: (gettext() is a translating function.)
```rust
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
```
The translator can create only two independent messages at most but Japanese translator need four independent messages.


### Alternative to printf
This phrase generator can play a role of printf by "external context":
```rust
    let s1 = if money < 10000 {
        gettext("poor")
    } else {
        gettext("rich")
    };
    let mut ph1: tphrase::Generator = gettext("main = You are {ECONOMICAL_SITUATION}.").parse().unwrap();
    let r1 = ph1.generate_with_context(&tphrase::ExtContext::from([
        ("ECONOMICAL_SITUATION".to_string(), s1),
    ]));
    // ...snip...
    let s2 = if cost < 10000 {
        gettext("poor")
    } else {
        gettext("rich")
    };
    let mut ph2: tphrase::Generator = gettext("main = You purchased a {ECONOMICAL_SITUATION} ship.").parse().unwrap();
    let r2 = ph2.generate_with_context(&tphrase::ExtContext::from([
        ("ECONOMICAL_SITUATION".to_string(), s2),
    ]));
```
If you use a format function, it might not be translatable. For example, this is not a translatable in Japanese by gettext:
```rust
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
```
because gettext merges same words into a single entry, so "poor" can have a single translated word, but it should be translate into two different words. (so GNU gettext has pgettext() function.)

In fact, this is the best way for the translation:
```rust
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
```
but it's not practical if the combination increases.

# License
TPhrase-rs is licensed under MIT or Apache-2.0.

Copyright © 2025 OOTA, Masato

This file is part of TPhrase-rs.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

OR

Licensed under the Apache License, Version 2.0 (the "License"); you may not use TPhrase-rs except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
