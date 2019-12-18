# semantics
- use the concept of **free variable proof** to explain the semantics of cicada
  (traditional type theoretical semantics)
# partech
- clean up partech
- good error report for earley
  like: https://github.com/kach/nearley/issues/451
# cicada
- [cicada] use exception instead of `Either`
- [big change] `ctx` contains both value and type
- factor out `Telescope`
  - should we also store `ctx` in `Telescope`?
    which means we need to thread `ctx` as an argument in `eval`
  - we may also take this opportunity to handle currying
    be careful about equality
- after out `Telescope`
  - `Tl` should be called `Cl`
  - `Cl` should be called `ClInferedFromObj`
- application of class return type instead of object
- should we handle `A : type` specially?
  - I think no.
- use `new` to construct object from class
- equality constrain `equal` in telescope
  - syntax should be
    `constrain equal x = y`
  - to define `eqv_t` as class
  - to specify partial algebraic structure
- make `Top` consistent with other `Entry`s
  [maybe] other `Entry`s should be like `Top`
- [note] no auto currying
- [less important] Exp use cofree