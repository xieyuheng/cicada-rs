# cicada

[![Build Status](https://travis-ci.com/xieyuheng/cicada.svg?branch=master)](https://travis-ci.com/xieyuheng/cicada)

## Welcome

cicada is a programming language

aims :
- intuitive to formalize mathematics theories

features :
- dependently-typed
- object-oriented
- logic programming interface
- uses M-expression as meta syntax

contains :
- [cicada](cicada/README.md) -- the implementation
- [cic](cic/README.md) -- command line interface
- [cicada-notebook](cicada-notebook/README.md) -- runs in web browser, with rich UI
- [cicada-server] (todo) -- language server
- [cicada-shell] (todo) -- a script language -- uses tmp file as function's local variable

## This Repo Also Contains ...

language implementation practice :
- [jojo](jojo/README.md) -- a dynamic language uses S-expression as meta syntax
- [wissen](wissen/README.md) -- an un-typed logic programming language
- [eopl](eopl/README.md) -- << Essentials of Programming Languages >>
- [kanren](kanren/README.md) -- Jason's microkanren and its Yoneda embedding version

useful libraries and tools :
- [error-report](error-report/README.md) -- error report in context -- for parser and compiler
- [lexing](lexing/README.md) -- zero copy lexer
- [mexp](mexp/README.md) -- M-expression parser and command line tools
- [parsing] (todo) -- a parser generator with type system -- specifies `.gr` and `.tr` formats

## Contributing

We use Collective Code Construction Contract (a.k.a. C4) as our collaboration protocol.

- [The C4 RFC](https://rfc.zeromq.org/spec:42/C4)
- [Style Guide](STYLE-GUIDE.md)

## CODE OF CONDUCT

[Contributor Covenant Code of Conduct](CODE-OF-CONDUCT.md)

## LICENSE

[GPLv3](LICENSE)
