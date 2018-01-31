use std::collections::hash_map::HashMap;
use std::collections::hash_map::Entry;
use std::collections::vec_deque::VecDeque;

struct Env<'a> {
    nd: HashMap<String, Den<'a>>,
    ds: Vec<Obj<'a>>,
    rs: VecDeque<Frame<'a>>,
}

struct Frame<'a> {
    index: usize,
    jojo: Vec<Ins<'a>>,
    locals: HashMap<String, Obj<'a>>,
}

type Name<'a> = Entry<'a, String, Obj<'a>>;

fn eval_one_step(mut env: &mut Env) {
    match env.rs.back_mut() {
        Some(frame) => {
            let ins = &frame.jojo[frame.index];
            frame.index = frame.index + 1;
            exe(ins, &mut env);
        }
        None => {
            panic!("- eval_one_step: meet empty rs");
        }
    }
}

fn eval(env: &mut Env) {

}

enum Den<'a> {
   Fun(FunDen<'a>),
}

struct FunDen<'a> {
   name: String,
   jojo: Vec<Ins<'a>>,
}

enum Tag {
    Prim,
    Clo,
    Int,
    Str,
}

enum Obj<'a> {
    Prim(PrimObj),
    Clo(CloObj<'a>),
    Int(IntObj),
    Str(StrObj),
}

struct PrimObj {
    tag: Tag,
    value: fn(&mut Env),
    // ><><><
}

struct CloObj<'a> {
    tag: Tag,
    value: Vec<Ins<'a>>,
    // ><><><
}

struct IntObj {
    tag: Tag,
    value: isize,
}

struct StrObj {
    tag: Tag,
    value: String,
}

enum Ins<'a> {
    Call(CallIns<'a>),
    TailCall(TailCallIns<'a>),
    End(EndIns),
    Get(GetIns<'a>),
    Set(SetIns<'a>),
    Clo(CloIns<'a>),
}

struct CallIns<'a> {
    name: Name<'a>
}

struct TailCallIns<'a> {
    name: Name<'a>
}

struct EndIns {
}

struct GetIns<'a> {
    local_name: Name<'a>
}

struct SetIns<'a> {
    local_name: Name<'a>
}

struct CloIns<'a> {
    exp: Vec<Ins<'a>>
}

fn exe(ins: &Ins, env: &mut Env) {

}

fn main() {

}
