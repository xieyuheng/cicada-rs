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
      (let sexp-list)
      `(case (@ sexp-list.car)
         (true-t (@ sexp-list.cdr.car))
         (false-t (@ sexp-list.cdr.cdr.car))))
    (+macro assert (let body)
      `(if [(@ body list-spread)]
         []
         ["- (assert) fail" string-print newline
          "  assertion : " string-print
          (quote (@ body)) sexp-list-print newline]))
    (+macro assert! (let body)
      `(if [(@ body list-spread)]
         []
         ["- (assert!) fail" string-print newline
          "  assertion : " string-print
          (quote (@ body)) sexp-list-print newline
          error]))



    (+fun list-length (let list)
      (if [list null-p]
        0
        [list.cdr recur number-inc]))
    (+fun list-append (let ante succ)
      (case ante
        (null-t succ)
        (cons-t ante.car ante.cdr succ recur cons-c)))
    (+fun tail-cons null-c cons-c list-append)
    (+union nat-u
      zero-t
      succ-t)

    (+data zero-t)

    (+data succ-t
      prev)

    (+fun add
      (let m n)
      (case n
        (zero-t m)
        (succ-t m n.prev recur succ-c)))

    (+fun mul
      (let m n)
      (case n
        (zero-t n)
        (succ-t m n.prev recur m add)))

    ;; (+fun factorial
    ;;   (let n)
    ;;   (case n
    ;;     (zero-t zero-c succ-c)
    ;;     (succ-t n.prev recur n mul)))

    ;; (begin
    ;;   zero-c succ-c succ-c succ-c
    ;;   zero-c succ-c succ-c succ-c add factorial)

    ;; (+fun factorial
    ;;   (let n)
    ;;   (case [n 0 eq-p]
    ;;     (true-t 1)
    ;;     (false-t n number-dec recur n number-mul)))

    ;; (begin
    ;;   10 factorial)

    ;; (begin
    ;;   1 2 3 null-c cons-c cons-c cons-c
    ;;   1 2 3 null-c cons-c cons-c cons-c eq-p)

    (+var var-1 1)

    (+macro echo-car)

    ;; (echo-car begin var-1 2 number-add)

    (begin
      `(1 2 (@ 1 2 number-add number->string))
      '(1 2 3) eq-p)


    ;; (+fun factorial
    ;;   (let n)
    ;;   n 0 eq-p
    ;;   {1}
    ;;   {n number-dec recur n number-mul}
    ;;   ifte)

    ;; (begin
    ;;   10 factorial)


    (+fun factorial
      (let n)
      (if [n 0 eq-p]
        1
        [n number-dec recur n number-mul]))

    (begin
      10 factorial)

    (assert 1 1 eq-p)


    (+gene k 2
      drop drop
      "default k"
      string-print
      newline)

    1 2 k

    (+disp k [number-t number-t]
      drop drop
      "number-t number-t k"
      string-print
      newline)

    1 2 k
