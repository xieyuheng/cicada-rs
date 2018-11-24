[![Build Status](https://travis-ci.com/xieyuheng/cicada.svg?branch=master)](https://travis-ci.com/xieyuheng/cicada)

# Cicada Language // 蟬語

Aims :
- intuitive to formalize mathematics theories
- approachable for both mathematicians and programmers

Features :
- dependently-typed
- logic programming interface
- with nominal subtyping and inheriting
- uses M-expression as meta syntax

Contains :
- [cicada](cicada/README.md) -- the implementation
- [cic](cic/README.md) -- command line interface
- [cicada-notebook](cicada-notebook/README.md) -- interactive UI

## Docs

- [typed logic programming](docs/typed-logic-programming/README.md)
- (todo) [dependable prover]
- (todo) [the little cicada]

## This Repo Also Contains ...

Language implementation practice :
- [jojo](jojo/README.md) -- a dynamic language uses S-expression as meta syntax
- [wissen](wissen/README.md) -- an un-typed logic programming language
- [eopl](eopl/README.md) -- << Essentials of Programming Languages >>
- [kanren](kanren/README.md) -- Jason's microkanren and its Yoneda embedding version

Useful libraries and tools :
- [error-report](error-report/README.md) -- error report in context -- for parser and compiler
- [lexing](lexing/README.md) -- zero copy lexer
- [mexp](mexp/README.md) -- M-expression parser and command line tools
- (todo) [parsing] -- a parser generator with type system -- specifies `.gr` and `.tr` formats

## Contributing

We enforce C4 as collaboration protocol :
- [The C4 RFC](https://rfc.zeromq.org/spec:42/C4)
- [Style Guide](STYLE-GUIDE.md)

## Code Of Conduct

- [Contributor Covenant Code of Conduct](CODE-OF-CONDUCT.md)

## License

- [GPLv3](LICENSE)
