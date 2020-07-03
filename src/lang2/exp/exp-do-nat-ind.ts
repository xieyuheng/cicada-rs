import * as Exp from "../exp"
import * as Env from "../env"
import * as Value from "../value"
import * as Normal from "../normal"
import * as Closure from "../closure"
import * as Trace from "../../trace"

export function do_nat_ind(
  target: Value.Value,
  motive: Value.Value,
  base: Value.Value,
  step: Value.Value
): Value.Value {
  switch (target.kind) {
    case "Value.Zero": {
      return base
    }
    case "Value.Succ": {
      return Exp.do_ap(
        Exp.do_ap(step, target.prev),
        Exp.do_nat_ind(target.prev, motive, base, step)
      )
    }
    case "Value.Reflection": {
      switch (target.t.kind) {
        case "Value.Nat": {
          const motive_t: Value.Pi = {
            kind: "Value.Pi",
            arg_t: { kind: "Value.Nat" },
            closure: new Closure.Closure(Env.init(), "k", { kind: "Exp.Type" }),
          }
          const base_t = Exp.do_ap(motive, { kind: "Value.Zero" })
          const step_t = Exp.nat_ind_step_type(motive)
          return {
            kind: "Value.Reflection",
            t: Exp.do_ap(motive, target),
            neutral: {
              kind: "Neutral.NatInd",
              target: target.neutral,
              motive: new Normal.Normal(motive_t, motive),
              base: new Normal.Normal(base_t, base),
              step: new Normal.Normal(step_t, step),
            },
          }
        }
        default: {
          throw new Trace.Trace(
            Exp.explain_elim_target_type_mismatch({
              elim: "nat_ind",
              expecting: ["Value.Nat"],
              reality: target.t.kind,
            })
          )
        }
      }
    }
    default: {
      throw new Trace.Trace(
        Exp.explain_elim_target_mismatch({
          elim: "nat_ind",
          expecting: ["Value.Zero", "Value.Succ", "Value.Reflection"],
          reality: target.kind,
        })
      )
    }
  }
}

export function nat_ind_step_type(motive: Value.Value): Value.Value {
  const env = Env.extend(Env.init(), "motive", motive)

  const exp: Exp.Pi = {
    kind: "Exp.Pi",
    name: "prev",
    arg_t: { kind: "Exp.Nat" },
    ret_t: {
      kind: "Exp.Pi",
      name: "almost",
      arg_t: {
        kind: "Exp.Ap",
        target: { kind: "Exp.Var", name: "motive" },
        arg: { kind: "Exp.Var", name: "prev" },
      },
      ret_t: {
        kind: "Exp.Ap",
        target: { kind: "Exp.Var", name: "motive" },
        arg: { kind: "Exp.Succ", prev: { kind: "Exp.Var", name: "prev" } },
      },
    },
  }
  const t = Exp.evaluate(env, exp)
  return t
}
