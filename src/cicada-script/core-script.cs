  (+union nat-u
    zero-t
    succ-t)

  (+data zero-t)

  (+data succ-t
    prev)

  (+jojo add
    (let m n)
    (case n
      (zero-t m)
      (succ-t m n.prev recur succ-c)))

  (+jojo mul
    (let m n)
    (case n
      (zero-t n)
      (succ-t m n.prev recur m add)))

  ;; (+jojo factorial
  ;;   (let n)
  ;;   (case n
  ;;     (zero-t zero-c succ-c)
  ;;     (succ-t n.prev recur n mul)))

  ;; (main
  ;;   zero-c succ-c succ-c succ-c
  ;;   zero-c succ-c succ-c succ-c add factorial)

  ;; (+jojo factorial
  ;;   (let n)
  ;;   (case [n 0 eq-p]
  ;;     (true-t 1)
  ;;     (false-t n number-dec recur n number-mul)))

  ;; (main
  ;;   10 factorial)

  (main
    1 2 3 null-c cons-c cons-c cons-c
    1 2 3 null-c cons-c cons-c cons-c eq-p)
