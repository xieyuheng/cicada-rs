# JoJo's Bizarre Programming Adventure

[![Build Status](https://travis-ci.com/xieyuheng/jojo.svg?branch=master)](https://travis-ci.com/xieyuheng/jojo)

------

> DIO: ZA WARUDO is full of languages without good type system, we don't need anymore!  
> JOJO: Just one more!  
> DIO: Do you care about HUMANITY! JOJO?  
> JOJO: I REJECT MY HUMANITY, DIO!  

------

## Features

A dynamic language -- just to practice my implementing skill -- **not to be released**

See [jojo-history](http://github.com/xieyuheng/jojo-history) for history of this project.

- object oriented, but no side-effect over objects
- implemented in rust by a [dynamic threaded code interpreter](http://www.bradrodriguez.com/papers/moving1.htm)
- uses rust's [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) as garbage collector
  - deterministic garbage collection
- sexp as syntax -- enabling a lisp-like macro system
- 🚧 simple FFI to rust library
- 🚧 forth like threading macro
  - `drop dup over tuck swap`
    - Oh tis so bizarre ~

## Getting start

With *nightly* rust toolchain :

`cargo install jojo --git https://github.com/xieyuheng/jojo`

## Examples

### Square Roots by Newton's Method

```lisp
(square x) = (mul x x)

(average x y) = (div (add x y) 2)

(= (sqrt-iter guess x)
   (if (good-enough-p guess x)
     guess
     (sqrt-iter (improve guess x) x)))

(improve guess x) = (average guess (div x guess))
(good-enough-p guess x) = (lt (abs (sub (square guess) x)) 0.001)
(sqrt x) = (sqrt-iter 1 x)

(sqrt 9)
(sqrt (add 100 21))
(square (sqrt 1000))
```

### Simpson's Rule for numerical integration

```lisp
(cube x) = (mul x (mul x x))

(= (sum term a next b)
   (if (gt a b)
     0
     (add (term a)
          (sum term (next a) next b))))

(= (simpson-integral f a b n)
   (= h (div (sub b a) n))
   (= (simpson-coefficient k)
      (cond [(or (eq 0 k) (eq n k)) 1]
            [(odd-p k) 4]
            [(even-p k) 2]))
   (= (simpson-term k)
      (mul (simpson-coefficient k)
           (f (add a (mul k h)))))
   (mul (sum simpson-term 0 inc n)
        (div h 3)))

(= assert-delta
   (macro [body]
     (= delta (car body))
     (= a (car (cdr body)))
     (= b (car (cdr (cdr body))))
     `(assert (lt (abs (sub ~a ~b)) ~delta))))

(assert-delta 0.0001
  (simpson-integral cube 0 1 100)
  (div 1 4))
```

## CODE OF CONDUCT

[Contributor Covenant Code of Conduct](CODE-OF-CONDUCT.md)

## LICENSE

[GPLv3](LICENSE)
