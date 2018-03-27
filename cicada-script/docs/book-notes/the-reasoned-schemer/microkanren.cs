    (+fun :)
    (+macro +alias note)
    (+macro -> (let body)
      body {'-- eq-p} list-ante
      {', eq-p not} list-filter
      sexp-filter-colon (let new-body)
      `(let (@ new-body list-spread)))
    (+fun sexp-filter-colon (let ante)
      (case ante
        (null-t null-c)
        (cons-t
          (case ante.cdr
            (null-t null-c)
            (cons-t
              (if [ante.cdr.car ': eq-p]
                [ante.car ante.cdr.cdr.cdr recur cons-c]
                [ante.cdr recur]))))))
    (+macro +type (let body)
       body.car (let name)
       body.cdr (let rest)
      `(+data (@ name) (@ rest sexp-filter-colon list-spread)))
  (+type var-t
    id : number-t)

  (+alias term-u
    (| string-t
       var-t
       term-u list-u))
  (+alias substitution-t [var-t term-u dict-t])

  (+fun empty-substitution
    : (-> -- substitution-t)
    new-dict)

  (+fun s-ext
    : (-> substitution-t
          var-t
          term-u
       -- substitution-t)
    dict-insert)

  (+fun walk
    : (-> term : term-u, substitution : substitution-t -- term-u)
    (case term
      (var-t
        (if [substitution term dict-find]
          [substitution recur]
          [term]))
      (else term)))

  (+fun unify
    : (-> s : substitution-t
          u : term-u
          v : term-u
       -- (| substitution-t
             false-t))
    u s walk (let u)
    v s walk (let v)
    (cond
      (and [u var-p] [v var-p] [u v eq-p]) [s]
      [u var-p] [s u v s-ext]
      [v var-p] [s v u s-ext]
      (and [u cons-p] [v cons-p])
      [s u.car v.car recur
       dup false-p (bool-when-not u.cdr v.cdr recur)]
      else (if [u v eq-p]
             s
             false-c)))
  (+type state-t
    substitution : substitution-t
    id-counter : number-t)
  #note
  stream-t

  (+fun unit
    : (-> state-t -- state-t stream-t)
    ><><><)

  (+fun mzero
    : (-> state-t -- state-t stream-t)
    ><><><)
  (+alias goal-t (-> state-t -- state-t stream-t))

  (+fun ==
    : (-> term-u, term-u -- goal-t)
    ><><><)

  (+fun call/fresh
    : (-> (-> var-t -- goal-t) -- goal-t)
    ><><><)

  (+fun conj
    : (-> goal-t, goal-t -- goal-t)
    ><><><)

  (+fun disj
    : (-> goal-t, goal-t -- goal-t)
    ><><><)

  (+fun mplus
    : (-> --))

  (+fun bind
    : (-> --))
    (begin
      empty-substitution
      '(a b c)
      '(a b c)
      unify
      empty-substitution
      eq-p bool-assert)

    (begin
      empty-substitution
      '((a b c) (a b c) (a b c))
      '((a b c) (a b c) (a b c))
      unify
      empty-substitution
      eq-p bool-assert)

    (begin
      empty-substitution
      (lit-list 'a 'b 0 var-c)
      (lit-list 'a 'b 'c)
      unify
      empty-substitution 0 var-c 'c s-ext
      eq-p bool-assert)

