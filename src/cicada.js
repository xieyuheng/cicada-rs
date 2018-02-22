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
      todo-dict : [string-t todo-u dict-t])
      (+fun name-dict/get
        : (-> env-t (: name string-t) -- env-t den-u)
        dup .name-dict name dict/get)
      (+fun data-stack/push
        : (-> (: env env-t) (: obj obj-u) -- env-t)
        (. data-stack = [obj env.data-stack cons])
        env clone)
      (+fun data-stack/pop
        : (-> (: env env-t) -- env-t obj-u)
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
        : (-> env-t (: number nat-u)
           -- env-t, obj-u list-u)
        )
      (+fun data-stack/list-push
        : (-> env-t obj-u list-u
           -- env-t)
        )
      (+fun frame-stack/push
        : (-> (: env env-t) (: frame frame-u) -- env-t)
        (. frame-stack = [frame env.frame-stack cons])
        env clone)
      (+fun frame-stack/pop
        : (-> (: env env-t) -- env-t frame-u)
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
        : (-> (: body-exp-list exp-u list-u) -- scoping-frame-t)
        (. body-exp-list = body-exp-list
           index = 0)
        scoping-frame-cr)
      (+data simple-frame-t
        body-exp-list : [exp-u list-u]
        index : nat-u)
      (+fun new/simple-frame
        : (-> (: body-exp-list exp-u list-u) -- simple-frame-t)
        (. body-exp-list = body-exp-list
           index = 0)
        simple-frame-cr)
      (+fun scope/get dict/get)
      (+fun scope/set
        : (-> string-t obj-u dict-t
              (: local-name string-t)
              (: obj obj-u)
           -- string-t obj-u dict-t)
        (dict local-name obj)
        dict-update)
      (+fun new/scope
        : (-> -- string-t obj-u dict-t)
        (dict))
      (+fun todo-dict/get
        : (-> env-t (: id string-t) -- env-t todo-u)
        dup .todu-dict id dict/get)
      (+fun todo-dict/set
        : (-> env-t (: id string-t) (: todo todo-u) -- env-t)
        )
    (+union exp-u
      call-exp-t
      let-exp-t
      closure-exp-t
      arrow-exp-t
      apply-exp-t
      case-exp-t
      sum-exp-t
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
    (+data sum-exp-t
      sub-exp-list-list : [exp-u list-u list-u])
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
      #:with trunk-u
      #:with type-u)
    (+data data-obj-t
      data-type : data-type-t
      field-obj-dict : [string-t obj-u dict-t])
    (+data closure-obj-t
      scope : [string-t obj-u dict-t]
      body-exp-list : [exp-u list-u])
      (+union trunk-u
        var-trunk-t
        case-trunk-t
        apply-trunk-t
        field-trunk-t)
      ;; done-t
      ;; hypo-t
      ;; bound-t
      (+data var-trunk-t
        trunk-id : string-t)
      ;; done-t
      ;; case-todo-t
      (+data case-trunk-t
        trunk-id : string-t)
      ;; done-t
      ;; apply-todo-t
      (+data apply-trunk-t
        trunk-id : string-t)
      ;; done-t
      ;; field-todo-t
      (+data field-trunk-t
        trunk-id : string-t)
      (+union todo-u
        done-t
        hypo-t
        bound-t
        case-todo-t
        apply-todo-t
        field-todo-t)
      (+data done-t
        type : type-u
        obj : obj-u)
      (+data hypo-t
        type : type-u)
      (+data bound-t
        to : var-trunk-t)
      (+data case-todo-t
        sum-type : sum-type-t
        arg-trunk : trunk-u
        closure-obj-dict : [string-t closure-obj-t dict-t])
      (+data apply-todo-t
        type : type-u
        fun-trunk : trunk-u
        arg-obj-list : [obj-u list-u])
      (+data field-todo-t
        type : type-u
        arg-trunk : trunk-u
        field-name : string-t)
      (+union type-u
        data-type-t
        type-type-t
        arrow-type-t
        sum-type-t)
      (+data data-type-t
        type-name : string-t
        field-obj-dict : [string-t obj-u dict-t])
      (+data type-type-t
        level : nat-u)
      (+data arrow-type-t
        ante-type-list : [type-u list-u]
        succ-type-list : [type-u list-u])
      (+data sum-type-t
        sub-type-list : [type-u list-u])
    (+fun exe
      : (-> env-t exp-u -- env-t)
      (case dup
        (call-exp-t call-exp/exe)
        (let-exp-t let-exp/exe)
        (closure-exp-t closure-exp/exe)
        (arrow-exp-t arrow-exp/exe)
        (apply-exp-t apply-exp/exe)
        (case-exp-t case-exp/exe)
        (sum-exp-t sum-exp/exe)
        (construct-exp-t construct-exp/exe)
        (field-exp-t field-exp/exe)
        (colon-exp-t colon-exp/exe)
        (double-colon-exp-t double-colon-exp/exe)))
    (+fun call-exp/exe
      : (-> env-t (: exp call-exp-t) -- env-t)
      exp.name name-dict/get den-exe)
      (+fun den-exe
        : (-> env-t den-u -- env-t)
        (case dup
          (fun-den-t fun-den/den-exe)
          (type-den-t type-den/den-exe)
          (union-den-t union-den/den-exe)))
      (+fun fun-den/den-exe
        : (-> env-t (: den fun-den-t) -- env-t)
        new/scope scope-stack/push
        den.type-arrow-exp collect (let arrow-type)
        arrow-type.ante-type-list ante-type-list/prepare
        den.type-arrow-exp.ante-exp-list ante-exp-list/correspond
        den.body-exp-list new/scoping-frame frame-stack/push)
      (+fun ante-type-list/prepare
        : (-> env-t (: ante-type-list type-u list-u) -- env-t)
        )
      (+fun ante-exp-list/correspond
        : (-> env-t (: ante-exp-list exp-u list-u) -- env-t)
        )
      (+fun type-den/den-exe
        : (-> env-t (: den type-den-t) -- env-t)
        den.type-arrow-exp collect (let arrow-type)
        (. type-name = den.type-name
           field-obj-dict = )
        data-type-cr)

      ;; (+data data-type-t
      ;;   type-name : string-t
      ;;   field-obj-dict : [string-t obj-u dict-t])
      (+fun union-den/den-exe
        : (-> env-t (: den union-den-t) -- env-t)
        )
    (+fun let-exp/exe
      : (-> env-t (: exp let-exp-t) -- env-t)
      exp.local-name-list list-reverse
      let-exp/exe/loop)
    (+fun let-exp/exe/loop
      : (-> env-t (: local-name-list string-t list-u) -- env-t)
      (case local-name-list
        (null-t)
        (cons-t
          data-stack/pop (let obj)
          scope-stack/pop
          local-name-list.car obj scope/set
          scope-stack/push
          local-name-list.cdr recur)))
    (+fun closure-exp/exe
      : (-> env-t (: exp closure-exp-t) -- env-t)
      (. scope = scope-stack/tos
         body-exp-list = [exp.body-exp-list])
      closure-obj-cr
      data-stack/push)
    (+fun arrow-exp/exe
      : (-> env-t (: exp arrow-exp-t) -- env-t)
      ;; calling collect-list
      ;;   might effect current scope
      (. ante-type-list = [exp.ante-exp-list collect-list]
         succ-type-list = [exp.succ-exp-list collect-list])
      arrow-type-cr
      data-stack/push)
    (+fun apply-exp/exe
      : (-> env-t (: exp apply-exp-t) -- env-t)
      data-stack/pop obj/try (let obj)
      (case obj
        (closure-obj-t
          obj.scope scope-stack/push
          obj.body-exp-list new/scoping-frame frame-stack/push)
        (trunk-u
          (. type = [obj infer .succ-type-list .car]
             fun-trunk = obj
             arg-obj-list =
             [obj infer .ante-type-list list-length
              data-stack/n-pop])
          apply-todo-cr
          new/apply-trunk
          data-stack/push)))
    (+fun new/apply-trunk
      : (-> env-t (: apply-todo apply-todo-t)
         -- env-t apply-trunk-t)
      "apply" generate/trunk-id (let id)
      id apply-todo todo-dict/set
      id apply-trunk-c)
    (+fun obj/try
      : (-> env-t obj-u -- env-t obj-u)
      ;; after obj/try
      ;; if the obj is still a trunk
      ;;   we know it is not done
      ;; if the obj is var-trunk
      ;;   we also know it is not bound to a done var-trunk
      )
    (+fun case-exp/exe
      : (-> env-t (: exp case-exp-t) -- env-t)
      ;; calling collect
      ;;   might effect current scope
      exp.arg-exp-list collect (let obj)
      ;; no need to use obj/try on newly collected obj
      (case obj
        (data-obj-t
          exp.closure-exp-dict
          obj.data-type.type-name dict/get
          closure-exp/exe
          apply-exp/exe)
        (trunk-u
          exp.closure-exp-dict
          closure-exp-dict->closure-obj-dict
          (let closure-obj-dict)
          obj closure-obj-dict closure-obj-dict->sum-type
          (let sum-type)
          (. sum-type = sum-type
             arg-trunk = obj
             closure-obj-dict = closure-obj-dict)
          case-todo-cr
          new/case-trunk
          data-stack/push)))
    (+fun closure-exp-dict->closure-obj-dict
      : (-> env-t
            (: closure-exp-dict string-t closure-exp-t dict-t)
         -- env-t, string-t closure-obj-t dict-t)
      )
    (+fun closure-obj-dict->sum-type
      : (-> env-t
            (: obj obj-u)
            (: closure-obj-dict string-t closure-obj-t dict-t)
         -- env-t sum-type-t)
      )
    (+fun sum-exp/exe
      : (-> env-t (: exp sum-exp-t) -- env-t)
      )
    (+fun construct-exp/exe
      : (-> env-t (: exp construct-exp-t) -- env-t)
      exp.type-name name-dict/get (let type-den)
      type-den.cons-arrow-exp )
    (+fun field-exp/exe
      : (-> env-t (: exp field-exp-t) -- env-t)
      )
    (+fun generate/trunk-id
      : (-> env-t (: seed string-t)
         -- env-t, string-t)
      )
    (+fun colon-exp/exe
      : (-> env-t (: exp colon-exp-t) -- env-t)
      exp.type-exp-list collect (let type)
      (. trunk-id = [exp.local-name generate/trunk-id]
         type = type)
      var-trunk-cr (let var-trunk)
      scope-stack/pop
      exp.local-name var-trunk scope/set
      scope-stack/push
      type data-stack/push)
    (+fun double-colon-exp/exe
      : (-> env-t double-colon-exp-t -- env-t)
      colon-exp/exe
      data-stack/drop)
    (+fun collect-list
      : (-> env-t (: exp-list exp-u list-u)
         -- env-t obj-u list-u)
      )
    (+fun collect
      : (-> env-t (: exp-list exp-u list-u)
         -- env-t obj-u)
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
        (sum-exp-t sum-exp/cut)
        (construct-exp-t construct-exp/cut)
        (field-exp-t field-exp/cut)
        (colon-exp-t colon-exp/cut)
        (double-colon-exp-t double-colon-exp/cut)))
    (+fun call-exp/cut
      : (-> env-t (: exp call-exp-t)-- env-t)
      exp.name name-dict/get den-cut)
      (+fun den-cut
        : (-> env-t den-u -- env-t)
        (case dup
          (fun-den-t fun-den/den-cut)
          (type-den-t type-den/den-cut)
          (union-den-t union-den/den-cut)))
      (+fun fun-den/den-cut
        : (-> env-t (: den fun-den-t) -- env-t)
        ;; must create a new scope
        ;;   before creating an arrow-type
        ;; because creating an arrow-type
        ;;   might effect current scope
        new/scope scope-stack/push
        den.type-arrow-exp collect (let arrow-type)
        arrow-type.ante-type-list ante-type-list/unify
        arrow-type.succ-type-list data-stack/list-push
        scope-stack/drop)
      (+fun ante-type-list/unify
        : (-> env-t (: ante-type-list type-u list-u) -- env-t)
        )
      (+fun type-den/den-cut
        : (-> env-t (: den type-den-t) -- env-t)
        )
      (+fun union-den/den-cut
        : (-> env-t (: den union-den-t) -- env-t)
        )
    (+fun infer
      : (-> env-t obj-u -- type-u env-t)
      (case dup
        (data-obj-t data-obj/infer)
        (closure-obj-t closure-obj/infer)
        (trunk-u trunk/infer)
        (type-u type/infer)))
    (+fun trunk/infer
      : (-> env-t trunk-u -- type-u env-t)
      (case ))
