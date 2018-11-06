use error_report::ErrorInCtx;
use mexp::{
    SyntaxTable,
    Mexp,
};

const NULL_DATA: &'static str = r#"
data {
  t : type-tt
}
"#;

fn main () -> Result<(), ErrorInCtx> {
    let input = NULL_DATA;
    let syntax_table = SyntaxTable::default ();
    let mut mexp_vec = syntax_table.parse (input)?;
    assert! (mexp_vec.len () == 1);
    let mexp: Mexp = mexp_vec.pop () .unwrap ();
    println! ("- mexp = {:#?}", mexp);
    println! ("- to_string = {}", mexp.to_string ());
    println! ("- to_tree_format = {}", mexp.to_tree_format ());
    Ok (())
}
