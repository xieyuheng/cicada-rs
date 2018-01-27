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

enum Tag {
    Prim,
    Clo,
    Bool,
    Int,
    Str,
    Dict,
    Vect,
    Data,
}

enum Obj<'a> {
    Prim(ObjPrim),
    Clo(ObjClo<'a>),
    Bool(ObjBool),
    Int(ObjInt),
    Str(ObjStr),
    Dict(ObjDict),
    Vect(ObjVect),
    Data(ObjData),
}

struct ObjPrim {
    tag: Tag,
    value: fn(&mut Env),
}

struct ObjClo<'a> {
    tag: Tag,
    value: Vec<Exp<'a>>,
}

struct ObjBool {
    tag: Tag,
    value: bool,
}

struct ObjInt {
    tag: Tag,
    value: isize,
}

struct ObjStr {
    tag: Tag,
    value: String,
}

struct ObjDict {
    tag: Tag,
    value: String,
}

struct ObjVect {
    tag: Tag,
    value: String,
}

struct ObjData {
    tag: Tag,
    value: String,
}

enum Exp<'a> {
   Call(ExpCall<'a>),
   Get(ExpGet<'a>),
   Set(ExpSet<'a>),
   Clo(ExpClo<'a>),
   Begin(ExpBegin<'a>),
}

struct ExpCall<'a> {
    name: Name<'a>
}

struct ExpGet<'a> {
    local_name: Name<'a>
}

struct ExpSet<'a> {
    local_name: Name<'a>
}

struct ExpClo<'a> {
    exp: &'a Exp<'a>
}

struct ExpBegin<'a> {
    body: Vec<Exp<'a>>
}

fn main() {

}
