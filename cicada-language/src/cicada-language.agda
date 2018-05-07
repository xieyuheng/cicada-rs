module cicada-language where

data bool-u : Set where
  true-c : bool-u
  false-c : bool-u

not : bool-u -> bool-u
not true-c = false-c
not false-c = true-c

or : bool-u -> bool-u -> bool-u
or false-c x = x
or true-c _  = true-c

and : bool-u -> bool-u -> bool-u
and true-c x = x
and false-c _  = false-c

iff : bool-u -> bool-u -> bool-u
iff true-c true-c = true-c
iff true-c false-c = false-c
iff false-c true-c = false-c
iff false-c false-c = true-c

xor : bool-u -> bool-u -> bool-u
xor a b = (not (iff a b))

-- if for lazy eval
if : {n : _} {A : Set n} -> bool-u -> A -> A -> A
if true-c x y = x
if false-c x y = y

data nat-u : Set where
  zero-c : nat-u
  succ-c : nat-u -> nat-u

add : nat-u -> nat-u -> nat-u
add zero-c n = n
add (succ-c m) n = (succ-c (add m n))

mul : nat-u -> nat-u -> nat-u
mul zero-c n = zero-c
mul (succ-c m) n = (add n (mul m n))

equal-p : nat-u -> nat-u -> bool-u
equal-p zero-c zero-c = true-c
equal-p zero-c (succ-c n) = false-c
equal-p (succ-c n) zero-c = false-c
equal-p (succ-c n) (succ-c m) = (equal-p n m)

data sum-t (A : Set) (B : A -> Set) : Set where
  sum-c : (first : A) -> (second : B first) -> (sum-t A B)

example-1 : sum-t nat-u (λ x -> (if (equal-p x zero-c) nat-u bool-u))
example-1 = (sum-c zero-c zero-c)

example-2 : sum-t nat-u (λ x -> (if (equal-p x zero-c) nat-u bool-u))
example-2 = (sum-c (succ-c zero-c) false-c)

Either : Set -> Set -> Set
Either A B = (sum-t bool-u (λ x -> (if x A B)))

left : {A B : _} ->
       A -> Either A B
left x = (sum-c true-c x)

right : {A B : _} ->
        B -> Either A B
right x = (sum-c false-c x)

either : {A B C : Set} ->
         (A -> C) -> (B -> C) -> Either A B -> C
either f g (sum-c true-c x) = f x
either f g (sum-c false-c x) = g x

Pair : Set -> Set -> Set
Pair A B = (sum-t A (λ _ -> B))

uncurry : {A B C : Set} ->
          (A -> B -> C) -> Pair A B -> C
uncurry f (sum-c x y) = (f x y)

compose : {A : Set}
          {B : A -> Set}
          {C : {x : A} -> (B x) -> Set} ->
          (f : {x : A} (y : B x) -> (C y)) ->
          (g : (x : A) -> (B x)) ->
          ((x : A) -> (C (g x)))
compose f g = (λ x -> (f (g x)))


data eqv-t {A : Set} (q : A) : A -> Set where
  eqv-c : (eqv-t q q)

cong : {A B : _} (f : A -> B)
       {x y : _} -> (eqv-t x y) -> (eqv-t (f x) (f y))
cong f eqv-c = eqv-c

associative : (x y z : nat-u) ->
              (eqv-t (add x (add y z)) (add (add x y) z))
associative zero-c y z = eqv-c
associative (succ-c x) y z = (cong succ-c (associative x y z))

-- commutative : (x y : nat-u) -> (eqv-t (add x y) (add y x))
-- commutative zero-c zero-c = eqv-c
-- commutative (succ-c x) zero-c = (cong succ-c (commutative x zero-c))
-- commutative zero-c y = {!!}
-- commutative (succ-c x) y = {!!}

data vect-u (A : Set) : nat-u -> Set where
  null-vect-c : vect-u A zero-c
  cons-vect-c : {n : nat-u} -> A -> (vect-u A n) -> (vect-u A (succ-c n))

-- inner : (n : nat-u) -> vect-u nat-u n -> vect-u nat-u n -> nat-u
-- (inner zero-c null-vect-c null-vect-c) = zero-c
-- (inner (succ-c .n) (cons-vect-c .n x xs) (cons-vect-c n y ys)) = (add (mul x y) (inner n xs ys))

-- inner : (n : nat-u) -> vect-u nat-u n -> vect-u nat-u n -> nat-u
-- (inner ._ null-vect-c null-vect-c) = zero-c
-- (inner ._ (cons-vect-c ._ x xs) (cons-vect-c _ y ys)) = (add (mul x y) (inner _ xs ys))

inner : {n : nat-u} -> vect-u nat-u n -> vect-u nat-u n -> nat-u
inner null-vect-c null-vect-c = zero-c
inner (cons-vect-c x xs) (cons-vect-c y ys) = (add (mul x y) (inner xs ys))
