(note
  (let :x 37
    (proc :y
      (let :z (diff :y :x)
        (diff :x :y)))))

(note
  (let-nameless 37
    (proc-nameless
      (let-nameless (diff (var-nameless 0) (var-nameless 1))
        (diff (var-nameless 2) (var-nameless 1))))))

(note program = exp)

(note exp
  = <const-exp> -- 123
  | <zero?-exp> -- (diff ... ...)
  | <if-exp>    -- (if ... ... ...)
  | <diff-exp>  -- (zero? ...)

  | <var-exp>   -- :var
  | <let-exp>   -- (let :var ... ...)
  | <proc-exp>  -- (proc :var ...)

  | <call-exp>  -- (call ... ...))

(+data <const-exp> .num)
(+data <zero?-exp> .exp1)
(+data <if-exp>    .exp1 .exp2 .exp3)
(+data <diff-exp>  .exp1 .exp2)
(+data <var-exp>   .var)
(+data <let-exp>   .var .exp1 .body)

(+data <proc-exp>  .var .body)
(+data <call-exp>  .rator .rand)

(note nameless-exp
  = <const-exp> -- 123
  | <zero?-exp> -- (diff ... ...)
  | <if-exp>    -- (if ... ... ...)
  | <diff-exp>  -- (zero? ...)

  | <call-exp>  -- (call ... ...)

  | <var-nameless-exp> -- (var-nameless :n)
  | <let-nameless-exp> -- (let-nameless ... ...)
  | <proc-nameless-exp> -- (proc-nameless ...))

(+data <var-nameless-exp> .num)
(+data <let-nameless-exp> .exp1 .body)
(+data <proc-nameless-exp> .body)

(note
  (let :x 37
    (proc :y
      (let :z (diff :y :x)
        (diff :x :y)))))

(+data <null-static-env>)
(+data <cons-static-env> .rest .var)

(+jojo static-env-apply
  (-> :var :static-env -- lexical-address)
  (case :static-env

    <null-static-env>
    ["- static-env-apply fail" p nl
     "  can not find var : " p :var p nl
     error]

    <cons-static-env>
    (if [:static-env .var :var equal?]
      0
      [:var :static-env .rest static-env-apply inc])))

(+jojo tran (-> :exp :static-env -- nameless-exp)
  (case :exp
    <const-exp> :exp
    <zero?-exp> [:exp .exp1 :static-env tran zero?-exp]
    <if-exp>    [:exp .exp1 :static-env tran
                 :exp .exp2 :static-env tran
                 :exp .exp3 :static-env tran if-exp]
    <diff-exp>  [:exp .exp1 :static-env tran
                 :exp .exp2 :static-env tran diff-exp]
    <call-exp>  [:exp .rator :static-env tran
                 :exp .rand  :static-env tran call-exp]

    <var-exp>   [:exp .var
                 :static-env static-env-apply
                 var-nameless-exp]
    <let-exp>   [:static-env, :exp .var, cons-static-env
                 :new-static-env!
                 :exp .exp1 :static-env tran
                 :exp .body :new-static-env tran
                 let-nameless-exp]
    <proc-exp>  [:static-env, :exp .var, cons-static-env
                 :new-static-env!
                 :exp .body :new-static-env tran
                 proc-nameless-exp]))

(+data <null-nameless-env>)
(+data <cons-nameless-env> .rest .val)

(+jojo nameless-env-apply
  (-> :lexical-address :nameless-env -- denval)
  (cond [:nameless-env null-nameless-env?]
        ["- nameless-env-apply fail" p nl
         "  lexical-address greater then" p nl
         "  length of nameless-env" p nl
         error]
        [:lexical-address 0 eq?] [:nameless-env .val]
        else [:lexical-address dec
              :nameless-env .rest nameless-env-apply]))

(note denval = expval)

(note expval
  = <num-val>
  | <bool-val>
  | <proc-val>)

(+data <num-val>  .num)
(+data <bool-val> .bool)

(+data <proc-val> .proc)

(+data <proc> .body .saved-env)

(+jojo proc-apply (-> :val :proc -- expval)
  :proc .saved-env :val cons-nameless-env
  :proc .body swap eval)

(+jojo eval (-> :exp :env -- expval)
  (case :exp
    <const-exp> [:exp .num num-val]
    <zero?-exp> [:exp .exp1 :env eval .num 0 equal? bool-val]
    <if-exp>    (if [:exp .exp1 :env eval .bool]
                    [:exp .exp2 :env eval]
                    [:exp .exp3 :env eval])
    <diff-exp>  [:exp .exp1 :env eval .num
                 :exp .exp2 :env eval .num sub num-val]
    <call-exp>  [:exp .rator :env eval .proc :proc!
                 :exp .rand  :env eval :arg!
                 :arg :proc proc-apply]

    <var-nameless-exp> [:exp .num :env nameless-env-apply]
    <let-nameless-exp> [:env
                        :exp .exp1 :env eval
                        cons-nameless-env :new-env!
                        :exp .body :new-env eval]
    <proc-nameless-exp> [:exp .body :env proc proc-val]))

(+jojo init-static-env
  null-static-env
  ':x cons-static-env
  ':v cons-static-env
  ':i cons-static-env)

(+jojo init-nameless-env
  null-nameless-env
  10 num-val cons-nameless-env
  5  num-val cons-nameless-env
  1  num-val cons-nameless-env)

(+jojo program-eval (-> :program -- expval)
  :program init-nameless-env eval)

(+jojo parse (-> :sexp -- exp)
  (cond
    [:sexp string?] [:sexp parse/string]
    [:sexp cons?] [:sexp parse/cons]
    else  ["- parse fail" p nl
           "  can not parse sexp : " p
           :sexp sexp-print nl
           error]))

(+jojo parse/string (-> :string -- exp)
  (cond
    [:string int-string?]   [:string string->int const-exp]
    [:string local-string?] [:string var-exp]
    else ["- parse/string fail" p nl
          "  can not parse string : " p :string p nl
          error]))

(+jojo parse/cons (-> :sexp -- exp)
  :sexp .car :key!
  :sexp .cdr :body!
  (cond [:key 'zero? eq?] [:body parse/spread zero?-exp]
        [:key 'if eq?]    [:body parse/spread if-exp]
        [:key 'diff eq?]  [:body parse/spread diff-exp]
        [:key 'let eq?]   [:body .car :body
                           .cdr parse/spread let-exp]

        [:key 'proc eq?]  [:body .car :body
                           .cdr parse/spread proc-exp]
        [:key 'call eq?]  [:body parse/spread call-exp]))

(+jojo parse/spread {parse} list-map list-spread)

(+jojo NAMELESS (-> :body --)
  :body {parse
         init-static-env tran
         program-eval
         expval-print nl}
  list-for-each
  '(begin))

(+jojo expval-print (-> :expval)
  (case :expval
    <num-val>  [:expval .num p]
    <bool-val> [:expval .bool p]))

(NAMELESS
 1 2 3
 (diff 2 1)
 (diff 3 1)
 (let :y 5 (diff :x :y))
 (zero? 1)
 (zero? 0)
 (let :y (diff :x 3)
   (if (zero? :y) 0 666))

 (let :x 200
   (let :f (proc :z (diff :z :x) )
     (let :x 100
       (let :g (proc :z (diff :z :x))
         (diff (call :f 1)
           (call :g 1)))))))

(NAMELESS
 (let :makemult
   (proc :maker
     (proc :x (if (zero? :x) 0
                (diff (call (call :maker :maker)
                        (diff :x 1))
                  -4))))
   (let :times4
     (proc :x
       (call (call :makemult :makemult) :x))
     (call :times4 3))))
