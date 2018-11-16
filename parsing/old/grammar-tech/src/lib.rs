#![feature(uniform_paths)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Grammar {
    rule_vec: Vec <Rule>,
}

impl Grammar {
    pub fn new () -> Self {
        let rule_vec: Vec <Rule> = Vec::new ();
        Grammar { rule_vec }
    }
}

impl Grammar {
    pub fn rule (mut self, rule: Rule) -> Self {
        self.rule_vec.push (rule);
        self
    }
}

impl Grammar {
    fn apply (
        &self,
        form: &SententialForm,
    ) -> Vec <SententialForm> {
        let mut result: Vec <SententialForm> = Vec::new ();
        for rule in self.rule_vec.iter () {
            let mut form_vec = rule.apply (form);
            result.append (&mut form_vec);
        }
        result
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub struct Rule {
    pub ante: Vec <Token>,
    pub succ: Vec <Vec <Token>>,
}

impl Rule {
    pub fn new () -> Self {
        Rule {
            ante: Vec::new (),
            succ: Vec::new (),
        }
    }
}

impl Rule {
    pub fn choice (mut self) -> Self {
        self.succ.push (Vec::new ());
        self
    }
}

impl Rule {
    pub fn var (mut self, name: &str) -> Self {
        if self.succ.is_empty () {
            self.ante.push (Token::new_var (name));
        } else {
            let mut symbol_vec = self.succ.pop () .unwrap ();
            symbol_vec.push (Token::new_var (name));
            self.succ.push (symbol_vec);
        }
        self
    }
}

impl Rule {
    pub fn chars (mut self, string: &str) -> Self {
        if self.succ.is_empty () {
            let mut symbol_vec = Token::new_chars (string);
            self.ante.append (&mut symbol_vec);
        } else {
            let mut symbol_vec = self.succ.pop () .unwrap ();
            symbol_vec.append (&mut Token::new_chars (string));
            self.succ.push (symbol_vec);
        }
        self
    }
}

fn symbol_vec_eq (
    ante: &[Token],
    succ: &[Token],
) -> bool {
    ante == succ
}

impl Rule {
    fn apply (
        &self,
        form: &SententialForm,
    ) -> Vec <SententialForm> {
        let mut result: Vec <SententialForm> = Vec::new ();
        let len = self.ante.len ();
        if len <= form.len () {
            let offset = form.len () - len;
            for i in 0..offset+1 {
                if symbol_vec_eq (
                    &self.ante,
                    &form.symbol_vec [i..i+len])
                {
                    for symbol_vec in &self.succ {
                        let new_form = form.substitute (
                            i, len,
                            symbol_vec);
                        result.push (new_form);
                    }
                }
            }
        }
        result
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Token {
    Var (String),
    Char  (char),
}

impl Token {
    pub fn var_p (&self) -> bool {
        if let Token::Var (_) = self {
            true
        } else {
            false
        }
    }

    pub fn str_p (&self) -> bool {
        if let Token::Char  (_) = self {
            true
        } else {
            false
        }
    }
}

impl Token {
    pub fn new_var (name: &str) -> Token {
        Token::Var (String::from (name))
    }
}

impl Token {
    pub fn new_chars (string: &str) -> Vec <Token> {
        let mut chars: Vec <Token> = Vec::new ();
        for c in string.chars () {
            chars.push (Token::Char (c))
        }
        chars
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
struct SententialForm {
    symbol_vec: Vec <Token>,
}

impl SententialForm {
    fn str_p (&self) -> bool {
        self.symbol_vec
            .iter ()
            .all (|symbol| symbol.str_p ())
    }
}

impl SententialForm {
    fn len (&self) -> usize {
        self.symbol_vec.len ()
    }
}

impl SententialForm {
    fn new (symbol_vec: &Vec <Token>) -> Self {
        let symbol_vec = symbol_vec.clone ();
        SententialForm { symbol_vec }
    }
}

impl SententialForm {
    fn substitute (
        &self,
        i: usize, len: usize,
        symbol_vec: &Vec <Token>,
    ) -> Self {
        let mut new_symbol_vec: Vec <Token> = Vec::new ();
        for symbol in &self.symbol_vec [0..i] {
            new_symbol_vec.push (symbol.clone ());
        }
        for symbol in symbol_vec {
            new_symbol_vec.push (symbol.clone ());
        }
        for symbol in &self.symbol_vec [i+len..] {
            new_symbol_vec.push (symbol.clone ());
        }
        SententialForm::new (&new_symbol_vec)
    }
}

impl Into <String> for SententialForm {
    fn into (self) -> String {
        let mut result = String::new ();
        for symbol in self.symbol_vec {
            match symbol {
                Token::Var (name) => {
                    let repr = format! ("{{{}}}", name);
                    result.push_str (&repr);
                }
                Token::Char  (c) => {
                    result.push (c);
                }
            }
        }
        result
    }
}

pub struct Generation {
    statistics: GenerationStatistics,
    grammar: Grammar,
    sentential_form_queue: VecDeque <SententialForm>,
}

impl Generation {
    pub fn new (grammar: Grammar, start: &str) -> Self {
        let statistics = GenerationStatistics {
            sentence_counter: 0,
            step_counter: 0,
            match_counter: 0,
        };
        let mut sentential_form_queue = VecDeque::new ();
        sentential_form_queue.push_back (
            SententialForm::new (
                &vec! [Token::new_var (start)]));
        Generation {
            statistics, grammar,
            sentential_form_queue,
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
struct GenerationStatistics {
    sentence_counter: usize,
    step_counter: usize,
    match_counter: usize,
}

impl Generation {
    pub fn finished_p (&self) -> bool {
        self.sentential_form_queue.is_empty ()
    }
}

pub enum StepResult {
    Sentence (String),
    NonSentence,
    Finished,
}

impl Generation {
    pub fn step (&mut self) -> StepResult {
        // print! (".");
        // if self.statistics.step_counter > 1000000 {
        //     // how to make this better ?
        //     panic! ("><");
        // }
        let first = self.sentential_form_queue.pop_front ();
        if let Some (form) = first {
            self.statistics.step_counter += 1;
            if form.str_p () {
                self.statistics.sentence_counter += 1;
                let sentence = form.into ();
                StepResult::Sentence (sentence)
            } else {
                for new_form in self.grammar.apply (&form) {
                    self.statistics.match_counter += 1;
                    self.sentential_form_queue
                        .push_back (new_form)
                }
                StepResult::NonSentence
            }
        } else {
            StepResult::Finished
        }
    }
}

impl Generation {
    pub fn next_sentence (&mut self) -> Option <String> {
        loop {
            match self.step () {
                StepResult::Sentence (sentence) => {
                    return Some (sentence);
                }
                StepResult::NonSentence => {
                    // go on
                }
                StepResult::Finished => {
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod generation_test {
    use super::*;

    fn example_grammar () -> Grammar {
        // grammar! {
        //     Start -> Sentence;
        //     Sentence -> Name | List and Name;
        //     Name -> tom | dick | harry;
        //     List -> Name | Name, List;
        // }
        Grammar::new ()
            .rule (Rule::new ()
                   .var ("Start")
                   .choice () .var ("Sentence"))
            .rule (Rule::new ()
                   .var ("Sentence")
                   .choice () .var ("Name")
                   .choice ()
                       .var ("List")
                       .chars (" and ")
                       .var ("Name"))
            .rule (Rule::new ()
                   .var ("Name")
                   .choice () .chars ("tom")
                   .choice () .chars ("dick")
                   .choice () .chars ("harry"))
            .rule (Rule::new ()
                   .var ("List")
                   .choice () .var ("Name")
                   .choice ()
                       .var ("Name")
                       .chars (", ")
                       .var ("List"))
    }

    #[test]
    fn test_next_sentence () {
        let mut generation = Generation::new (
            example_grammar (), "Start");
        for _ in 0..1000 {
            let sentence = generation .next_sentence ();
            assert! (sentence.is_some ());
        }
    }
}
