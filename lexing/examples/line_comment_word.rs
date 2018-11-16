use error_report::{ Span, ErrorInCtx };
use lexing::{ LexTable, Token };

fn main () -> Result<(), ErrorInCtx> {
    let lex_table = LexTable::new ()
        .quotation_mark ('"')
        .space ('\n') .space ('\t') .space (' ')
        .char (';')
        .line_comment_word ("//");
    let input = r#"
    a
    // "b"
    c;
    "#;
    let token_vec = lex_table.lex (input)?;
    println! ("- token_vec = {:#?}", token_vec);
    Ok (())
}
