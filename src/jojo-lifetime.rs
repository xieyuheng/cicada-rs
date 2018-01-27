use std::collections::hash_map::HashMap;
use std::collections::hash_map::Entry;

struct Env<'a> {
    nd: HashMap<String, Obj<'a>>,
    ds: Vec<Obj<'a>>,
    rs: Frame<'a>,
}

struct Frame<'a> {
    index: usize,
    jojo: Vec<Exp<'a>>,
    locals: HashMap<String, Obj<'a>>,
}

type Name<'a> = Entry<'a, String, Obj<'a>>;

enum Obj<'a> {
    Prim    { tag: Tag, value: fn(&mut Env) },
    Clo     { tag: Tag, value: Vec<Exp<'a>> },
    Bool    { tag: Tag, value: bool },
    Int     { tag: Tag, value: isize },
    String  { tag: Tag, value: String },
    // Dict   { tag: Tag, value: },
    // Vect   { tag: Tag, value: },
    Data    { tag: Tag, value: String },
}

enum Tag {
    Primitive,
    Closure,
    Bool,
    Int,
    String,
    // Dict,
    // Vect,
    Data,
}

enum Exp<'a> {
    Call  { name: Name<'a> },
    Get   { local_name: Name<'a> },
    Set   { local_name: Name<'a> },
    Clo   { exp: &'a Exp<'a> },
    Begin { body: Vec<Exp<'a>>},
}

fn main() {

}
