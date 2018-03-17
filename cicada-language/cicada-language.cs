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
    (+type env-t
      name-dict : [string-t den-u dict-t]
      data-stack : [obj-u list-u]
      frame-stack : [frame-u list-u]
      scope-stack : [scope-t list-u]
      goal-stack : [goal-t list-u]
      data-bind-dict : [hypo-id-t obj-u dict-t]
      type-bind-dict : [hypo-id-t obj-u dict-t])
    (+alias scope-t [string-t obj-u dict-t])
    (+fun new-env
      : (-> -- env-t)
      (lit-dict)
      (lit-list)
      (lit-list)
      (lit-list)
      (lit-list)
      (lit-dict)
      (lit-dict)
      (. name-dict
         data-stack
         frame-stack
         scope-stack
         goal-stack
         data-bind-dict
         type-bind-dict)
      env-cr)
      (+fun name-dict-get
        : (-> env-t, name : string-t -- env-t den-u)
        dup .name-dict name dict-get)
      (+fun name-dict-insert
        : (-> env : env-t, name : string-t, den : den-u -- env-t)
        env.name-dict name den dict-insert
        (. name-dict)
        env clone)
      (+fun data-stack-push
        : (-> env : env-t, obj : obj-u -- env-t)
        obj env.data-stack cons-c
        (. data-stack)
        env clone)
      (+fun data-stack-pop
        : (-> env : env-t -- env-t obj-u)
        env.data-stack.cdr
        (. data-stack)
        env clone
        env.data-stack.car)
      (+fun data-stack-drop
        : (-> env-t -- env-t)
        data-stack-pop drop)
      (+fun data-stack-tos
        : (-> env-t -- env-t obj-u)
        dup .data-stack.car)
      (+fun data-stack-n-pop
        : (-> env-t, number-t
           -- env-t, obj-u list-u)
        null-c data-stack-n-pop-to-list)

      (+fun data-stack-n-pop-to-list
        : (-> env-t, n : number-t, list : [obj-u list-u]
           -- env-t, obj-u list-u)
        (if [n 0 lteq-p]
          list
          [data-stack-pop list cons-c
           n dec swap recur]))
      (+fun data-stack-list-push
        : (-> env-t, list : [obj-u list-u]
           -- env-t)
        (case list
          (null-t)
          (cons-t
            list.car data-stack-push
            list.cdr recur)))
      (+fun frame-stack-push
        : (-> env : env-t, frame : frame-u -- env-t)
        frame env.frame-stack cons-c
        (. frame-stack)
        env clone)
      (+fun frame-stack-pop
        : (-> env : env-t -- env-t frame-u)
        env.frame-stack.cdr
        (. frame-stack)
        env clone
        env.frame-stack.car)
      (+fun frame-stack-drop
        : (-> env-t -- env-t)
        frame-stack-pop drop)
      (+fun frame-stack-tos
        : (-> env-t -- env-t frame-u)
        dup .frame-stack.car)
      (+union frame-u
        scoping-frame-t
        simple-frame-t)
      (+type scoping-frame-t
        body-exp-list : [exp-u list-u]
        index : number-t)
      (+fun new-scoping-frame
        : (-> body-exp-list : [exp-u list-u] -- scoping-frame-t)
        body-exp-list 0
        (. body-exp-list index)
        scoping-frame-cr)
      (+type simple-frame-t
        body-exp-list : [exp-u list-u]
        index : number-t)
      (+fun new-simple-frame
        : (-> body-exp-list : [exp-u list-u] -- simple-frame-t)
        body-exp-list 0
        (. body-exp-list index)
        simple-frame-cr)
      (+fun top-frame-finished-p
        : (-> env-t -- env-t bool-u)
        frame-stack-tos (let frame)
        frame.index frame.body-exp-list list-length eq-p)
      (+fun top-frame-next-exp
        : (-> env-t -- env-t exp-u)
        frame-stack-pop (let frame)
        frame.index number-inc
        (. index)
        frame clone
        frame-stack-push
        frame.body-exp-list frame.index list-ref)
      (+fun scope-stack-push
        : (-> env : env-t
              scope : scope-t
           -- env-t)
        scope env.scope-stack cons-c
        (. scope-stack)
        env clone)
      (+fun scope-stack-pop
        : (-> env : env-t -- env-t scope-t)
        env.scope-stack.cdr
        (. scope-stack)
        env clone
        env.scope-stack.car)
      (+fun scope-stack-drop
        : (-> env-t -- env-t)
        scope-stack-pop drop)
      (+fun scope-stack-tos
        : (-> env-t -- env-t scope-t)
        dup .scope-stack.car)
      (+fun scope-get dict-get)
      (+fun scope-insert
        : (-> scope-t
              local-name : string-t
              obj : obj-u
           -- scope-t)
        (lit-dict local-name obj)
        dict-update)
      (+fun current-scope-insert
        : (-> env-t
              local-name : string-t
              obj : obj-u
           -- env-t)
      scope-stack-pop
      local-name obj scope-insert
      scope-stack-push)
      (+fun new-scope
        : (-> -- scope-t)
        (lit-dict))
      (+fun data-bind-dict-find
        : (-> env-t, hypo-id : hypo-id-t
           -- env-t (| false-t
                       [obj-u true-t]))
        dup .data-bind-dict hypo-id dict-find)
      (+fun data-bind-dict-insert
        : (-> env : env-t
              hypo-id : hypo-id-t
              obj : obj-u
           -- env-t)
        env.data-bind-dict hypo-id obj dict-insert
        (. data-bind-dict)
        env clone)
      (+fun type-bind-dict-find
        : (-> env-t, hypo-id : hypo-id-t
           -- env-t (| false-t
                       [obj-u true-t]))
        dup .type-bind-dict hypo-id dict-find)
      (+fun type-bind-dict-insert
        : (-> env : env-t
              hypo-id : hypo-id-t
              obj : obj-u
           -- env-t)
        env.type-bind-dict hypo-id obj dict-insert
        (. type-bind-dict)
        env clone)
    (+disp p [env-t]
      name-dict-print
      ;; goal-stack-print
      ;; data-bind-dict-print
      ;; type-bind-dict-print
      scope-stack-print
      frame-stack-print
      data-stack-print
      drop
      nl)
    (+fun name-dict-print
      : (-> env-t -- env-t)
      "- name-dict : " p nl
      dup .name-dict
      {(let key den)
       "  " p key p " = " p den p nl}
      dict-for-each
      nl)
    (+fun data-stack-print
      : (-> env-t -- env-t)
      "- data-stack : " p nl
      "- data-stack : " p nl
      dup .data-stack
      {"  " p p nl}
      list-for-each
      nl)
    (+fun frame-stack-print
      : (-> env-t -- env-t)
      "- frame-stack : " p nl
      dup .frame-stack
      {"  " p p nl}
      list-for-each
      nl)
    (+fun scope-stack-print
      : (-> env-t -- env-t)
      "- scope-stack : " p nl
      dup .scope-stack
      {"  " p p nl}
      list-for-each
      nl)
    (+fun goal-stack-print
      : (-> env-t -- env-t)
      )
    (+fun data-bind-dict-print
      : (-> env-t -- env-t)
      )
    (+fun type-bind-dict-print
      : (-> env-t -- env-t)
      )
    (+union exp-u
      call-exp-t
      let-exp-t
      closure-exp-t
      arrow-exp-t
      apply-exp-t
      case-exp-t
      field-exp-t
      colon-exp-t
      double-colon-exp-t
      comma-exp-t
      type-tt-exp-t)
    (+type call-exp-t
      name : string-t)
    (+type let-exp-t
      local-name-list : [string-t list-u])
    (+type closure-exp-t
      body-exp-list : [exp-u list-u])
    (+type arrow-exp-t
      ante-exp-list : [exp-u list-u]
      succ-exp-list : [exp-u list-u])
    (+type apply-exp-t)
    (+type case-exp-t
      arg-exp-list : [exp-u list-u]
      closure-exp-dict : [string-t closure-exp-t dict-t])
    (+type field-exp-t
      field-name : string-t)
    (+type colon-exp-t
      local-name : string-t
      type-exp-list : [exp-u list-u])
    (+type double-colon-exp-t
      local-name : string-t
      type-exp-list : [exp-u list-u])
    (+type comma-exp-t)
    (+type type-tt-exp-t)
    (+union den-u
      fun-den-t
      data-cons-den-t
      type-cons-den-t
      union-cons-den-t)
    (+type fun-den-t
      name : string-t
      type-arrow-exp : arrow-exp-t
      body-exp-list : [exp-u list-u])
    (+type data-cons-den-t
      name : string-t
      type-arrow-exp : arrow-exp-t
      cons-arrow-exp : arrow-exp-t)
    (+type type-cons-den-t
      name : string-t
      type-arrow-exp : arrow-exp-t
      cons-arrow-exp : arrow-exp-t)
    (+type union-cons-den-t
      name : string-t
      type-arrow-exp : arrow-exp-t
      sub-name-list : [string-t list-u])
    (+union obj-u
      data-obj-t data-type-t
      union-type-t
      type-type-t
      closure-obj-t arrow-type-t
      data-hypo-t type-hypo-t)
    (+type data-obj-t
      data-type : data-type-t
      field-obj-dict : [string-t obj-u dict-t])
    (+type data-type-t
      name : string-t
      field-obj-dict : [string-t obj-u dict-t])
    (+type union-type-t
      name : string-t
      field-obj-dict : [string-t obj-u dict-t])
    (+type type-type-t
      level : number-t)
    (+type closure-obj-t
      scope : scope-t
      body-exp-list : [exp-u list-u])
    (+type arrow-type-t
      ante-type-list : [obj-u list-u]
      succ-type-list : [obj-u list-u])
    (+type data-hypo-t
      id : hypo-id-t)
    (+type type-hypo-t
      id : hypo-id-t)
    (+type hypo-id-t
      string : string-t)
    (+fun exe
      : (-> env-t exp-u -- env-t)
      (case dup
        (call-exp-t call-exp-exe)
        (let-exp-t let-exp-exe)
        (closure-exp-t closure-exp-exe)
        (arrow-exp-t arrow-exp-exe)
        (apply-exp-t apply-exp-exe)
        (case-exp-t case-exp-exe)
        (field-exp-t field-exp-exe)
        (colon-exp-t colon-exp-exe)
        (double-colon-exp-t double-colon-exp-exe)
        (comma-exp-t comma-exp-exe)
        (type-tt-exp-t type-tt-exp-exe)))
    (+fun call-exp-exe
      : (-> env-t, exp : call-exp-t -- env-t)
      exp.name name-dict-get den-exe)
      (+fun den-exe
        : (-> env-t den-u -- env-t)
        (case dup
          (fun-den-t fun-den-exe)
          (data-cons-den-t data-cons-den-exe)
          (type-cons-den-t type-cons-den-exe)
          (union-cons-den-t union-cons-den-exe)))
      (+fun fun-den-exe
        : (-> env-t, den : fun-den-t -- env-t)
        new-scope scope-stack-push
        den.type-arrow-exp exp-collect-one drop
        den.type-arrow-exp.ante-exp-list exp-list-let-colon
        den.body-exp-list new-scoping-frame frame-stack-push)
      (+fun exp-list-let-colon
        : (-> env : env-t
              exp-list : [exp-u list-u]
           -- env-t)
        exp-list
        {(let exp)
         (or [exp colon-exp-p]
             [exp colon-exp-p])}
        list-filter
        list-reverse
        {env swap exp-let-colon} list-for-each
        env)
      (+fun exp-let-colon
        : (-> env-t
              exp : exp-u
           -- env-t)
        data-stack-pop
        exp.local-name swap
        current-scope-insert)
      (+fun data-cons-den-exe
        : (-> env-t, den : data-cons-den-t -- env-t)
        den.type-arrow-exp exp-collect-one drop
        den.cons-arrow-exp.succ-exp-list
        exp-list-collect-one
        (let data-type)
        den.cons-arrow-exp.ante-exp-list new-field-obj-dict
        data-type
        (. field-obj-dict data-type)
        data-obj-cr
        data-stack-push)
      (+fun type-cons-den-exe
        : (-> env-t, den : type-cons-den-t -- env-t)
        den.type-arrow-exp.ante-exp-list new-field-obj-dict
        den.name
        (. field-obj-dict name)
        data-type-cr
        data-stack-push)
      (+fun union-cons-den-exe
        : (-> env-t, den : union-cons-den-t -- env-t)
        den.type-arrow-exp.ante-exp-list new-field-obj-dict
        den.name
        (. field-obj-dict name)
        union-type-cr
        data-stack-push)
      (+fun new-field-obj-dict
        : (-> env-t
              ante-exp-list : [exp-u list-u]
           -- env-t, string-t obj-u dict-t)
        new-dict ante-exp-list
        ante-exp-list-merge-fields)
      (+fun ante-exp-list-merge-fields
        : (-> env-t
              field-obj-dict : [string-t obj-u dict-t]
              ante-exp-list : [exp-u list-u]
           -- env-t, string-t obj-u dict-t)
        (case ante-exp-list
          (null-t field-obj-dict)
          (cons-t
            (case ante-exp-list.car
              (colon-exp-t
                data-stack-pop (let obj)
                field-obj-dict
                ante-exp-list.car.local-name
                obj dict-insert
                ante-exp-list.cdr recur)
              (else
                field-obj-dict
                ante-exp-list.cdr recur)))))
    (+fun let-exp-exe
      : (-> env-t, exp : let-exp-t -- env-t)
      exp.local-name-list list-reverse
      let-exp-exe-loop)
    (+fun let-exp-exe-loop
      : (-> env-t, local-name-list : [string-t list-u] -- env-t)
      (case local-name-list
        (null-t)
        (cons-t
          data-stack-pop (let obj)
          scope-stack-pop
          local-name-list.car obj scope-insert
          scope-stack-push
          local-name-list.cdr recur)))
    (+fun closure-exp-exe
      : (-> env-t, exp : closure-exp-t -- env-t)
      scope-stack-tos
      exp.body-exp-list
      (. scope body-exp-list)
      closure-obj-cr
      data-stack-push)
    (+fun arrow-exp-exe
      : (-> env-t, exp : arrow-exp-t -- env-t)
      ;; calling collect-many
      ;;   might effect current scope
      exp.ante-exp-list exp-list-collect-many (let ante-type-list)
      exp.succ-exp-list exp-list-collect-many (let succ-type-list)
      ante-type-list succ-type-list
      arrow-type-c
      data-stack-push)
    (+fun apply-exp-exe
      : (-> env-t, exp : apply-exp-t -- env-t)
      data-stack-pop (let obj)
      (case obj
        (closure-obj-t
          obj.scope scope-stack-push
          obj.body-exp-list new-scoping-frame frame-stack-push)))
    (+fun case-exp-exe
      : (-> env-t, exp : case-exp-t -- env-t)
      ;; calling collect-one
      ;;   might effect current scope
      exp.arg-exp-list exp-list-collect-one (let obj)
      (case obj
        (data-obj-t
          exp.closure-exp-dict
          obj.data-type.name dict-get
          closure-exp-exe
          apply-exp-exe)))
    (+fun field-exp-exe
      : (-> env-t, exp : field-exp-t -- env-t)
      data-stack-pop (let obj)
      (case obj
        (data-obj-t
          obj.field-obj-dict
          exp.field-name dict-get)))
    (+fun colon-exp-exe
      : (-> env-t, exp : colon-exp-t -- env-t)
      exp.type-exp-list exp-list-collect-one (let type)
      exp.local-name generate-hypo-id (let hypo-id)
      hypo-id type-hypo-c
      type type-hypo-insert
      exp.local-name hypo-id data-hypo-c current-scope-insert
      type data-stack-push)
    (+fun double-colon-exp-exe
      : (-> env-t double-colon-exp-t -- env-t)
      colon-exp-exe
      data-stack-drop)
    (+type counter-t
      number : number-t)
    (+fun new-counter
      : (-> -- counter-t)
      0 counter-c)
    (+fun counter-inc
      : (-> counter-t --)
      dup .number inc
      swap .number!)
    (+fun counter-number
      : (-> counter-t -- number-t)
      .number)
    (+var hypo-id-counter new-counter)

    (+fun generate-hypo-id
      : (-> env-t, base-name : string-t
         -- env-t, hypo-id-t)
      hypo-id-counter counter-number repr (let postfix)
      hypo-id-counter counter-inc
      base-name postfix string-append hypo-id-c)
    (+fun comma-exp-exe
      : (-> env-t comma-exp-t -- env-t)
      drop)
    (+fun type-tt-exp-exe
      : (-> env-t type-tt-exp-t -- env-t)
      drop
      1 type-type-c
      data-stack-push)
    (+fun run-one-step
      : (-> env-t -- env-t)
      (if top-frame-finished-p
        (case frame-stack-pop
          (scoping-frame-t scope-stack-drop)
          (simple-frame-t))
        [top-frame-next-exp exe]))
    (+fun run-with-base
      : (-> env-t, base : number-t -- env-t)
      (unless [dup .frame-stack list-length base eq-p]
        run-one-step base recur))
    (+fun exp-list-run
      : (-> env-t, exp-list : [exp-u list-u] -- env-t)
      dup .frame-stack list-length (let base)
      exp-list new-simple-frame frame-stack-push
      base run-with-base)
    (+fun exp-run
      : (-> env-t, exp-u -- env-t)
      null-c cons-c exp-list-run)
    (+fun exp-list-collect-many
      : (-> env-t, exp-list : [exp-u list-u]
         -- env-t, obj-u list-u)
      dup .data-stack list-length (let old)
      exp-list exp-list-run
      dup .data-stack list-length (let new)
      new old number-sub data-stack-n-pop)
    (+fun exp-list-collect-one
      : (-> env-t, exp-list : [exp-u list-u]
         -- env-t, obj-u)
      exp-list exp-list-run
      data-stack-pop)
    (+fun exp-collect-one
      : (-> env-t, exp : exp-u
         -- env-t, obj-u)
      exp null-c cons-c exp-list-run
      data-stack-pop)
    (+fun cut
      : (-> env-t exp-u -- env-t)
      (case dup
        (call-exp-t call-exp-cut)
        (let-exp-t let-exp-cut)
        (closure-exp-t closure-exp-cut)
        (arrow-exp-t arrow-exp-cut)
        (apply-exp-t apply-exp-cut)
        (case-exp-t case-exp-cut)
        (field-exp-t field-exp-cut)
        (colon-exp-t colon-exp-cut)
        (double-colon-exp-t double-colon-exp-cut)))
    (+fun call-exp-cut
      : (-> env-t, exp : call-exp-t -- env-t)
      exp.name name-dict-get den-cut)
      (+fun den-cut
        : (-> env-t den-u -- env-t)
        (case dup
          (fun-den-t fun-den-cut)
          (type-cons-den-t type-cons-den-cut)
          (union-cons-den-t union-cons-den-cut)))
      (+fun fun-den-cut
        : (-> env-t, den : fun-den-t -- env-t)
        den.type-arrow-exp arrow-exp-cut-apply)
      (+fun arrow-exp-cut-apply
        : (-> env-t, arrow-exp : arrow-exp-t -- env-t)
        ;; must create a new scope
        ;;   before creating an arrow-type
        ;; because creating an arrow-type
        ;;   might effect current scope
        new-scope scope-stack-push
        arrow-exp exp-collect-one (let arrow-type)
        arrow-type.ante-type-list ante-type-list-unify
        arrow-type.succ-type-list data-stack-list-push
        scope-stack-drop)
      (+fun ante-type-list-unify
        : (-> env-t, ante-type-list : [obj-u list-u] -- env-t)
        )
      (+fun type-cons-den-cut
        : (-> env-t, den : type-cons-den-t -- env-t)
        )
      (+fun union-cons-den-cut
        : (-> env-t, den : union-cons-den-t -- env-t)
        )
    (+fun infer
      : (-> env-t obj-u -- obj-u env-t)
      (case dup
        (data-obj-t data-obj-infer)
        (closure-obj-t closure-obj-infer)
        ;; ><><><
        (obj-u type-infer)))
    (+alias sexp-u (| string-t [sexp-u list-u]))
    (+fun sexp-list-pass
      : (-> sexp-u list-u -- sexp-u list-u)
      ;; the order matters
      {sexp-pass-for-recur} list-map
      sexp-list-remove-infix-notation
      sexp-list-expand-multi-bind
      {sexp-pass-for-arrow} list-map
      sexp-list-pass-to-break-dot-string)
    (+fun sexp-pass-for-recur
      : (-> sexp : sexp-u -- sexp-u)
      (if (and [sexp cons-p]
               [sexp.car '+fun eq-p])
        [sexp.cdr.car (let name)
         sexp.cdr.cdr (let body)
         (lit-list sexp.car name)
         body name sexp-substitute-recur
         list-append]
        sexp))

    (+fun sexp-substitute-recur
      : (-> sexp : sexp-u, name : string-t -- sexp-u)
      (cond
        (and [sexp string-p] [sexp "recur" eq-p]) name
        [sexp cons-p] [sexp.car name recur
                       sexp.cdr name recur cons-c]
        else sexp))
    (+fun sexp-list-remove-infix-notation
      : (-> sexp-list : [sexp-u list-u] -- sexp-u list-u)
      (cond [sexp-list list-length 3 lt-p]
            [sexp-list {sexp-remove-infix-notation} list-map]

            (or [sexp-list.cdr.car ': eq-p]
                [sexp-list.cdr.car ':: eq-p])
            [sexp-list.cdr.cdr.cdr recur
             (lit-list
              sexp-list.cdr.car
              sexp-list.car sexp-remove-infix-notation
              sexp-list.cdr.cdr.car sexp-remove-infix-notation)
             swap cons-c]

            else
            [sexp-list.cdr recur
             sexp-list.car sexp-remove-infix-notation
             swap cons-c]))

    (+fun sexp-remove-infix-notation
      : (-> sexp-u -- sexp-u)
      dup cons-p (bool-when sexp-list-remove-infix-notation))
    (+fun sexp-list-expand-multi-bind
      : (-> sexp-list : [sexp-u list-u] -- sexp-u list-u)
      (case sexp-list
        (null-t null-c)
        (cons-t
          (cond
            [sexp-list.car multi-bind-colon-sexp-p]
            [sexp-list.car colon-sexp-head (let head)
             sexp-list.car colon-sexp-type (let type)
             sexp-list.car colon-sexp-multi-bind-list
             {(let name) `((@ head name type))} list-map
             sexp-list.cdr recur
             list-append]
            else
            [sexp-list.cdr recur
             sexp-list.car sexp-expand-multi-bind
             swap cons-c]))))

    (+fun sexp-expand-multi-bind
      : (-> sexp-u -- sexp-u)
      dup cons-p (bool-when sexp-list-expand-multi-bind))

    (+fun colon-sexp-p
      : (-> sexp : sexp-u -- bool-u)
      (and [sexp cons-p]
           (or [sexp.car ': eq-p]
               [sexp.car ':: eq-p])))

    (+fun multi-bind-colon-sexp-p
      : (-> sexp : sexp-u -- bool-u)
      (and [sexp colon-sexp-p]
           [sexp.cdr.car cons-p]
           [sexp.cdr.car.car 'begin eq-p]))

    (+fun colon-sexp-multi-bind-list
      : (-> sexp : sexp-u -- string-t list-u)
      sexp.cdr.car
      .cdr)

    (+fun colon-sexp-head
      : (-> sexp : sexp-u -- sexp-u)
      sexp.car)

    (+fun colon-sexp-type
      : (-> sexp : sexp-u -- sexp-u)
      sexp.cdr.cdr.car)
    (+fun sexp-pass-for-arrow
      : (-> sexp : sexp-u -- sexp-u)
      (case sexp
        (cons-t
          (if [sexp.car '-> eq-p]
            [sexp.cdr {'-- eq-p} list-split-to-two (let ante succ)
             `(arrow (@ ante {recur} list-map)
                     (@ succ.cdr {recur} list-map))]
            [sexp {recur} list-map]))
        (else sexp)))
    (+fun sexp-list-pass-to-break-dot-string
      : (-> sexp-list : [sexp-u list-u] -- sexp-u list-u)
      (case sexp-list
        (null-t null-c)
        (cons-t
          (cond
            (and [sexp-list.car string-p]
                 [sexp-list.car dot-string-p])
            [sexp-list.car '. string-split-by-char (let name-list)
             name-list.cdr {'. swap string-append} list-map
             name-list.car swap cons-c
             sexp-list.cdr recur
             list-append]
            else
            [sexp-list.cdr recur
             sexp-list.car sexp-pass-to-break-dot-string
             swap cons-c]))))

    (+fun sexp-pass-to-break-dot-string
      : (-> sexp-u -- sexp-u)
      dup cons-p (bool-when sexp-list-pass-to-break-dot-string))

    (+fun dot-string-p
      : (-> string : string-u -- bool-u)
      (and [string string-head '. eq-p not]
           [string string-last '. eq-p not]
           [string '. string-member-p]))
      (+fun parse-den
        : (-> sexp : sexp-u -- den-u)
        sexp.car (let head)
        sexp.cdr (let body)
        (cond
          [head '+fun eq-p] [body parse-fun-den]
          [head '+type eq-p] [body parse-type-cons-den]
          [head '+union eq-p] [body parse-union-cons-den]
          else error))
      (+fun parse-fun-den
        : (-> body : [sexp-u list-u] -- den-u)
        body.car parse-exp (let colon-exp)
        body.cdr {parse-exp} list-map (let body-exp-list)
        colon-exp.local-name.name (let name)
        colon-exp.type-exp-list.car (let type-exp)
        (case type-exp
          (arrow-exp-t type-exp)
          (else (lit-list) (lit-list type-exp) arrow-exp-c))
        (let type-arrow-exp)
        name type-arrow-exp body-exp-list fun-den-c)
      (+fun parse-type-cons-den
        : (-> body : [sexp-u list-u] -- den-u)
        body.car parse-exp (let colon-exp)
        body.cdr {parse-exp} list-map .car (let cons-arrow-exp)
        colon-exp.local-name.name (let name)
        colon-exp.type-exp-list.car (let type-exp)
        (case type-exp
          (arrow-exp-t type-exp)
          (else (lit-list) (lit-list type-exp) arrow-exp-c))
        (let type-arrow-exp)
        name type-arrow-exp cons-arrow-exp type-cons-den-c)
      (+fun parse-union-cons-den
        : (-> body : [sexp-u list-u] -- den-u)
        body.car parse-exp (let colon-exp)
        body.cdr (let sub-name-list)
        colon-exp.local-name.name (let name)
        colon-exp.type-exp-list.car (let type-exp)
        (case type-exp
          (arrow-exp-t type-exp)
          (else (lit-list) (lit-list type-exp) arrow-exp-c))
        (let type-arrow-exp)
        name type-arrow-exp sub-name-list union-cons-den-c)
    (+fun parse-exp
      : (-> sexp : sexp-u -- exp-u)
      (if [sexp string-p]
        [sexp string-parse-exp]
        [sexp.car (let head)
         sexp.cdr (let body)
         (cond
           [head 'let eq-p]
           [body {recur} list-map let-exp-c]

           [head 'closure eq-p]
           [body {recur} list-map closure-exp-c]

           [head 'arrow eq-p]
           [body list-spread
            {recur} list-map swap
            {recur} list-map swap
            arrow-exp-c]

           [head 'case eq-p]
           [new-dict
            body.cdr
            {(let clause)
             clause.cdr {recur} list-map closure-exp-c
             clause.car swap dict-insert}
            list-for-each
            (lit-list body.car recur)
            swap case-exp-c]

           [head ': eq-p]
           [body.car recur
            body.cdr {recur} list-map
            colon-exp-c]

           [head ':: eq-p]
           [body.car recur
            body.cdr {recur} list-map
            double-colon-exp-c]

           else error)]))
    (+fun string-parse-exp
      : (-> string : string-u -- exp-u)
      (cond
        [string 'apply eq-p]
        [apply-exp-c]

        [string 'type-tt eq-p]
        [type-tt-exp-c]

        [string ', eq-p]
        [comma-exp-c]

        [string field-string-p]
        [string field-string->field-name field-exp-c]

        [string name-string-p]
        [string call-exp-c]

        else
        [error]))


    (+fun field-string-p
      : (-> string-u -- bool-u)
      string-head '. eq-p)

    (+fun field-string->field-name
      : (-> string-u -- string-u)
      string-tail)

    (+fun name-string-p
      : (-> string : string-u -- bool-u)
      string '. string-member-p not)
    (+fun top-sexp-list-eval
      : (-> env-t, sexp-list : [sexp-u list-u] -- env-t)
      (case sexp-list
        (null-t)
        (cons-t
          sexp-list.car top-sexp-eval
          sexp-list.cdr recur)))
    (+fun top-sexp-eval
      : (-> env-t, sexp : sexp-u -- env-t)
      (cond
        [sexp sexp-den-p]
        [sexp parse-den den-define]
        else [sexp parse-exp exp-run]))
    (+fun den-define
      : (-> env-t, den : den-u -- env-t)
      (case den
        (fun-den-t den.name den name-dict-insert)
        (type-cons-den-t den.name den name-dict-insert
          den.name
          dup string-length dec dec
          0 swap string-slice
          "-c" string-append
          den.type-arrow-exp
          den.cons-arrow-exp
          data-cons-den-c
          dup .name swap name-dict-insert)
        (union-cons-den-t den.name den name-dict-insert)))
    (+fun sexp-den-p
      : (-> sexp : sexp-u -- bool-u)
      (and [sexp cons-p]
           (or [sexp.car '+fun eq-p]
               [sexp.car '+type eq-p]
               [sexp.car '+union eq-p])))
    (+macro cicada-language
      (-> body : [sexp-u list-u] -- sexp-u)
      `(begin
         new-env (quote (@ body))
         sexp-list-pass
         top-sexp-list-eval))
    (+disp p [obj-u]
      (let obj)
      (case obj
        (data-obj-t
          obj.field-obj-dict p " " p
          obj.data-type.name
          dup string-length 2 sub string-take p "-c" p
          " : " p
          obj.data-type p)
        (data-type-t
          obj.field-obj-dict p " " p obj.name p)
        (union-type-t
          obj.field-obj-dict p " " p obj.name p)
        (type-type-t
          (if [obj.level 1 eq-p]
            ["type-tt" p]
            ["type-<" p obj.level p ">" p]))
        (closure-obj-t obj w)
        (arrow-type-t obj w)
        (data-hypo-t obj w)
        (type-hypo-t obj w)))

    (+fun field-obj-dict-print
      {(let field-name obj)
       field-name p " = " p obj p}
      dict-for-each)


    (assert
      1 2
      : (-> num0 : number-t, num1 : number-t -- number-t)
      num0 num1 add
      3 eq-p)
    (begin
      new-env
      "1" 1 name-dict-insert
      "2" 2 name-dict-insert
      "1" name-dict-get 1 eq-p bool-assert
      "1" name-dict-get 1 eq-p bool-assert
      "2" name-dict-get 2 eq-p bool-assert
      "2" name-dict-get 2 eq-p bool-assert
      drop)
    (begin
      new-env
      0 data-stack-push
      1 data-stack-push
      2 data-stack-push
      3 data-stack-push
      data-stack-pop 3 eq-p bool-assert
      data-stack-pop 2 eq-p bool-assert
      data-stack-tos 1 eq-p bool-assert
      data-stack-tos 1 eq-p bool-assert
      data-stack-tos 1 eq-p bool-assert
      data-stack-drop
      data-stack-pop 0 eq-p bool-assert
      drop)

    (begin
      new-env
      0 data-stack-push
      1 data-stack-push
      2 data-stack-push
      3 data-stack-push
      3 data-stack-n-pop
      (lit-list 1 2 3) eq-p bool-assert
      data-stack-pop 0 eq-p bool-assert
      drop)

    (begin
      new-env
      (lit-list 1 2 3) data-stack-list-push
      data-stack-pop 3 eq-p bool-assert
      data-stack-pop 2 eq-p bool-assert
      data-stack-pop 1 eq-p bool-assert
      drop)
    (begin
      new-env
      0 scope-stack-push
      1 scope-stack-push
      2 scope-stack-push
      3 scope-stack-push
      scope-stack-pop 3 eq-p bool-assert
      scope-stack-pop 2 eq-p bool-assert
      scope-stack-tos 1 eq-p bool-assert
      scope-stack-tos 1 eq-p bool-assert
      scope-stack-tos 1 eq-p bool-assert
      scope-stack-drop
      scope-stack-pop 0 eq-p bool-assert
      drop)
    (begin
      new-env
      "1" 1 data-bind-dict-insert
      "2" 2 data-bind-dict-insert
      "1" data-bind-dict-find bool-assert 1 eq-p bool-assert
      "1" data-bind-dict-find bool-assert 1 eq-p bool-assert
      "2" data-bind-dict-find bool-assert 2 eq-p bool-assert
      "2" data-bind-dict-find bool-assert 2 eq-p bool-assert
      drop)

    (begin
      new-env
      "1" 1 type-bind-dict-insert
      "2" 2 type-bind-dict-insert
      "1" type-bind-dict-find bool-assert 1 eq-p bool-assert
      "1" type-bind-dict-find bool-assert 1 eq-p bool-assert
      "2" type-bind-dict-find bool-assert 2 eq-p bool-assert
      "2" type-bind-dict-find bool-assert 2 eq-p bool-assert
      drop)
    (assert
      '((+union nat-u : type-tt
          (-> -- zero-t)
          (-> prev : nat-u -- succ-t)))
      sexp-list-pass
      '((+union (: nat-u type-tt)
          (arrow () (zero-t))
          (arrow ((: prev nat-u)) (succ-t))))
      eq-p)

    (assert
      '((+fun nat-add : (-> [m n] : nat-u -- nat-u)
          (case n
            (zero-t m)
            (succ-t m n.prev recur succ-c)))

        (+fun nat-mul : (-> [m n] : nat-u -- nat-u)
          (case n
            (zero-t n)
            (succ-t m n.prev recur m nat-add))))
      sexp-list-pass
      '((+fun (: nat-add
                 (arrow ((: m nat-u) (: n nat-u))
                        (nat-u)))
          (case n
            (zero-t m)
            (succ-t m n .prev nat-add succ-c)))

        (+fun (: nat-mul
                 (arrow ((: m nat-u) (: n nat-u))
                        (nat-u)))
          (case n
            (zero-t n)
            (succ-t m n .prev nat-mul m nat-add))))
      eq-p)
    (assert
      '((case n
          (zero-t n)
          (succ-t m n.prev nat-mul m nat-add)))
      sexp-list-pass
      {parse-exp} list-map
      (lit-list
       (lit-list "n" call-exp-c)
       (lit-dict
        "succ-t"
        (lit-list
         "m" call-exp-c
         "n" call-exp-c
         "prev" field-exp-c
         "nat-mul" call-exp-c
         "m" call-exp-c
         "nat-add" call-exp-c)
        closure-exp-c,
        "zero-t"
        (lit-list
         "n" call-exp-c)
        closure-exp-c)
       case-exp-c)
      eq-p)
    #note
    (begin
      '((+fun nat-add : (-> [m n] : nat-u -- nat-u)
          (case n
            (zero-t m)
            (succ-t m n.prev recur succ-c)))

        (+fun nat-mul : (-> [m n] : nat-u -- nat-u)
          (case n
            (zero-t n)
            (succ-t m n.prev recur m nat-add)))

        (+fun nat-factorial : (-> n : nat-u -- nat-u)
          (case n
            (zero-t n succ-c)
            (succ-t n.prev recur n nat-mul))))
      sexp-list-pass
      {parse-den} list-map)
    (cicada-language
      (+union bool-u : type-tt
        true-t
        false-t)
      (+type true-t : type-tt
        (-> -- true-t))
      (+type false-t : type-tt
        (-> -- false-t))
      true-c
      false-c
      true-t
      bool-u
      type-tt)

    p nl

    print-the-stack

