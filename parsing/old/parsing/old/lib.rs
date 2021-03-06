

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::collections::VecDeque;
use std::str::FromStr;
use std::fmt;
use regex::Regex;
use dic::Dic;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
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
        self.rule_vec
            .push (rule);
        self
    }
}

impl Grammar {
    pub fn find_rules (
        &self,
        name: &str,
    ) -> Vec <Rule> {
        let mut rules = Vec::new ();
        for rule in &self.rule_vec {
            if name == &rule.name {
                rules.push (rule.clone ());
            }
        }
        rules
    }
}

impl Grammar {
    pub fn apply (
        &self,
        tree: &LinearParseTree,
    ) -> Vec <LinearParseTree> {
        let mut result = Vec::new ();
        if let Some (name) = tree.first_var_name () {
            for rule in self.find_rules (&name) {
                result.push (rule.apply (&tree))
            }
        }
        result
    }
}

impl ToString for Grammar {
    fn to_string (&self) -> String {
        let mut string = String::new ();
        for rule in &self.rule_vec {
            string += &rule.to_string ();
            string += "\n";
        }
        string
    }
}

#[derive (Debug)]
pub struct ParseError;

fn rule_from_str (
    mut string: &str
) -> Result <Rule, ParseError> {
    let re = Regex::new (r"^\s*(\S+):(\S+)\s*=\s*") .unwrap ();
    if let Some (caps) = re.captures (string) {
        let mut rule = Rule::new (&caps[1], &caps[2]);
        let mat = caps.get (0) .unwrap ();
        string = &string [mat.end () ..];
        loop {
            if string .trim () .is_empty () {
                return Ok (rule);
            } else {
                let (token, next) = next_token_from_str (string)?;
                rule.token_vec.push (token);
                string = next;
            }
        }
    } else {
        Err (ParseError)
    }
}

impl FromStr for Grammar {
    type Err = ParseError;

    fn from_str (
        string: &str
    ) -> Result <Self, Self::Err> {
        let mut rule_vec = Vec::new ();
        for s in string.split (";") {
            if ! s .trim () .is_empty () {
                rule_vec.push (rule_from_str (s)?);
            }
        }
        Ok (Grammar { rule_vec })
    }
}

#[derive (Clone)]
#[derive (PartialEq, Eq, Hash)]
pub struct Rule {
    name: String,
    choice: String,
    token_vec: Vec <Token>,
}

impl Rule {
    pub fn new (
        name: &str,
        choice: &str,
    ) -> Self {
        Rule {
            name: name.to_string (),
            choice: choice.to_string (),
            token_vec: Vec::new (),
        }
    }
}

impl Rule {
    pub fn var (mut self, name: &str) -> Self {
        self.token_vec
            .push (Token::new_var (name));
        self
    }
}

impl Rule {
    pub fn string (mut self, string: &str) -> Self {
        self.token_vec
            .push (Token::new_string (string));
        self
    }
}

impl Rule {
    pub fn bra (mut self, name: &str, choice: &str) -> Self {
        self.token_vec
            .push (Token::new_bra (name, choice));
        self
    }
}

impl Rule {
    pub fn ket (mut self, name: &str, choice: &str) -> Self {
        self.token_vec
            .push (Token::new_ket (name, choice));
        self
    }
}

impl Rule {
    fn find_var (
        &self,
        tree: &LinearParseTree,
    ) -> Option <(Vec <Token>, Vec <Token>)>{
        let mut token_vec = tree.token_vec.clone ();
        let mut index: Option <usize> = None;
        for (i, token) in token_vec .iter () .enumerate () {
            if let Token::Var { name } = token {
                if &self.name == name {
                    index = Some (i);
                }
            }
        }
        if let Some (i) = index {
            let succ = token_vec.split_off (i+1);
            token_vec.pop ();
            let ante = token_vec;
            Some ((ante, succ))
        } else {
            None
        }
    }

    fn apply (
        &self,
        tree: &LinearParseTree,
    ) -> LinearParseTree {
        let mut token_vec = Vec::new ();
        if let Some ((mut ante, mut succ)) = self.find_var (tree) {
            let mut rhs = self.token_vec.clone ();
            let bra = Token::new_bra (&self.name, &self.choice);
            let ket = Token::new_ket (&self.name, &self.choice);
            token_vec.append (&mut ante);
            token_vec.push (bra);
            token_vec.append (&mut rhs);
            token_vec.push (ket);
            token_vec.append (&mut succ);
            LinearParseTree { token_vec }
        } else {
            panic! ("Rule::apply failed");
        }
    }
}

impl ToString for Rule {
    fn to_string (&self) -> String {
        let mut string = format! (
            "{}:{} = ", self.name, self.choice);
        for token in &self.token_vec {
            string += &token.to_string ();
            string += " ";
        }
        string.pop ();
        string += ";";
        string
    }
}

impl fmt::Debug for Rule {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "{}", self.to_string ())
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub enum Token {
    Var { name: String },
    Str { string: String },
    Bra { name: String, choice: String },
    Ket { name: String, choice: String },
}

impl Token {
    pub fn new_var (name: &str) -> Token {
        Token::Var { name: String::from (name) }
    }
}

impl Token {
    pub fn new_string (string: &str) -> Token {
        Token::Str { string: String::from (string) }
    }
}

impl Token {
    pub fn new_bra (name: &str, choice: &str) -> Token {
        Token::Bra {
            name: String::from (name),
            choice: String::from (choice),
        }
    }
}

impl Token {
    pub fn new_ket (name: &str, choice: &str) -> Token {
        Token::Ket {
            name: String::from (name),
            choice: String::from (choice),
        }
    }
}

impl ToString for Token {
    fn to_string (&self) -> String {
        match self {
           Token::Var { name } => {
              format! ("{}", name)
           }
           Token::Str { string } => {
              format! ("\"{}\"", string)
           }
           Token::Bra { name, choice } => {
              format! ("{}:{}", name, choice)
           }
           Token::Ket { name, choice } => {
              format! ("/{}:{}", name, choice)
           }
        }
    }
}

impl Token {
    fn to_word (&self) -> String {
        match self {
           Token::Var { name } => {
              format! ("<{}>", name)
           }
           Token::Str { string } => {
              format! ("{}", string)
           }
           Token::Bra { .. } => {
              format! ("")
           }
           Token::Ket { .. } => {
              format! ("")
           }
        }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
pub struct ParseTree {
    head: Rule,
    body: Vec <ParseTree>,
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct LinearParseTree {
    pub token_vec: Vec <Token>,
}

impl LinearParseTree {
    pub fn unit_var (name: &str) -> Self {
        LinearParseTree {
            token_vec: vec! [Token::new_var (name)],
        }
    }
}

impl LinearParseTree {
    pub fn complete_p (&self) -> bool {
        for token in &self.token_vec {
            if let Token::Var { .. } = token {
                return false;
            }
        }
        true
    }
}

impl LinearParseTree {
    pub fn first_var_name (&self) -> Option <String> {
        for token in &self.token_vec {
            if let Token::Var { name } = token {
                return Some (name.to_string ())
            }
        }
        None
    }
}

impl ToString for LinearParseTree {
    fn to_string (&self) -> String {
        let mut string = String::new ();
        for token in &self.token_vec {
            string += &token.to_string ();
        }
        string
    }
}

impl LinearParseTree {
    pub fn to_sentence (&self) -> String {
        let mut string = String::new ();
        for token in &self.token_vec {
            string += &token.to_word ();
        }
        string
    }
}

fn next_token_from_str (
    string: &str
) -> Result <(Token, &str), ParseError> {
    // the order matters
    let re = Regex::new (r"^\s*/(\S+):(\S+)\s*") .unwrap ();
    if let Some (caps) = re.captures (string) {
        let token = Token::new_ket (&caps[1], &caps[2]);
        let mat = caps.get (0) .unwrap ();
        return Ok ((token, &string [mat.end () ..]));
    }
    let re = Regex::new (r"^\s*(\S+):(\S+)\s*") .unwrap ();
    if let Some (caps) = re.captures (string) {
        let token = Token::new_bra (&caps[1], &caps[2]);
        let mat = caps.get (0) .unwrap ();
        return Ok ((token, &string [mat.end () ..]));
    }
    let re = Regex::new (r#"^\s*"(.*?)"\s*"#) .unwrap ();
    if let Some (caps) = re.captures (string) {
        let token = Token::new_string (&caps[1]);
        let mat = caps.get (0) .unwrap ();
        return Ok ((token, &string [mat.end () ..]));
    }
    let re = Regex::new (r"^\s*(\S+)\s*") .unwrap ();
    if let Some (caps) = re.captures (string) {
        let token = Token::new_var (&caps[1]);
        let mat = caps.get (0) .unwrap ();
        return Ok ((token, &string [mat.end () ..]));
    }
    Err (ParseError)
}

impl FromStr for LinearParseTree {
    type Err = ParseError;

    fn from_str (
        mut string: &str
    ) -> Result <Self, Self::Err> {
        let mut token_vec = Vec::new ();
        while ! string .trim () .is_empty () {
            let (token, next) = next_token_from_str (string)?;
            token_vec.push (token);
            string = next;
        }
        Ok (LinearParseTree { token_vec })
    }
}

pub struct Generation {
    grammar: Grammar,
    queue: VecDeque <LinearParseTree>,
}

impl Generation {
    pub fn new (grammar: Grammar, name: &str) -> Self {
        let mut queue = VecDeque::new ();
        let init = LinearParseTree::unit_var (name);
        queue.push_back (init);
        Generation { grammar, queue }
    }
}

#[derive (Debug)]
enum StepResult {
    Tree (LinearParseTree),
    Continue,
    Finished,
}

impl Generation {
    fn step (&mut self) -> StepResult {
        if let Some (tree) = self.queue.pop_front () {
            if tree.complete_p () {
                StepResult::Tree (tree)
            } else {
                for new_tree in self.grammar.apply (&tree) {
                    self.queue.push_back (new_tree)
                }
                StepResult::Continue
            }
        } else {
            StepResult::Finished
        }
    }
}

impl Generation {
    pub fn next_tree (
        &mut self
    ) -> Option <LinearParseTree> {
        loop {
            match self.step () {
                StepResult::Tree (tree) => {
                    return Some (tree)
                }
                StepResult::Continue => { }
                StepResult::Finished => {
                    return None
                }
            }
        }
    }
}

#[derive (Clone)]
#[derive (PartialEq, Eq, Hash)]
pub struct Frame {
    index: usize,
    rule_vec: Vec <Rule>,
    back_cursor: usize,
    back_token_stack: Vec <Token>,
}

impl fmt::Debug for Frame {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (
            f, "
            - frame :
              index = {}
              rule_vec = {:?}
              back_cursor = {}
              back_token_stack = {:?}",
            self.index,
            self.rule_vec,
            self.back_cursor,
            self.back_token_stack)
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq, Hash)]
pub struct Parsing {
    grammar: Grammar,
    input: String, cursor: usize,
    token_stack: Vec <Token>,
    frame_stack: Vec <Frame>,
}

impl Parsing {
    fn new (
        grammar: Grammar,
        name: &str,
        input: String,
    ) -> Self {
        Parsing {
            grammar,
            input, cursor: 0,
            token_stack: vec! [ Token::new_var (name) ],
            frame_stack: Vec::new (),
        }
    }
}

impl Parsing {
    fn next_tree (&mut self) -> Option <LinearParseTree> { loop {
        let remain = &self.input [self.cursor ..];
        // println! ("- remain = {}", remain);
        if let Some (token) = self.token_stack.pop () {
            match token {
                Token::Var { name } => {
                    self.frame_stack.push (Frame {
                        index: 0,
                        rule_vec: self.grammar.find_rules (&name),
                        back_cursor: self.cursor,
                        back_token_stack:
                        self.token_stack.clone (),
                    });
                    if self.next_choice () .is_none () {
                        return None;
                    }
                }
                Token::Str { string } => {
                    if remain.starts_with (&string) {
                        self.cursor += string.len ();
                    } else {
                        if self.next_choice () .is_none () {
                            return None;
                        }
                    }
                }
                _ => {
                    panic! ("Parsing::next_tree failed");
                }
            }
        } else if remain.is_empty () {
            let tree = self.collect_tree ();
            self.next_choice ();
            return Some (tree);
        } else {
            if self.next_choice () .is_none () {
                return None;
            }
        }
    }}
}

impl Parsing {
    fn next_choice (&mut self) -> Option <()> { loop {
        if let Some (mut frame) = self.frame_stack.pop () {
            if frame.index < frame.rule_vec.len () {
                let rule = frame.rule_vec [frame.index] .clone ();
                frame.index += 1;
                self.cursor = frame.back_cursor;
                self.token_stack = frame.back_token_stack.clone ();
                self.frame_stack.push (frame);
                for token in rule.token_vec.into_iter () .rev () {
                    self.token_stack.push (token);
                }
                return Some (());
            }
        } else {
            return None;
        }
    }}
}

impl Parsing {
    fn collect_tree (&mut self) -> LinearParseTree {
        let mut rule_tree_stack: Vec <RuleTree> = Vec::new ();
        for frame in self.frame_stack .iter () .rev () {
            let rule = &frame.rule_vec [frame.index - 1];
            let arity = rule.arity ();
            let mut body = Vec::new ();
            for _ in 0..arity {
                body.push (rule_tree_stack.pop () .unwrap ());
            }
            rule_tree_stack.push (RuleTree {
                rule: rule.clone (),
                body: body.into_iter () .rev () .collect (),
                // why a rev here ?
            });
        }
        let rule_tree = rule_tree_stack.pop () .unwrap ();
        assert! (rule_tree_stack.is_empty ());
        let token_vec = rule_tree_to_token_vec (rule_tree);
        LinearParseTree { token_vec }
    }
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq, Eq)]
struct RuleTree {
    rule: Rule,
    body: Vec <RuleTree> ,
}

impl Rule {
    fn arity (&self) -> usize {
        let mut n = 0;
        for token in &self.token_vec {
            if let Token::Var { .. } = token {
                n += 1;
            }
        }
        n
    }
}

fn rule_tree_to_token_vec (
    mut rule_tree: RuleTree
) -> Vec <Token> {
    let mut token_vec = Vec::new ();
    let name = &rule_tree.rule.name;
    let choice = &rule_tree.rule.choice;
    token_vec.push (Token::new_bra (name, choice));
    for token in &rule_tree.rule.token_vec {
        match token {
            Token::Var { .. } => {
                let sub_tree = rule_tree.body.pop () .unwrap ();
                for token in rule_tree_to_token_vec (sub_tree) {
                    token_vec.push (token);
                }
            }
            _ => {
                token_vec.push (token.clone ());
            }
        }
    }
    token_vec.push (Token::new_ket (name, choice));
    token_vec
}

impl Grammar {
    fn parsing (&self, name: &str, input: &str) -> Parsing {
        Parsing::new (
            self.clone (),
            name,
            input.to_string ())
    }
}

#[cfg(test)]
mod example_grammar {
    use super::*;

    pub fn tom_dick_and_harry () -> Grammar {
        // grammar! {
        //     Sentence::Name = Name;
        //     Sentence::List = List " and " Name;
        //     Name::Tom = "tom";
        //     Name::Dick = "dick";
        //     Name::Harry = "harry";
        //     List::Car = Name;
        //     List::Cdr = Name ", " List;
        // }
        Grammar::new ()
            .rule (Rule::new ("Sentence", "Name")
                   .var ("Name"))
            .rule (Rule::new ("Sentence", "List")
                   .var ("List")
                   .string (" and ")
                   .var ("Name"))
            .rule (Rule::new ("Name", "Tom")
                   .string ("tom"))
            .rule (Rule::new ("Name", "Dick")
                   .string ("dick"))
            .rule (Rule::new ("Name", "Harry")
                   .string ("harry"))
            .rule (Rule::new ("List", "Head")
                   .var ("Name"))
            .rule (Rule::new ("List", "Tail")
                   .var ("Name")
                   .string (", ")
                   .var ("List"))
    }

    pub fn tdh () -> Grammar {
        Grammar::new ()
            .rule (Rule::new ("S", "N")
                   .var ("N"))
            .rule (Rule::new ("S", "L")
                   .var ("L")
                   .string ("&")
                   .var ("N"))
            .rule (Rule::new ("N", "T")
                   .string ("t"))
            .rule (Rule::new ("N", "D")
                   .string ("d"))
            .rule (Rule::new ("N", "H")
                   .string ("h"))
            .rule (Rule::new ("L", "H")
                   .var ("N"))
            .rule (Rule::new ("L", "T")
                   .var ("N")
                   .string ("")
                   .var ("L"))
    }
}

#[test]
fn test_grammar_to_string () {
    use example_grammar::*;
    assert! (tom_dick_and_harry () .to_string ()
             .starts_with ("Sentence:Name = Name;"));
    assert! (tdh () .to_string ()
             .starts_with ("S:N = N;"));
}

#[test]
fn test_generation () {
    use example_grammar::*;
    let mut gen = Generation::new (
        tom_dick_and_harry (), "Sentence");
    for _ in 0..10 {
         assert! (gen.next_tree () .is_some ());
        // let tree = gen.next_tree () .unwrap ();
        // println! ("- tree : {}", tree.to_string ());
        // println! ("- sentence : {}", tree.to_sentence ());
    }

    let mut gen = Generation::new (
        tdh (), "S");
    for _ in 0..10 {
        assert! (gen.next_tree () .is_some ());
        // let tree = gen.next_tree () .unwrap ();
        // println! ("- tree : {}", tree.to_string ());
        // println! ("- sentence : {}", tree.to_sentence ());
    }
}

#[test]
fn test_linear_parse_tree_from_str () {
   let string = r#"
   sum:sum
       sum:sum
           sum:digit
               digit:3 "3" /digit:3
           /sum:digit
           " + "
           sum:digit
               digit:5 "5" /digit:5
           /sum:digit
       /sum:sum
       " + "
       sum:digit
           digit:1 "1" /digit:1
       /sum:digit
   /sum:sum
   "#;
   let tree = LinearParseTree::from_str (string) .unwrap ();
   assert_eq! ("3 + 5 + 1", tree.to_sentence ());
}

#[test]
fn test_grammar_from_str () {
    let string = r#"
    tom-dick-and-harry:name = name;
    tom-dick-and-harry:list = list " and " name;

    name:tom = "tom";
    name:dick = "dick";
    name:harry = "harry";

    list:unit = name;
    list:cons = name ", " list;
    "#;
    assert! (Grammar::from_str (string) .is_ok ());
}

mod example_format {
    pub const SEXP: &'static str = r#"
    sexp:list = "(" sexp-list ")";
    sexp:bool = bool;

    sexp-list:unit = sexp;
    sexp-list:cons = sexp " " sexp-list;

    bool:true = "true";
    bool:false = "false";
    "#;

    pub const TOM_DICK_AND_HARRY: &'static str = r#"
    tom-dick-and-harry:name = name;
    tom-dick-and-harry:list = list " and " name;

    name:tom = "tom";
    name:dick = "dick";
    name:harry = "harry";

    list:unit = name;
    list:cons = name ", " list;
    "#;

    pub const TDH: &'static str = r#"
    tdh:t = "t";
    tdh:d = "d";
    tdh:h = "h";
    tdh:list = list;

    list:t = "t" list-tail;
    list:d = "d" list-tail;
    list:h = "h" list-tail;

    list-tail:list = "," list;
    list-tail:t = "&t";
    list-tail:d = "&d";
    list-tail:h = "&h";
    "#;

    pub const TDH_LEFT: &'static str = r#"
    tdh-left:t = "t";
    tdh-left:d = "d";
    tdh-left:h = "h";
    tdh-left:list = list;

    list:t =  list-head "&t";
    list:d =  list-head "&d";
    list:h =  list-head "&h";

    list-head:t = "t";
    list-head:d = "d";
    list-head:h = "h";
    list-head:before-t = list-head ",t";
    list-head:before-d = list-head ",d";
    list-head:before-h = list-head ",h";
    "#;

    pub const SUM: &'static str = r#"
    sum:digit = digit;
    sum:sum = sum " + " sum;

    digit:0 = "0";
    digit:1 = "1";
    "#;

    pub const AB: &'static str = r#"
    ab:head-a = "a" b;
    ab:head-b = "b" a;

    a:one = "a";
    a:more = "a" s;
    a:after-b = "b" a a;

    b:one = "b";
    b:more = "b" s;
    b:after-a = "a" b b;
    "#;

    pub const ABC: &'static str = r#"
    abc:a-bc = a bc;
    abc:ab-c = ab c;

    a:one = "a";
    a:more = "a" a;

    bc:one = "bc";
    bc:more = "b" bc "c";

    ab:one = "ab";
    ab:more = "a" ab "b";

    c:one = "c";
    c:more = "c" c;
    "#;
}

#[cfg(test)]
fn assert_parse_sentence (
    format: &str,
    start: &str,
    sentence: &str,
) {
    let grammar = Grammar::from_str (format) .unwrap ();
    let mut parsing = grammar.parsing (start, sentence);
    let tree = parsing.next_tree () .unwrap ();
    assert_eq! (sentence, tree.to_sentence ());
}

#[cfg(test)]
fn assert_parse_none (
    format: &str,
    start: &str,
    sentence: &str,
) {
    let grammar = Grammar::from_str (format) .unwrap ();
    let mut parsing = grammar.parsing (start, sentence);
    let tree = parsing.next_tree ();
    assert! (tree .is_none ());
}

#[cfg(test)]
fn print_all_trees (
    format: &str,
    start: &str,
    sentence: &str,
) {
    let grammar = Grammar::from_str (format) .unwrap ();
    let mut parsing = grammar.parsing (start, sentence);
    while let Some (tree) = parsing.next_tree () {
        println! ("- tree = {}", tree.to_string ());
    }
}

#[test]
fn test_parsing () {
    use example_format::*;

    assert_parse_sentence (
        TOM_DICK_AND_HARRY,
        "tom-dick-and-harry",
        "tom, tom, dick, tom, dick and harry");

    assert_parse_sentence (
        TOM_DICK_AND_HARRY,
        "tom-dick-and-harry",
        "tom");

    assert_parse_sentence (
        TOM_DICK_AND_HARRY,
        "tom-dick-and-harry",
        "tom and tom");

    assert_parse_none (
        TOM_DICK_AND_HARRY,
        "tom-dick-and-harry",
        "ted");

    assert_parse_none (
        TOM_DICK_AND_HARRY,
        "tom-dick-and-harry",
        "tom, tom and jerry");

    assert_parse_sentence (
        SEXP,
        "sexp",
        "(((((((((true))) ((((false)))) ((((true))))))))))");

    assert_parse_none (
        SEXP,
        "sexp",
        "(((((((((true))) ((((false)))) ((((true)))))))))) )");

    assert_parse_sentence (
        TDH,
        "tdh",
        "t,d&h");

    // assert_parse_sentence (
    //     TDH_LEFT,
    //     "tdh-left",
    //     "t,d&h");

    assert_parse_sentence (
        SUM,
        "sum",
        "0 + 1 + 0");

    // print_all_trees (
    //     SUM,
    //     "sum",
    //     "0 + 1 + 0");

    assert_parse_sentence (
        SUM,
        "sum",
        "0 + 1 + 0 + 1 + 0 + 1 + 0");

    assert_parse_sentence (
        AB,
        "ab",
        "aaabbb");

    assert_parse_sentence (
        AB,
        "ab",
        "ab");

    assert_parse_sentence (
        ABC,
        "abc",
        "abbbccc");

    assert_parse_sentence (
        ABC,
        "abc",
        "aaabbbccccccc");

    assert_parse_none (
        ABC,
        "abc",
        "abbccc");

    assert_parse_sentence (
        ABC,
        "abc",
        "abc");

    assert_parse_sentence (
        ABC,
        "abc",
        "aaabbbccc");

    // print_all_trees (
    //     ABC,
    //     "abc",
    //     "abc");

    // print_all_trees (
    //     ABC,
    //     "abc",
    //     "aaabbbccc");

    let aaa = r#"
    aaa:1 = "a";
    aaa:2 = "a";
    aaa:3 = "a";
    "#;

    assert_parse_sentence (aaa, "aaa", "a");
    // print_all_trees (aaa, "aaa", "a");
}

#[test]
fn play_regex () {
    let re = Regex::new (r"^(([tdh],)*[tdh]&)?[tdh]") .unwrap ();
    assert! (re.find ("  t,t&t") .is_none ());
    let mat = re.find ("t") .unwrap ();
    assert_eq! (0, mat.start ());
    assert_eq! (1, mat.end ());
    let mat = re.find ("t,t&t") .unwrap ();
    assert_eq! (0, mat.start ());
    assert_eq! (5, mat.end ());

    // let re = Regex::new (r"^(x*)(y*)(z*)") .unwrap ();
    // let caps = re.captures ("xyyzzz") .unwrap ();
    // println! ("- caps : {:?}", caps);
    // println! ("{:?}\n{:?}\n{:?}\n{:?}\n",
    //           caps.get (0),
    //           caps.get (1),
    //           caps.get (2),
    //           caps.get (3));
    // println! ("{} {} {} {}",
    //           &caps [0],
    //           &caps [1],
    //           &caps [2],
    //           &caps [3]);

    // let re = Regex::new (r"^\s*(\S+):(\S+)\s*") .unwrap ();
    // let caps = re.captures (" sexp:null ") .unwrap ();
    // println! ("- caps : {:?}", caps);
    // println! ("{:?}\n{:?}\n{:?}\n",
    //           caps.get (0),
    //           caps.get (1),
    //           caps.get (2));
    // println! ("{} {} {}",
    //           &caps [0],
    //           &caps [1],
    //           &caps [2]);

    // let re = Regex::new (r"^\s*/(\S+):(\S+)\s*") .unwrap ();
    // let caps = re.captures (r" /sexp:null ") .unwrap ();
    // println! ("- caps : {:?}", caps);
    // println! ("{:?}\n{:?}\n{:?}\n",
    //           caps.get (0),
    //           caps.get (1),
    //           caps.get (2));
    // println! ("{} {} {}",
    //           &caps [0],
    //           &caps [1],
    //           &caps [2]);

    // let re = Regex::new (r#"^\s*"(.*?)"\s*"#) .unwrap ();
    // let caps = re.captures (r#" "asdf" "asd" "#) .unwrap ();
    // println! ("- caps : {:?}", caps);
    // println! ("{:?}\n{:?}\n",
    //           caps.get (0),
    //           caps.get (1));
    // println! ("{} {}",
    //           &caps [0],
    //           &caps [1]);
}
