      (+union bool-u
        true-t
        false-t)
      (+data true-t)
      (+data false-t)
      (+union nat-u
        zero-t
        succ-t)
      (+data zero-t)
      (+data succ-t
        prev : nat-u)




    (+data env-t
      name-dict : [string-t den-u dict-t]
      data-stack : [obj-u list-u]
      frame-stack : [frame-u list-u]
      scope-stack : [string-t obj-u dict-t list-u]
      goal-stack : [goal-t list-u]
      hypo-dict : [string-t hypo-u dict-t])
      (+fun name-dict/get
        : (-> env-t, name : string-t -- env-t den-u)
        dup .name-dict name dict/get)
      (+fun data-stack/push
        : (-> env : env-t, obj : obj-u -- env-t)
        (. data-stack = [obj env.data-stack cons])
        env clone)
      (+fun data-stack/pop
        : (-> env : env-t -- env-t obj-u)
        (. data-stack = [env.data-stack.cdr])
        env clone
        env.data-stack.car)
      (+fun data-stack/drop
        : (-> env-t -- env-t)
        data-stack/pop drop)
      (+fun data-stack/tos
        : (-> env-t -- env-t obj-u)
        dup .data-stack.car)
      (+fun data-stack/n-pop
        : (-> env-t, number : nat-u
           -- env-t, obj-u list-u)
        )
      (+fun data-stack/list-push
        : (-> env-t obj-u list-u
           -- env-t)
        )
      (+fun frame-stack/push
        : (-> env : env-t, frame : frame-u -- env-t)
        (. frame-stack = [frame env.frame-stack cons])
        env clone)
      (+fun frame-stack/pop
        : (-> env : env-t -- env-t frame-u)
        (. frame-stack = [env.frame-stack.cdr])
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
      (+data scoping-frame-t
        body-exp-list : [exp-u list-u]
        index : nat-u)
      (+fun new/scoping-frame
        : (-> body-exp-list : [exp-u list-u] -- scoping-frame-t)
        (. body-exp-list = body-exp-list
           index = 0)
        scoping-frame-cr)
      (+data simple-frame-t
        body-exp-list : [exp-u list-u]
        index : nat-u)
      (+fun new/simple-frame
        : (-> body-exp-list : [exp-u list-u] -- simple-frame-t)
        (. body-exp-list = body-exp-list
           index = 0)
        simple-frame-cr)
      (+fun scope/get dict/get)
      (+fun scope/set
        : (-> string-t obj-u dict-t
              local-name : string-t
              obj : obj-u
           -- string-t obj-u dict-t)
        (dict local-name obj)
        dict-update)
      (+fun new/scope
        : (-> -- string-t obj-u dict-t)
        (dict))
      (+fun hypo-dict/get
        : (-> env-t, id : string-t -- env-t hypo-u)
        dup .todu-dict id dict/get)
      (+fun hypo-dict/set
        : (-> env-t, id : string-t, hypo : hypo-u -- env-t)
        )
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
    (+data call-exp-t
      name : string-t)
    (+data let-exp-t
      local-name-list : [string-t list-u])
    (+data closure-exp-t
      body-exp-list : [exp-u list-u])
    (+data arrow-exp-t
      ante-exp-list : [exp-u list-u]
      succ-exp-list : [exp-u list-u])
    (+data apply-exp-t)
    (+data case-exp-t
      arg-exp-list : [exp-u list-u]
      closure-exp-dict : [string-t closure-exp-t dict-t])
    (+data construct-exp-t
      type-name : string-t)
    (+data field-exp-t
      field-name : string-t)
    (+data colon-exp-t
      local-name : string-t
      type-exp-list : [exp-u list-u])
    (+data double-colon-exp-t
      local-name : string-t
      type-exp-list : [exp-u list-u])
    (+union den-u
      fun-den-t
      type-den-t
      union-den-t)
    (+data fun-den-t
      fun-name : string-t
      type-arrow-exp : arrow-exp-t
      body-exp-list : [exp-u list-u])
    (+data type-den-t
      type-name : string-t
      type-arrow-exp : arrow-exp-t
      cons-arrow-exp : arrow-exp-t)
    (+data union-den-t
      union-name : string-t
      type-arrow-exp : arrow-exp-t
      type-name-list : [string-t list-u])
    (+union obj-u
      data-obj-t
      closure-obj-t
      hypo-obj-t
      #:with type-u)
    (+data data-obj-t
      data-type : data-type-t
      field-obj-dict : [string-t obj-u dict-t])
    (+data closure-obj-t
      scope : [string-t obj-u dict-t]
      body-exp-list : [exp-u list-u])
    (+data hypo-obj-t
      hypo-id : string-t)
      (+union hypo-u
        done-hypo-t
        todo-hypo-t
        bound-hypo-t)
      (+data done-hypo-t
        type : type-u
        obj : obj-u)
      (+data todo-hypo-t
        type : type-u)
      (+data bound-hypo-t
        to : hypo-obj-t)
      (+union type-u
        data-type-t
        type-type-t
        arrow-type-t)
      (+data data-type-t
        type-name : string-t
        field-obj-dict : [string-t obj-u dict-t])
      (+data type-type-t
        level : nat-u)
      (+data arrow-type-t
        ante-type-list : [type-u list-u]
        succ-type-list : [type-u list-u])
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
        den.type-arrow-exp.ante-exp-list
        ante-exp-list/pick-up
        den.body-exp-list new/scoping-frame frame-stack/push)
      (+fun arrow-exp/extend-scope
        : (-> env-t, arrow-exp-t -- env-t)
        collect drop)
      (+fun ante-exp-list/pick-up
        : (-> env-t, ante-exp-list : [exp-u list-u] -- env-t)
        ><><><)
      (+fun type-den/den-exe
        : (-> env-t, den : type-den-t -- env-t)
        (. type-name = den.type-name
           field-obj-dict =
           [new/dict den.type-arrow-exp.ante-exp-list
            ante-exp-list/merge-fields])
        data-type-cr data-stack/push)
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
                data-stack/pop dict/set
                ante-exp-list.cdr recur)
              (else
                field-obj-dict
                ante-exp-list.cdr recur)))))
      (+fun union-den/den-exe
        : (-> env-t, den : union-den-t -- env-t)
        )
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
          local-name-list.car obj scope/set
          scope-stack/push
          local-name-list.cdr recur)))
    (+fun closure-exp/exe
      : (-> env-t, exp : closure-exp-t -- env-t)
      (. scope = scope-stack/tos
         body-exp-list = [exp.body-exp-list])
      closure-obj-cr
      data-stack/push)
    (+fun arrow-exp/exe
      : (-> env-t, exp : arrow-exp-t -- env-t)
      ;; calling collect-list
      ;;   might effect current scope
      (. ante-type-list = [exp.ante-exp-list collect-list]
         succ-type-list = [exp.succ-exp-list collect-list])
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
      ;; calling collect
      ;;   might effect current scope
      exp.arg-exp-list collect (let obj)
      (case obj
        (data-obj-t
          exp.closure-exp-dict
          obj.data-type.type-name dict/get
          closure-exp/exe
          apply-exp/exe)))
    (+fun construct-exp/exe
      : (-> env-t, exp : construct-exp-t -- env-t)
      den.type-arrow-exp arrow-exp/extend-scope
      exp.type-name name-dict/get (let den)
      (. type =
         [den.cons-arrow-exp.succ-exp-list collect]
         field-obj-dict =
         [new/dict den.cons-arrow-exp.ante-exp-list
          ante-exp-list/merge-fields])
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
      exp.type-exp-list collect (let type)
      exp.local-name type new-hypo-in-scope
      type data-stack/push)
    (+fun new-hypo-in-scope
      : (-> env-t, name : string-t, type : type-u
         -- env-t)
      name generate/hypo-id hypo-obj-c (let hypo-obj)
      scope-stack/pop
      name hypo-obj scope/set
      scope-stack/push
      hypo-obj.hypo-id
      type todo-hypo-c
      hypo-dict/set)
    (+fun generate/hypo-id
      : (-> env-t, seed : string-t
         -- env-t, string-t)
      )
    (+fun double-colon-exp/exe
      : (-> env-t double-colon-exp-t -- env-t)
      colon-exp/exe
      data-stack/drop)
    (+fun collect-list
      : (-> env-t, exp-list : [exp-u list-u]
         -- env-t, obj-u list-u)
      )
    (+fun collect
      : (-> env-t, exp-list : [exp-u list-u]
         -- env-t, obj-u)
      )
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
        arrow-exp collect (let arrow-type)
        arrow-type.ante-type-list ante-type-list/unify
        arrow-type.succ-type-list data-stack/list-push
        scope-stack/drop)
      (+fun ante-type-list/unify
        : (-> env-t, ante-type-list : [type-u list-u] -- env-t)
        )
      (+fun type-den/den-cut
        : (-> env-t, den : type-den-t -- env-t)
        )
      (+fun union-den/den-cut
        : (-> env-t, den : union-den-t -- env-t)
        )
    (+fun infer
      : (-> env-t obj-u -- type-u env-t)
      (case dup
        (data-obj-t data-obj/infer)
        (closure-obj-t closure-obj/infer)
        (hypo-obj-t hypo-obj/infer)
        (type-u type-infer)))
    (+fun hypo-obj/infer
      : (-> env-t hypo-obj-t -- type-u env-t)
      (case ))
