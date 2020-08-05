# M-expression in rust

## Welcome

Parser and tools for a variant of [M-expression](https://en.wikipedia.org/wiki/M-expression)

- zero copy parsing
- as consistent and elegant as [S-expression](https://en.wikipedia.org/wiki/S-expression)
- simple infix operator support
- command line tool
- good error message support

## Intro

S-expression and Lisp are well known from John McCarthy's
[1960 Lisp paper](http://www-formal.stanford.edu/jmc/recursive/recursive.html).
In that paper another kind of expression -- M-expression, was also introduced.
Where "M" denotes "Meta", and M-expression was used as a meta language
to specify semantics of S-expression and Lisp.

Here I show another formal language
to solve the "rigid body indentation" problem of S-expression.

I inherit the name "M-expression" from John's paper.

## The "rigid body indentation" problem

Idiomatic S-expression use "rigid body indentation",
where the whole expression is moved like a rigid body,
as the following :

```
(data
  (: t type-tt)
  (: car t)
  (: cdr (list-t t)))

(= cons-t (data
            (: t type-tt)
            (: car t)
            (: cdr (list-t t))))

(= cons-t
   (data
     (: t type-tt)
     (: car t)
     (: cdr (list-t t))))
```

Moving expression non-rigidly is not idomtic :

```
(= cons-t (data
  (: t type-tt)
  (: car t)
  (: cdr (list-t t))))
```

It wastes indentation as the following example :

```
(: nat-add-commutative
   (-- (: [x y] nat-t)
    -> (eqv-t
        (nat-add x y)
        (nat-add y x))))
(= nat-add-commutative (x y)
   (case [x]
     [zero-t] (nat-add-zero-commutative y)
     [succ-t] (eqv-compose
               (eqv-apply succ-c (nat-add-commutative x.prev y))
               (nat-add-succ-commutative y x.prev))))
```

**I want to save indentation.**

Using M-expression, I can write the above example as :

```
nat-add-commutative :
  -- [x y] : nat-t
  -> eqv-t (
       nat-add (x y)
       nat-add (y x))
nat-add-commutative (x y) = case (x) {
  zero-t => nat-add-zero-commutative (y)
  succ-t => eqv-compose (
    eqv-apply (succ-c nat-add-commutative (x.prev y))
    nat-add-succ-commutative (y x.prev))
}
```

And the above above example as :

```
data {
  t : type-tt
  car : t
  cdr : list-t (t)
}

cons-t = data {
  t : type-tt
  car : t
  cdr : list-t (t)
}
```

## Grammar of Mexp

The grammar is written in [`.grammar` format].

- **note** : the `.grammar` format itself uses M-expression as its meta syntax.

- **tip** :
  When designing a formal language, using M-expression as your meta syntax.
  - First parse sentence in your language to M-expression
  - Then convert it to algebraic data type of your language's AST

``` grammar
mexp:str  = { quotation ('"') }
mexp:char = { quotation ('\'') }
mexp:sym  = { symbol? }
mexp:dot   = { [ mexp:sym mexp:dot mexp:apply ] '.' mexp:sym }
mexp:apply = { [ mexp:sym mexp:dot mexp:apply ] arg }
mexp:array = { '[' list (mexp) ']' }
mexp:arrow = { "--" list (mexp) "->" mexp }
mexp:infix       = { mexp op? mexp }
mexp:infix-apply = { mexp op? arg }

arg:tuple = { '(' list (mexp) ')' }
arg:block = { '{' list (mexp) '}' }

list : -- Gr -> Gr
list:null (t) = {}
list:cons (t) = { t list (t) }
```

## Getting start on Command Line

With *nightly* rust toolchain :

`cargo install mexp-cli --git https://github.com/xieyuheng/cicada-rs`
