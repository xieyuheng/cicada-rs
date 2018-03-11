    (+fun :)
    (+macro -> (let body)
      )
    (+macro +type)
    (+type env-t
      name-dict : [string-t den-u dict-t]
      data-stack : [obj-u list-u]
      frame-stack : [frame-u list-u]
      scope-stack : [string-t obj-u dict-t list-u]
      goal-stack : [goal-t list-u]
      data-bind-dict : [hypo-id-t obj-u dict-t]
      type-bind-dict : [hypo-id-t obj-u dict-t])
      (+fun name-dict/get
        : (-> env-t, name : string-t -- env-t den-u)
        dup .name-dict name dict/get)
      (+fun data-stack/push
        : (-> env : env-t, obj : obj-u -- env-t)
        obj env.data-stack cons
        (. data-stack)
        env clone)
      (+fun data-stack/pop
        : (-> env : env-t -- env-t obj-u)
        env.data-stack.cdr
        (. data-stack)
        env clone
        env.data-stack.car)
      (+fun data-stack/drop
        : (-> env-t -- env-t)
        data-stack/pop drop)
      (+fun data-stack/tos
        : (-> env-t -- env-t obj-u)
        dup .data-stack.car)
      (+fun data-stack/n-pop
        : (-> env-t, number : number-t
           -- env-t, obj-u list-u)
        )
      (+fun data-stack/list-push
        : (-> env-t obj-u list-u
           -- env-t)
        )
      (+fun frame-stack/push
        : (-> env : env-t, frame : frame-u -- env-t)
        frame env.frame-stack cons
        (. frame-stack)
        env clone)
      (+fun frame-stack/pop
        : (-> env : env-t -- env-t frame-u)
        env.frame-stack.cdr
        (. frame-stack)
        env clone
        env.frame-stack.car)
      (+fun frame-stack/drop
        : (-> env-t -- env-t)
        frame-stack/pop drop)
      (+fun frame-stack/tos
        : (-> env-t -- env-t frame-u)
        dup .frame-stack.car)
      (+union frame-u
        scoping-frame-t
        simple-frame-t)
      (+type scoping-frame-t
        body-exp-list : [exp-u list-u]
        index : number-t)
      (+fun new/scoping-frame
        : (-> body-exp-list : [exp-u list-u] -- scoping-frame-t)
        body-exp-list 0
        (. body-exp-list index)
        scoping-frame-cr)
      (+type simple-frame-t
        body-exp-list : [exp-u list-u]
        index : number-t)
      (+fun new/simple-frame
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
      (+fun scope/get dict/get)
      (+fun scope/insert
        : (-> string-t obj-u dict-t
              local-name : string-t
              obj : obj-u
           -- string-t obj-u dict-t)
        (dict local-name obj)
        dict-update)
      (+fun current-scope/insert
        : (-> string-t obj-u dict-t
              local-name : string-t
              obj : obj-u
           -- string-t obj-u dict-t)
      scope-stack/pop
      name hypo-obj scope/insert
      scope-stack/push)
      (+fun new/scope
        : (-> -- string-t obj-u dict-t)
        (dict))
    (+union exp-u
      call-exp-t
      let-exp-t
      closure-exp-t
      arrow-exp-t
      apply-exp-t
      case-exp-t
      construct-exp-t
      field-exp-t
      colon-exp-t
      double-colon-exp-t)
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
    (+type construct-exp-t
      type-name : string-t)
    (+type field-exp-t
      field-name : string-t)
    (+type colon-exp-t
      local-name : string-t
      type-exp-list : [exp-u list-u])
    (+type double-colon-exp-t
      local-name : string-t
      type-exp-list : [exp-u list-u])
    (+union den-u
      fun-den-t
      type-den-t
      union-den-t)
    (+type fun-den-t
      fun-name : string-t
      type-arrow-exp : arrow-exp-t
      body-exp-list : [exp-u list-u])
    (+type type-den-t
      type-name : string-t
      type-arrow-exp : arrow-exp-t
      cons-arrow-exp : arrow-exp-t)
    (+type union-den-t
      union-name : string-t
      type-arrow-exp : arrow-exp-t
      type-name-list : [string-t list-u])
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
      type-name : string-t
      field-obj-dict : [string-t obj-u dict-t])
    (+type union-type-t
      union-name : string-t
      field-obj-dict : [string-t obj-u dict-t])
    (+type type-type-t
      level : number-t)
    (+type closure-obj-t
      scope : [string-t obj-u dict-t]
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
        (call-exp-t call-exp/exe)
        (let-exp-t let-exp/exe)
        (closure-exp-t closure-exp/exe)
        (arrow-exp-t arrow-exp/exe)
        (apply-exp-t apply-exp/exe)
        (case-exp-t case-exp/exe)
        (construct-exp-t construct-exp/exe)
        (field-exp-t field-exp/exe)
        (colon-exp-t colon-exp/exe)
        (double-colon-exp-t double-colon-exp/exe)))
    (+fun call-exp/exe
      : (-> env-t, exp : call-exp-t -- env-t)
      exp.name name-dict/get den-exe)
      (+fun den-exe
        : (-> env-t den-u -- env-t)
        (case dup
          (fun-den-t fun-den/den-exe)
          (type-den-t type-den/den-exe)
          (union-den-t union-den/den-exe)))
      (+fun fun-den/den-exe
        : (-> env-t, den : fun-den-t -- env-t)
        new/scope scope-stack/push
        den.type-arrow-exp arrow-exp/extend-scope
        den.type-arrow-exp.ante-exp-list ante-exp-list/pick-up
        den.body-exp-list new/scoping-frame frame-stack/push)
      (+fun arrow-exp/extend-scope
        : (-> env-t, arrow-exp-t -- env-t)
        collect-one drop)
      (+fun ante-exp-list/pick-up
        : (-> env-t, ante-exp-list : [exp-u list-u] -- env-t)
        ><><><)
      (+fun type-den/den-exe
        : (-> env-t, den : type-den-t -- env-t)
        den.type-arrow-exp.ante-exp-list new/field-obj-dict
        den.type-name
        (. field-obj-dict type-name)
        data-type-cr data-stack/push)
      (+fun union-den/den-exe
        : (-> env-t, den : union-den-t -- env-t)
        den.type-arrow-exp.ante-exp-list new/field-obj-dict
        den.union-name
        (. field-obj-dict union-name)
        union-type-cr data-stack/push)
      (+fun new/field-obj-dict
        : (-> env-t
              ante-exp-list : [exp-u list-u]
           -- env-t, string-t obj-u dict-t)
        new/dict ante-exp-list
        ante-exp-list/merge-fields)
      (+fun ante-exp-list/merge-fields
        : (-> env-t
              field-obj-dict : [string-t obj-u dict-t]
              ante-exp-list : [exp-u list-u]
           -- env-t, string-t obj-u dict-t)
        (case ante-exp-list
          (null-t)
          (cons-t
            (case ante-exp-list.car
              (colon-exp-t
                field-obj-dict
                ante-exp-list.car.local-name
                data-stack/pop dict/insert
                ante-exp-list.cdr recur)
              (else
                field-obj-dict
                ante-exp-list.cdr recur)))))
    (+fun let-exp/exe
      : (-> env-t, exp : let-exp-t -- env-t)
      exp.local-name-list list-reverse
      let-exp/exe/loop)
    (+fun let-exp/exe/loop
      : (-> env-t, local-name-list : [string-t list-u] -- env-t)
      (case local-name-list
        (null-t)
        (cons-t
          data-stack/pop (let obj)
          scope-stack/pop
          local-name-list.car obj scope/insert
          scope-stack/push
          local-name-list.cdr recur)))
    (+fun closure-exp/exe
      : (-> env-t, exp : closure-exp-t -- env-t)
      scope-stack/tos
      exp.body-exp-list
      (. scope body-exp-list)
      closure-obj-cr
      data-stack/push)
    (+fun arrow-exp/exe
      : (-> env-t, exp : arrow-exp-t -- env-t)
      ;; calling collect-list
      ;;   might effect current scope
      exp.ante-exp-list collect-list (let ante-type-list)
      exp.succ-exp-list collect-list (let succ-type-list)
      (. ante-type-list succ-type-list)
      arrow-type-cr
      data-stack/push)
    (+fun apply-exp/exe
      : (-> env-t, exp : apply-exp-t -- env-t)
      data-stack/pop (let obj)
      (case obj
        (closure-obj-t
          obj.scope scope-stack/push
          obj.body-exp-list new/scoping-frame frame-stack/push)))
    (+fun case-exp/exe
      : (-> env-t, exp : case-exp-t -- env-t)
      ;; calling collect-one
      ;;   might effect current scope
      exp.arg-exp-list collect-one (let obj)
      (case obj
        (data-obj-t
          exp.closure-exp-dict
          obj.data-type.type-name dict/get
          closure-exp/exe
          apply-exp/exe)))
    (+fun construct-exp/exe
      : (-> env-t, exp : construct-exp-t -- env-t)
      exp.type-name name-dict/get (let den)
      den.type-arrow-exp arrow-exp/extend-scope
      den.cons-arrow-exp.succ-exp-list collect-one (let return-type)
      den.cons-arrow-exp.ante-exp-list new/field-obj-dict
      return-type
      (. field-obj-dict type)
      data-obj-cr data-stack/push)
    (+fun field-exp/exe
      : (-> env-t, exp : field-exp-t -- env-t)
      data-stack/pop (let obj)
      (case obj
        (data-obj-t
          obj.field-obj-dict
          exp.field-name dict/get)))
    (+fun colon-exp/exe
      : (-> env-t, exp : colon-exp-t -- env-t)
      exp.type-exp-list collect-one (let type)
      exp.local-name generate/hypo-id (let hypo-id)
      hypo-id type-hypo-c
      type type-hypo/insert
      exp.local-name hypo-id data-hypo-c current-scope/insert
      type data-stack/push)
    (+fun double-colon-exp/exe
      : (-> env-t double-colon-exp-t -- env-t)
      colon-exp/exe
      data-stack/drop)
    (+type counter-t
      .number : number-t)
    (+fun new/counter
      : (-> -- counter-t)
      0 counter-c)
    (+fun counter/inc
      : (-> counter-t --)
      swap .number!)
    (+fun counter/number
      : (-> counter-t -- number-t)
      .number)
    (+var hypo-id/counter new/counter)

    (+fun generate/hypo-id
      : (-> env-t, base-name : string-t
         -- env-t, hypo-id-t)
      hypo-id/counter counter/number repr (let postfix)
      hypo-id/counter counter/inc
      base-name postfix string-append hypo-id-c)
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
    (+fun exp-list/run
      : (-> env-t, exp-list : [exp-u list-u] -- env-t)
      dup .frame-stack list-length (let base)
      exp-list new/simple-frame frame-stack/push
      base run-with-base)
    (+fun collect-list
      : (-> env-t, exp-list : [exp-u list-u]
         -- env-t, obj-u list-u)
      dup .data-stack list-length (let old)
      exp-list exp-list/run
      dup .data-stack list-length (let new)
      new old number-sub data-stack/n-pop)
    (+fun collect-one
      : (-> env-t, exp-list : [exp-u list-u]
         -- env-t, obj-u)
      exp-list exp-list/run
      data-stack/pop)
    (+fun cut
      : (-> env-t exp-u -- env-t)
      (case dup
        (call-exp-t call-exp/cut)
        (let-exp-t let-exp/cut)
        (closure-exp-t closure-exp/cut)
        (arrow-exp-t arrow-exp/cut)
        (apply-exp-t apply-exp/cut)
        (case-exp-t case-exp/cut)
        (construct-exp-t construct-exp/cut)
        (field-exp-t field-exp/cut)
        (colon-exp-t colon-exp/cut)
        (double-colon-exp-t double-colon-exp/cut)))
    (+fun call-exp/cut
      : (-> env-t, exp : call-exp-t -- env-t)
      exp.name name-dict/get den-cut)
      (+fun den-cut
        : (-> env-t den-u -- env-t)
        (case dup
          (fun-den-t fun-den/den-cut)
          (type-den-t type-den/den-cut)
          (union-den-t union-den/den-cut)))
      (+fun fun-den/den-cut
        : (-> env-t, den : fun-den-t -- env-t)
        den.type-arrow-exp arrow-exp/cut-apply)
      (+fun arrow-exp/cut-apply
        : (-> env-t, arrow-exp : arrow-exp-t -- env-t)
        ;; must create a new scope
        ;;   before creating an arrow-type
        ;; because creating an arrow-type
        ;;   might effect current scope
        new/scope scope-stack/push
        arrow-exp collect-one (let arrow-type)
        arrow-type.ante-type-list ante-type-list/unify
        arrow-type.succ-type-list data-stack/list-push
        scope-stack/drop)
      (+fun ante-type-list/unify
        : (-> env-t, ante-type-list : [obj-u list-u] -- env-t)
        )
      (+fun type-den/den-cut
        : (-> env-t, den : type-den-t -- env-t)
        )
      (+fun union-den/den-cut
        : (-> env-t, den : union-den-t -- env-t)
        )
    (+fun infer
      : (-> env-t obj-u -- obj-u env-t)
      (case dup
        (data-obj-t data-obj/infer)
        (closure-obj-t closure-obj/infer)
        ;; ><><><
        (obj-u type-infer)))

