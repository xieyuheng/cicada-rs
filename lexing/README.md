# Lexing

## Welcome

A lexer in rust

- zero copy
- handles doublequoted string

## Example

### abc

``` rust
use error_report::{ Span, ErrorInCtx };
use lexing::{ CharTable, Token };

fn main () -> Result<(), ErrorInCtx> {
    let char_table = CharTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"a "b" c;"#;
    let token_vec = char_table.lex (input)?;
    println! ("- token_vec = {:#?}", token_vec);
    Ok (())
}
```

output :

``` rust
- token_vec = [
    Word {
        span: Span {
            lo: 0,
            hi: 1
        },
        word: "a"
    },
    Quotation {
        span: Span {
            lo: 2,
            hi: 5
        },
        quotation_mark: '\"',
        string: "b"
    },
    Word {
        span: Span {
            lo: 6,
            hi: 7
        },
        word: "c"
    },
    Char {
        span: Span {
            lo: 7,
            hi: 8
        },
        ch: ';'
    }
]
```
