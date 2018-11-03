use std::io;
use std::sync::Arc;

use linefeed as lf;

type CharVec = Vec <char>;

static NEW_PROMPT: &'static str = ">>> ";
static BRA_PROMPT: &'static str = "... ";

fn new_env () -> jojo_core::Env {
    let mut env = jojo_core::Env::new ();
    jojo_core::expose_core (&mut env);
    env
}

pub fn repl () -> io::Result<()> {
    let mut code = String::new ();
    let mut env = new_env ();

    let interface = Arc::new (lf::Interface::new ("jojo")?);
    interface.set_prompt (NEW_PROMPT)?;

    while let lf::ReadResult::Input (line) = interface.read_line ()? {
        code = format! ("{}{}", &code, line);
        match bar_ket_check (&code) {
            Ok (BalanceResult::Balanced) => {
                let sexp_list = jojo_core::sexp_list_from_code (&code);
                let result = std::panic::catch_unwind (
                    std::panic::AssertUnwindSafe (|| {
                        jojo_core::top_sexp_list_repl (&mut env, sexp_list);
                    }));
                code = String::new ();
                interface.set_prompt (NEW_PROMPT)?;
                if let Err (_cause) = result {
                    eprintln! ("- resetting env to a new one, due to panic!");
                    env = new_env ();
                }
            }
            Ok (BalanceResult::ToBeBalanced) => {
                interface.set_prompt (BRA_PROMPT)?;
            }
            Err (error) => {
                println! ("{}", error);
                code = String::new ();
                interface.set_prompt (NEW_PROMPT)?;
            }
        }
    }
    Ok (())
}

enum BalanceResult {
    Balanced,
    ToBeBalanced,
}

fn in_doublequote_p (bar_stack: &CharVec) -> bool {
    let len = bar_stack.len ();
    if len == 0 {
        false
    } else {
        let last_bar = &bar_stack [len-1];
        last_bar == &'"'
    }
}

fn bar_to_ket (bar: char) -> char {
    match bar {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic! ("bar_to_ket fail"),
    }
}

fn error_no_bar (ket: char) -> String {
    format! (r#"
- bar_ket_check fail
  no bar
  ket : {}
"#, ket)
}

fn error_mis_bar (bar: char, ket: char) -> String {
    format! (r#"
- bar_ket_check fail
  bar : {}
  ket : {}
"#, bar, ket)
}

fn bar_ket_check (code: &str) -> Result <BalanceResult, String> {
    let mut bar_stack = CharVec::new ();
    for c in code.chars () {
        match c {
            '(' | '[' | '{' => {
                if ! in_doublequote_p (&bar_stack) {
                    bar_stack.push (c);
                }
            }
            ')' | ']' | '}' => {
                if ! in_doublequote_p (&bar_stack) {
                    if let Some (bar) = bar_stack.pop () {
                        let ket = bar_to_ket (bar);
                        if c != ket {
                            return Err (error_mis_bar (bar, c));
                        }
                    } else {
                        return Err (error_no_bar (c));
                    }
                }
            }
            '"' => {
                if ! in_doublequote_p (&bar_stack) {
                    bar_stack.push (c);
                } else {
                    bar_stack.pop ();
                }
            }
            _ => {
                // ok
            }
        }
    }
    if bar_stack.len () == 0 {
        Ok (BalanceResult::Balanced)
    } else {
        Ok (BalanceResult::ToBeBalanced)
    }
}
