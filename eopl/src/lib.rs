#![feature (uniform_paths)]

#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

use std::sync::Arc;
use error_report::{
    Span,
    ErrorMsg,
    ErrorCtx,
    ErrorInCtx,
};
use mexp_parser::{
    SyntaxTable,
    Mexp,
    Arg,
};

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Exp {
    Const { num: isize },
    ZeroP { exp1: Arc <Exp> },
    If { exp1: Arc <Exp>, exp2: Arc <Exp>, exp3: Arc <Exp> },
    Diff { exp1: Arc <Exp>, exp2: Arc <Exp> },
    Var { var: Var },
    Let { var: Var, exp1: Arc <Exp>, body: Arc <Exp> },
}

pub type Var = String;

fn note_about_mexp_syntax_of_exp () -> ErrorMsg {
    ErrorMsg::new ()
        .head ("mexp syntax of <Exp>")
        .line ("<Exp::Const> = num?")
        .line ("<Exp::ZeroP> = zero-p (<Exp>)")
        .line ("<Exp::If> = if { <Exp> <Exp> <Exp> }")
        .line ("<Exp::Diff> = diff (<Exp> <Exp>)")
        .line ("<Exp::Var> = <Var>")
        .line ("<Exp::Let> = let { <Var> = <Exp> <Exp> }")
}

fn num_symbol_p (symbol: &str) -> bool {
    symbol .parse::<isize> () .is_ok ()
}

fn var_symbol_p (symbol: &str) -> bool {
    ! num_symbol_p (symbol)
}

fn exp_from_sym_mexp <'a> (
    mexp: Mexp <'a>,
) -> Result <Exp, ErrorInCtx> {
    if let Mexp::Sym {
        span: symbol_span,
        symbol
    } = mexp {
        if num_symbol_p (symbol) {
            let num = symbol.parse::<isize> () .unwrap ();
            Ok (Exp::Const { num })
        } else if var_symbol_p (symbol) {
            let var = symbol.to_string ();
            Ok (Exp::Var { var })
        } else {
            ErrorInCtx::new ()
                .head ("unknown symbol")
                .span (symbol_span)
                .wrap_in_err ()
        }
    } else {
        panic! ("exp_from_sym_mexp")
    }
}

fn exp_from_apply_tuple_arg <'a> (
    mexp: Mexp <'a>,
) -> Result <Exp, ErrorInCtx> {
    if let Mexp::Apply {
        span: apply_span,
        head,
        arg: Arg::Tuple {
            span: arg_span,
            mut body,
        },
    } = mexp {
        match &*head {
            Mexp::Sym {
                symbol: "zero-p", ..
            } => {
                if body.len () == 1 {
                    let mexp = body.pop () .unwrap ();
                    let exp1 = exp_from_mexp (mexp)?;
                    Ok (Exp::ZeroP {
                        exp1: Arc::new (exp1),
                    })
                } else {
                    ErrorInCtx::new ()
                        .head ("wrong arity of `zero-p`")
                        .line ("the arity should be 1")
                        .span (arg_span)
                        .wrap_in_err ()
                }
            }
            Mexp::Sym {
                symbol: "diff", ..
            } => {
                if body.len () == 2 {
                    let mexp = body.pop () .unwrap ();
                    let exp2 = exp_from_mexp (mexp)?;
                    let mexp = body.pop () .unwrap ();
                    let exp1 = exp_from_mexp (mexp)?;
                    Ok (Exp::Diff {
                        exp1: Arc::new (exp1),
                        exp2: Arc::new (exp2),
                    })
                } else {
                    ErrorInCtx::new ()
                        .head ("wrong arity of `diff`")
                        .line ("the arity should be 2")
                        .span (arg_span)
                        .wrap_in_err ()
                }
            }
            _ => {
                ErrorInCtx::new ()
                    .head ("unknown apply")
                    .span (apply_span)
                    .wrap_in_err ()
            }
        }
    } else {
        panic! ("exp_from_apply_tuple_arg")
    }
}

fn exp_from_apply_block_arg <'a> (
    mexp: Mexp <'a>,
) -> Result <Exp, ErrorInCtx> {
    if let Mexp::Apply {
        span: apply_span,
        head,
        arg: Arg::Block {
            span: arg_span,
            mut body,
        },
    } = mexp {
        match &*head {
            Mexp::Sym {
                symbol: "if", ..
            } => {
                if body.len () == 3 {
                    let mexp = body.pop () .unwrap ();
                    let exp3 = exp_from_mexp (mexp)?;
                    let mexp = body.pop () .unwrap ();
                    let exp2 = exp_from_mexp (mexp)?;
                    let mexp = body.pop () .unwrap ();
                    let exp1 = exp_from_mexp (mexp)?;
                    Ok (Exp::If {
                        exp1: Arc::new (exp1),
                        exp2: Arc::new (exp2),
                        exp3: Arc::new (exp3),
                    })
                } else {
                    ErrorInCtx::new ()
                        .head ("syntax error in `if {}`")
                        .line ("there must be 3 <exp> in `{}`")
                        .span (arg_span)
                        .wrap_in_err ()
                }
            }
            Mexp::Sym {
                symbol: "let", ..
            } => {
                if body.len () == 2 {
                    let mexp = body.pop () .unwrap ();
                    let exp2 = exp_from_mexp (mexp)?;
                    let mexp = body.pop () .unwrap ();
                    match mexp {
                        Mexp::Infix {
                            span: _infix_span,
                            op: "=",
                            lhs, rhs,
                        } => match &*lhs {
                            Mexp::Sym {
                                span: _var_span,
                                symbol,
                            } => {
                                let exp1 = exp_from_mexp (
                                    (*rhs).clone ())?;
                                Ok (Exp::Let {
                                    var: symbol.to_string (),
                                    exp1: Arc::new (exp1),
                                    body: Arc::new (exp2),
                                })
                            }
                            _ => {
                                ErrorInCtx::new ()
                                    .head ("syntax error in `let {}`")
                                    .line ("the first <exp> in `{}` must be `<exp:var> = <exp>`")
                                    .span (lhs.span ())
                                    .wrap_in_err ()
                            }
                        }
                        _ => {
                            ErrorInCtx::new ()
                                .head ("syntax error in `let {}`")
                                .line ("the first <exp> in `{}` must be `<exp:var> = <exp>`")
                                .span (mexp.span ())
                                .wrap_in_err ()
                        }
                    }
                } else {
                    ErrorInCtx::new ()
                        .head ("syntax error in `let {}`")
                        .line ("there must be 2 <exp> in `{}`")
                        .span (arg_span)
                        .wrap_in_err ()
                }
            }
            _ => {
                ErrorInCtx::new ()
                    .head ("unknown apply")
                    .span (apply_span)
                    .wrap_in_err ()
            }
        }
    } else {
        panic! ("exp_from_apply_block_arg")
    }
}

pub fn exp_from_mexp <'a> (
    mexp: Mexp <'a>,
) -> Result <Exp, ErrorInCtx> {
    match mexp {
        Mexp::Sym { ..
        } => exp_from_sym_mexp (mexp),
        Mexp::Apply {
            arg: Arg::Tuple { .. },
            ..
        } => exp_from_apply_tuple_arg (mexp),
        Mexp::Apply {
            arg: Arg::Block { .. },
            ..
        } => exp_from_apply_block_arg (mexp),
        _ => {
            ErrorInCtx::new ()
                .head ("unknown mexp")
                .span (mexp.span ())
                .note (note_about_mexp_syntax_of_exp ())
                .wrap_in_err ()
        }
    }
}

pub fn exp_vec_from_str (
    s: &str,
) -> Result <Vec <Exp>, ErrorInCtx> {
    let syntax_table = SyntaxTable::default ();
    let mexp_vec = syntax_table.parse (s)?;
    let mut exp_vec = Vec::new ();
    for mexp in mexp_vec {
        exp_vec.push (exp_from_mexp (mexp)?);
    }
    Ok (exp_vec)
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Val {
    Num { num: isize },
    Bool { boolean: bool },
}

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Env {
    Null {},
    Cons { var: Var, val: Val, rest: Arc <Env> },
}

impl Env {
    pub fn apply (&self, var: &Var) -> Val {
        match self {
            Env::Null {} => {
                panic! ("Env::apply fail");
            }
            Env::Cons { var: head, val, rest } => {
                if head == var {
                    val.clone ()
                } else {
                    rest.apply (var)
                }
            }
        }
    }
}

impl Env {
    pub fn eval (&self, exp: &Exp) -> Val {
        match exp {
            Exp::Const { num } => {
                Val::Num { num: *num }
            }
            Exp::ZeroP { exp1 } => {
                let boolean = {
                    self.eval (exp1) == Val::Num { num: 0 }
                };
                Val::Bool { boolean }
            }
            Exp::If { exp1, exp2, exp3 } => {
                if let Val::Bool { boolean } = self.eval (exp1) {
                    if boolean {
                        self.eval (exp2)
                    } else {
                        self.eval (exp3)
                    }
                } else {
                    panic! ("eval Exp:If fail")
                }
            }
            Exp::Diff { exp1, exp2 } => {
                let num1 = if let Val::Num { num }
                = self.eval (exp1) {
                    num
                } else {
                    panic! ("eval Exp::Diff fail")
                };
                let num2 = if let Val::Num { num }
                = self.eval (exp2) {
                    num
                } else {
                    panic! ("eval Exp::Diff fail")
                };
                Val::Num { num: num1 - num2 }
            }
            Exp::Var { var } => {
                self.apply (var)
            }
            Exp::Let { var, exp1, body } => {
                let new_env = Env::Cons {
                    var: var.clone (),
                    val: self.eval (exp1),
                    rest: Arc::new (self.clone ()),
                };
                new_env.eval (body)
            }
        }
    }
}

#[cfg (test)]
const EXAMPLE_CODE: &'static str = "
1
2
3

diff (2 1)
diff (3 1)

zero-p (1)
zero-p (0)

if { zero-p (1)
  0
  666
}

let {
  y = 5
  diff (x y)
}

let {
  y = diff (x 3)
  if { zero-p (y)
    0
    666
  }
}
";

#[test]
fn test_exp_vec_from_str () {
    let input = EXAMPLE_CODE;
    match exp_vec_from_str (input) {
        Ok (exp_vec) => {
            for exp in exp_vec {
                println! ("- exp = {:?}", exp);
            }
        }
        Err (error) => {
            error.report (
                ErrorCtx::new ()
                    .body (input))
        }
    }
}

#[test]
fn test_env_eval () {
    let input = EXAMPLE_CODE;
    let env = Env::Null {};
    let env = Env::Cons {
       var: "x".to_string (),
       val: Val::Num { num: 6 },
       rest: Arc::new (env),
    };

    match exp_vec_from_str (input) {
        Ok (exp_vec) => {
            for exp in exp_vec {
                let val = env.eval (&exp);
                println! ("> {:?}\n{:?}", exp, val);
            }
        }
        Err (error) => {
            error.report (
                ErrorCtx::new ()
                    .body (input))
        }
    }
}
