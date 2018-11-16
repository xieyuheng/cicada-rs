use std::str::FromStr;

use parsing_tech::{
    Grammar, Generation,
};

const SEXP: &'static str = r#"
sexp:list = "(" sexp-list ")";
sexp:bool = bool;

sexp-list:unit = sexp;
sexp-list:cons = sexp " " sexp-list;

bool:true = "true";
bool:false = "false";
"#;

const TOM_DICK_AND_HARRY: &'static str = r#"
tom-dick-and-harry:name = name;
tom-dick-and-harry:list = list " and " name;

name:tom = "tom";
name:dick = "dick";
name:harry = "harry";

list:unit = name;
list:cons = name ", " list;
"#;

const TDH: &'static str = r#"
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

const TDH_LEFT: &'static str = r#"
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

const SUM: &'static str = r#"
sum:digit = digit;
sum:sum = sum " + " sum;

digit:0 = "0";
digit:1 = "1";
"#;

const AB: &'static str = r#"
ab:head-a = "a" b;
ab:head-b = "b" a;

a:one = "a";
a:more = "a" s;
a:after-b = "b" a a;

b:one = "b";
b:more = "b" s;
b:after-a = "a" b b;
"#;

const ABC: &'static str = r#"
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

fn sentences (n: usize, format: &str, start: &str) {
    let mut gen = Generation::new (
        Grammar::from_str (format) .unwrap (),
        start);
    println! ("- {}", start);
    for _ in 0..n {
        let tree = gen.next_tree () .unwrap ();
        println! ("  {}", tree.to_sentence ());
    }
}

#[test]
fn test_generation () {
    sentences (10, TOM_DICK_AND_HARRY, "tom-dick-and-harry");
    sentences (10, TDH, "tdh");
    sentences (10, TDH_LEFT, "tdh-left");
    sentences (10, SEXP, "sexp");
    sentences (10, SEXP, "sexp-list");
    sentences (10, SUM, "sum");
    sentences (10, AB, "ab");
    sentences (10, ABC, "abc");
}
