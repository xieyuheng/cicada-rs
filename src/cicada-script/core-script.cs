    (+fun drop (let x))
    (+fun dup (let x) x x)
    (+fun over (let x y) x y x)
    (+fun tuck (let x y) y x y)
    (+fun swap (let x y) y x)
    (+fun ifte
      (let test true-fn false-fn)
      (case test
        (true-t true-fn)
        (false-t false-fn)))
    (+macro if
      (let body)
      body.car (let question)
      body.cdr.car (let on-true)
      body.cdr.cdr.car (let on-false)
      `(case (@ question)
         (true-t (@ on-true))
         (false-t (@ on-false))))
    (+macro assert (let body)
      `(if [(@ body list/spread)]
         []
         ["- assertion fail : " string/print
          (quote (@ body)) sexp-list/print nl]))

    (+macro assert! (let body)
      `(if [(@ body list/spread)]
         []
         ["- assertion fail : " string/print
          (quote (@ body)) sexp-list/print nl
          error]))
    (+macro when (let body)
      `(if (@ body.car)
         (@ 'begin body.cdr cons-c)
         []))

    (+macro unless (let body)
      `(if (@ body.car)
         []
         (@ 'begin body.cdr cons-c)))
    (+macro and (let body)
      (if [body null-p]
        'true-c
        `(if (@ body.car)
           [(@ body.cdr recur)]
           false-c)))

    (+macro or (let body)
      (if [body null-p]
        'false-c
        `(if (@ body.car)
           true-c
           [(@ body.cdr recur)])))
    (+macro cond (let body)
      (if [body list/length 1 number/lteq-p]
        `(begin
           "- cond mismatch!" string/print nl
           error)
        [body.car (when [dup 'else eq-p] drop 'true-c) (let question)
         body.cdr.car (let answer)
         `(if (@ question)
            (@ answer)
            (@ body.cdr.cdr recur))]))
    (+fun :)
    ;; (+macro (->) )

    (+macro lit/list (let body)
      `(begin mark (@ body list/spread) collect-list))
    (+fun list/length (let list)
      (if [list null-p]
        0
        [list.cdr recur number/inc]))
    (+fun list/append (let ante succ)
      (case ante
        (null-t succ)
        (cons-t ante.car ante.cdr succ recur cons-c)))
    (+fun tail-cons null-c cons-c list/append)
    (+fun list/any-p (let list pred)
      (cond [list null-p] false-c
            [list.car pred] true-c
            else [list.cdr {pred} recur]))
    (+fun list/every-p (let list pred)
      (cond [list null-p] true-c
            [list.car pred bool/not] false-c
            else [list.cdr {pred} recur]))
    (+fun list/ante (let list pred)
      (cond [list null-p] null-c
            [list.car pred] null-c
            else [list.car
                  list.cdr {pred} recur
                  cons-c]))
    (+fun list/succ (let list pred)
      (cond [list null-p] null-c
            [list.car pred] list
            else [list.cdr {pred} recur]))
    (+fun list/split (let list pred) ;; -- ante succ
      (cond [list null-p] [null-c null-c]
            [list.car pred] [null-c list]
            else [list.car
                  list.cdr {pred} recur
                  succ! cons-c succ]))
    (+fun list/map (let list fun)
      (case list
        (null-t null-c)
        (cons-t list.car fun list.cdr {fun} recur cons-c)))
    (+fun list/for-each (let list fun)
      (case list
        (null-t)
        (cons-t list.car fun list.cdr {fun} recur)))
    (+fun list/filter (let list pred)
      (cond [list null-p] null-c
            [list.car pred]
            [list.car list.cdr {pred} recur cons-c]
            else [list.cdr {pred} recur]))
    (+fun list/reverse null-c swap list/reverse-swap-append)

    (+fun list/reverse-swap-append (let ante list)
      (case list
        (null-t)
        (cons-t list.car swap cons-c list.cdr recur)))
    (+fun list/foldr (let list b a-b->b)
      (case list
        (null-t b)
        (cons-t
          list.car
          list.cdr b {a-b->b} recur
          a-b->b)))
    (+fun list/foldl (let list b b-a->b)
      (case list
        (null-t b)
        (cons-t
          list.cdr b {b-a->b} recur
          list.car
          b-a->b)))
    (+fun list/member-p (let list x)
      list {x eq-p} list/any-p)
    (+gene repr 1
      default-repr)
    (+disp repr [string-t]
      doublequote/string swap string/append
      doublequote/string string/append)
    (+disp repr [number-t]
      number->string)
    (+gene w 1
      repr string/print)
    (+gene p 1
      w)
    (+disp p [string-t]
      string/print)
    (+gene length 1
      error)
    (+gene empty-p 1
      error)
    (+gene gt-p 2
      error)
    (+gene lt-p 2
      error)
    (note
      (+gene gteq-p 2
        (let x y)
        (or [x y eq-p]
            [x y gt-p])))
    (note
      (+gene lteq-p 2
        (let x y)
        (or [x y eq-p]
            [x y lt-p])))
    (+fun times (let fun n)
      (unless [n 0 number/lteq-p]
        fun
        {fun} n number/dec recur))
      (assert
        true-c false-c bool/and
        false-c eq-p)

      (assert
        true-c false-c bool/or
        true-c eq-p)

      (assert
        true-c bool/not
        false-c eq-p)

      (assert
        true-c bool/not bool/not
        true-c eq-p)
      (assert
        1 2 3 null-c cons-c cons-c cons-c
        1 2 3 null-c cons-c cons-c cons-c eq-p)
        (+fun number/factorial/case
          (let n)
          (case [n 0 eq-p]
            (true-t 1)
            (false-t n number/dec recur n number/mul)))

        (assert
          5 number/factorial/case
          120 eq-p)

        (+fun number/factorial/ifte
          (let n)
          n 0 eq-p
          {1}
          {n number/dec recur n number/mul}
          ifte)

        (assert
          5 number/factorial/ifte
          120 eq-p)

        (+fun number/factorial
          (let n)
          (if [n 0 eq-p]
            1
            [n number/dec recur n number/mul]))

        (assert
          5 number/factorial
          120 eq-p)
      (assert
        "0123" string/length
        4 eq-p)

      (assert
        "0123" "4567" string/append
        "01234567" eq-p)

      (assert
        "01234567" 3 string/ref
        "3" eq-p)

      (assert
        "01234567" 3 5 string/slice
        "34" eq-p)

      (assert
        123 number->string
        "123" eq-p)
      (+union nat-u
        zero-t
        succ-t)

      (+data zero-t)

      (+data succ-t
        prev)

      (+fun nat/add
        (let m n)
        (case n
          (zero-t m)
          (succ-t m n.prev recur succ-c)))

      (+fun nat/mul
        (let m n)
        (case n
          (zero-t n)
          (succ-t m n.prev recur m nat/add)))

      (+fun nat/factorial
        (let n)
        (case n
          (zero-t zero-c succ-c)
          (succ-t n.prev recur n nat/mul)))

      (assert
        zero-c succ-c succ-c succ-c succ-c succ-c nat/factorial
        zero-c succ-c succ-c succ-c succ-c succ-c
        zero-c succ-c succ-c succ-c succ-c nat/mul
        zero-c succ-c succ-c succ-c nat/mul
        zero-c succ-c succ-c nat/mul
        zero-c succ-c nat/mul
        eq-p)
      (+var var/cons 1 null-c cons-c)

      (assert
        2 var/cons.car!
        var/cons 2 null-c cons-c eq-p)

      (+fun nat->number
        (let n)
        (case n
          (zero-t 0)
          (succ-t n.prev recur number/inc)))

      (+var var/nat zero-c succ-c succ-c)

      (assert
        var/nat nat->number 2 eq-p)

      (assert
        zero-c var/nat.prev!
        var/nat nat->number 1 eq-p)
      (assert
        `(1 2 (@ 1 2 number/add number->string))
        '(1 2 3) eq-p)
      (+gene gene0 2
        drop drop
        "default gene0")

      (assert 1 2 gene0 "default gene0" eq-p)

      (+disp gene0 [number-t number-t]
        drop drop
        "number-t number-t gene0")

      (assert 1 2 gene0 "number-t number-t gene0" eq-p)
      (assert
        (when [1 1 eq-p] 'ok)
        'ok eq-p)

      (assert
        true-c
        (unless [1 1 eq-p] 'ugh))

      (assert
        true-c
        (when [1 2 eq-p] 'ugh))

      (assert
        (unless [1 2 eq-p] 'ok)
        'ok eq-p)
        (assert
          "" {"*" string/append} 3 times
          "***" eq-p)
        (assert '(a b c) list/length 3 eq-p)
        (assert '(a b c) '(d e f) list/append '(a b c d e f) eq-p)
        (assert '(a b c) 'd tail-cons '(a b c d) eq-p)

        (assert
          mark 0 1 2 3 4 collect-list
          (lit/list 0 1 2 3 4)
          eq-p)

        (assert
          (lit/list 0 1 2 3 4)
          (lit/list 5 6 7 8 9)
          list/append
          (lit/list 0 1 2 3 4 5 6 7 8 9)
          eq-p)

        (assert
          (or       [3 2 eq-p]
                    [1 1 eq-p]
                    ))

        ;; (assert
        ;;   (lit/list 0 1 2 3 4 5 6 7 8 9)
        ;;   {5 gteq-p} list/ante
        ;;   (lit/list 0 1 2 3 4)
        ;;   eq-p)

        ;; (assert
        ;;   (lit/list 0 1 2 3 4 5 6 7 8 9)
        ;;   {5 gteq-p} list/split
        ;;   swap (lit/list 0 1 2 3 4) eq-p
        ;;   swap (lit/list 5 6 7 8 9) eq-p
        ;;   and)

        ;; (assert
        ;;   (lit/list 0 1 2 3 4 5 6 7 8 9)
        ;;   {inc} list/map
        ;;   (lit/list 1 2 3 4 5 6 7 8 9 10)
        ;;   eq-p)

        ;; (assert
        ;;   (lit/list 0 1 2 3 4 5 6 7 8 9)
        ;;   {2 mod 0 eq-p} list/filter
        ;;   (lit/list 0 2 4 6 8)
        ;;   eq-p)

        ;; (assert
        ;;   (lit/list 0 1 2 3 4 5 6 7 8 9)
        ;;   list/reverse
        ;;   (lit/list 9 8 7 6 5 4 3 2 1 0)
        ;;   eq-p)


        ;; (assert
        ;;   (lit/list 0 1 2 3 4 5) 100 {add} list/foldr
        ;;   0 1 2 3 4 5 100 add add add add add add
        ;;   eq-p)

        ;; (assert
        ;;   (lit/list 0 1 2 3 4 5) 100 {add} list/foldl
        ;;   100 5 add 4 add 3 add 2 add 1 add 0 add
        ;;   eq-p)

        ;; (assert
        ;;   (lit/list
        ;;     (lit/list 1 2 3)
        ;;     (lit/list 4 5 6)
        ;;     (lit/list 7 8 9))
        ;;   null-c {list/append} list/foldr

        ;;   (lit/list 1 2 3, 4 5 6, 7 8 9)
        ;;   eq-p)

        ;; (assert
        ;;   (lit/list
        ;;     (lit/list 1 2 3)
        ;;     (lit/list 4 5 6)
        ;;     (lit/list 7 8 9))
        ;;   null-c {list/append} list/foldl
        ;;   (lit/list 7 8 9, 4 5 6, 1 2 3)
        ;;   eq-p)

        ;; (assert
        ;;   (lit/list 1 2 3 4 5) (lit/list 1 2 3 4 5) list/append
        ;;   10 tail-cons list/length
        ;;   11 eq-p)

        ;; (assert
        ;;   (lit/list 1 2 3 4 5) 2 list/ref
        ;;   3 eq-p)


