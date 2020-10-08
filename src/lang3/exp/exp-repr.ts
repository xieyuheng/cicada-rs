import * as Exp from "../exp"
import * as Stmt from "../stmt"
import * as ut from "../../ut"

export function repr(exp: Exp.Exp): string {
  switch (exp.kind) {
    case "Exp.v": {
      return exp.name
    }
    case "Exp.pi": {
      return `(${exp.name}: ${Exp.repr(exp.arg_t)}) -> ${Exp.repr(exp.ret_t)}`
    }
    case "Exp.fn": {
      return `(${exp.name}) => ${Exp.repr(exp.ret)}`
    }
    case "Exp.ap": {
      return `${Exp.repr(exp.target)}(${Exp.repr(exp.arg)})`
    }
    case "Exp.cls": {
      const s = exp.scope
        .map(({ name, t }) => `${name} : ${Exp.repr(t)}`)
        .join("\n")
      return `{\n${ut.indent(s, "  ")}\n}`
    }
    case "Exp.fill": {
      return `${Exp.repr(exp.target)}[${Exp.repr(exp.arg)}]`
    }
    case "Exp.obj": {
      const s = Array.from(exp.properties)
        .map(([ name, exp ]) => `${name} = ${Exp.repr(exp)}`)
        .join("\n")
      return `{\n${ut.indent(s, "  ")}\n}`
    }
    case "Exp.dot": {
      throw new Error("TODO")
    }
    case "Exp.equal": {
      return `Equal(${Exp.repr(exp.t)}, ${Exp.repr(exp.from)}, ${Exp.repr(
        exp.to
      )})`
    }
    case "Exp.same": {
      return "same"
    }
    case "Exp.replace": {
      return `replace(${Exp.repr(exp.target)}, ${Exp.repr(
        exp.motive
      )}, ${Exp.repr(exp.base)})`
    }
    case "Exp.absurd": {
      return "Absurd"
    }
    case "Exp.absurd_ind": {
      return `Absurd.ind(${Exp.repr(exp.target)}, ${Exp.repr(exp.motive)})`
    }
    case "Exp.str": {
      return "String"
    }
    case "Exp.quote": {
      return `"${exp.str}"`
    }
    case "Exp.type": {
      return "Type"
    }
    case "Exp.suite": {
      const s = [...exp.stmts.map(Stmt.repr), repr(exp.ret)].join("\n")
      return `{\n${ut.indent(s, "  ")}\n}`
    }
    case "Exp.the": {
      return `${Exp.repr(exp.exp)}: ${Exp.repr(exp.t)}`
    }
  }
}