use error_report::{ Span, ErrorInCtx };
use lexing::{ LexTable, Token };

fn main () -> Result<(), ErrorInCtx> {
    let lex_table = LexTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';');
    let input = r#"a "b" c;"#;
    let token_vec = lex_table.lex (input)?;
    let mut iter = token_vec.iter ();
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 0, hi: 1 },
        word: "a",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Quotation {
        span: Span { lo: 2, hi: 5 },
        quotation_mark: '"',
        string: "b",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Word {
        span: Span { lo: 6, hi: 7 },
        word: "c",
    });
    assert_eq! (iter.next () .unwrap (), &Token::Char {
        span: Span { lo: 7, hi: 8 },
        ch: ';',
    });
    assert_eq! (iter.next (), None);
    println! ("- token_vec = {:#?}", token_vec);
    Ok (())
}
