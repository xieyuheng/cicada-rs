package xieyuheng.partech

case class Parser(lexer: Lexer, rule: Rule, partech: Partech) {

  def parse(text: String): Tree = {
    val tokens = this.lexer.lex(text)
    this.partech.parse_tokens_by_rule(tokens, this.rule)
  }

}
