import * as Exp from "../exp"
import { Obj } from "../../ut"
import * as ut from "../../ut"

export function build(present: Exp.Present): Exp.Exp {
  if (typeof present === "string") return from_string(present)
  else if (present instanceof Array) return from_array(present)
  else return from_object(present)
}

function from_string(str: string): Exp.Exp {
  return Exp.str(str)
}

// NOTE array is like function application syntax in common lisp.
// - the head is in function space.
// - when the head is a string, it is viewed as a variable.
function build_head(head: Exp.Present): Exp.Exp {
  if (typeof head === "string") {
    return Exp.v(head)
  } else {
    return build(head)
  }
}

function from_array(array: Array<any>): Exp.Exp {
  if (array.length === 1) {
    return build_head(array[0])
  } else if (array.length > 1) {
    const [target, ...args] = array
    return Exp.ap(build_head(target), args.map(build))
  } else {
    throw new Error(
      `array.length must be >= 1.\n` + `array.length: ${array.length}\n`
    )
  }
}

function from_object(obj: Obj<any>): Exp.Exp {
  if (obj.hasOwnProperty("$fn")) {
    const [name, ret] = obj["$fn"]
    return Exp.fn(name, build(ret))
  } else if (obj.hasOwnProperty("$pattern")) {
    const [label, value] = obj["$pattern"].split(":")
    return Exp.pattern(label, new RegExp(value))
  } else {
    return build_grammar(obj)
    throw new Error()
  }
}

function build_grammar(obj: Obj<any>): Exp.Exp {
  let name: string | undefined = undefined
  let choices = new Map()

  for (const [key, parts] of Object.entries(obj)) {
    const [grammar_name, choice_name] = key.split(":")

    if (name && name !== grammar_name) {
      throw new Error(
        `ambiguous grammar name.\n` +
          `new name: ${grammar_name}\n` +
          `old name: ${name}\n`
      )
    } else {
      name = grammar_name
    }

    choices.set(choice_name, parts.map(build_part))
  }

  if (name) {
    return Exp.grammar(name, choices)
  } else {
    throw new Error(`can not find grammar name from obj: ${ut.inspect(obj)}`)
  }
}

function build_part(part: any): { name?: string; value: Exp.Exp } {
  const result = parse_bind(part)
  if (result) {
    const [name, present] = result
    // NOTE a string in bind is special, it is always Exp.v -- instead of Exp.str.
    const value = typeof present === "string" ? Exp.v(present) : build(present)
    return { name, value }
  } else {
    return { value: build(part) }
  }
}

function parse_bind(present: Exp.Present): null | [string, Exp.Present] {
  if (typeof present === "string") return null
  if (present instanceof Array) return null
  const keys = Object.keys(present)
  if (keys.length !== 1) return null
  const key = keys[0]
  if (key.startsWith("$")) return null
  if (key.includes(":")) return null
  return [key, present[key]]
}
