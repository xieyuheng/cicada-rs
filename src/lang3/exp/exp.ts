import * as Stmt from "../stmt"
import * as Pattern from "../pattern"
import { Var } from "../exps/var"
import { Pi } from "../exps/pi/pi"
import { Fn } from "../exps/pi/fn"
import { Ap } from "../exps/pi/ap"

export type Exp =
  | Var
  | Pi
  | Fn
  | case_fn
  | Ap
  | cls
  | obj
  | dot
  | equal
  | same
  | replace
  | absurd
  | absurd_ind
  | str
  | quote
  | union
  | type_constructor
  | type
  | begin
  | the

export type v = Var
export const v = Var
export type pi = Pi
export const pi = Pi
export type fn = Fn
export const fn = Fn

export type Case = {
  pattern: Pattern.Pattern
  ret: Exp
}

export type case_fn = {
  kind: "Exp.case_fn"
  cases: Array<Case>
}

export const case_fn = (cases: Array<Case>): case_fn => ({
  kind: "Exp.case_fn",
  cases,
})

export type ap = Ap
export const ap = Ap

export type cls = {
  kind: "Exp.cls"
  sat: Array<{ name: string; t: Exp; exp: Exp }>
  scope: Array<{ name: string; t: Exp }>
}

export const cls = (
  sat: Array<{ name: string; t: Exp; exp: Exp }>,
  scope: Array<{ name: string; t: Exp }>
): cls => ({
  kind: "Exp.cls",
  sat,
  scope,
})

export type obj = {
  kind: "Exp.obj"
  properties: Map<string, Exp>
}

export const obj = (properties: Map<string, Exp>): obj => ({
  kind: "Exp.obj",
  properties,
})

export type dot = {
  kind: "Exp.dot"
  target: Exp
  name: string
}

export const dot = (target: Exp, name: string): dot => ({
  kind: "Exp.dot",
  target,
  name,
})

export type equal = {
  kind: "Exp.equal"
  t: Exp
  from: Exp
  to: Exp
}

export const equal = (t: Exp, from: Exp, to: Exp): equal => ({
  kind: "Exp.equal",
  t,
  from,
  to,
})

export type same = {
  kind: "Exp.same"
}

export const same: same = {
  kind: "Exp.same",
}

export type replace = {
  kind: "Exp.replace"
  target: Exp
  motive: Exp
  base: Exp
}

export const replace = (target: Exp, motive: Exp, base: Exp): replace => ({
  kind: "Exp.replace",
  target,
  motive,
  base,
})

export type absurd = {
  kind: "Exp.absurd"
}

export const absurd: absurd = {
  kind: "Exp.absurd",
}

export type absurd_ind = {
  kind: "Exp.absurd_ind"
  target: Exp
  motive: Exp
}

export const absurd_ind = (target: Exp, motive: Exp): absurd_ind => ({
  kind: "Exp.absurd_ind",
  target,
  motive,
})

export type str = {
  kind: "Exp.str"
}

export const str: str = {
  kind: "Exp.str",
}

export type quote = {
  kind: "Exp.quote"
  str: string
}

export const quote = (str: string): quote => ({
  kind: "Exp.quote",
  str,
})

export type union = {
  kind: "Exp.union"
  left: Exp
  right: Exp
}

export const union = (left: Exp, right: Exp): union => ({
  kind: "Exp.union",
  left,
  right,
})

export type type_constructor = {
  kind: "Exp.type_constructor"
  name: string
  t: Exp
  sums: Array<{ tag: string; t: Exp }>
}

export const type_constructor = (
  name: string,
  t: Exp,
  sums: Array<{ tag: string; t: Exp }>
): type_constructor => ({
  kind: "Exp.type_constructor",
  name,
  t,
  sums,
})

export type type = {
  kind: "Exp.type"
}

export const type: type = {
  kind: "Exp.type",
}

export type begin = {
  kind: "Exp.begin"
  stmts: Array<Stmt.Stmt>
  ret: Exp
}

export const begin = (stmts: Array<Stmt.Stmt>, ret: Exp): begin => ({
  kind: "Exp.begin",
  stmts,
  ret,
})

export type the = {
  kind: "Exp.the"
  t: Exp
  exp: Exp
}

export const the = (t: Exp, exp: Exp): the => ({
  kind: "Exp.the",
  t,
  exp,
})
