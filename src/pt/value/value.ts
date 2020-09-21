import * as Closure from "../closure"

export type Value = fn | str | pattern | gr

interface fn {
  kind: "Value.fn"
  ret_cl: Closure.Closure
}

export const fn = (ret_cl: Closure.Closure): fn => ({
  kind: "Value.fn",
  ret_cl,
})

interface str {
  kind: "Value.str"
  value: string
}

export const str = (value: string): str => ({
  kind: "Value.str",
  value,
})

interface pattern {
  kind: "Value.pattern"
  label: string
  value: RegExp
}

export const pattern = (label: string, value: RegExp): pattern => ({
  kind: "Value.pattern",
  label,
  value,
})

interface gr {
  kind: "Value.gr"
  name: string
  choices: Map<string, Array<Part>>
}

type Part = {
  name?: string
  value: Value
}

export const gr = (name: string, choices: Map<string, Array<Part>>): gr => ({
  kind: "Value.gr",
  name,
  choices,
})