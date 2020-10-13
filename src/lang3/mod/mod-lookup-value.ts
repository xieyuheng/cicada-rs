import * as Mod from "../mod"
import * as Env from "../env"
import * as Exp from "../exp"
import * as Value from "../value"

export function lookup_value(
  mod: Mod.Mod,
  name: string
): undefined | Value.Value {
  const entry = Mod.lookup_entry(mod, name)
  if (entry === undefined) return undefined
  if (entry.value_cache !== undefined) return entry.value_cache
  // NOTE maybe use try and Trace
  return Exp.evaluate(mod, Env.init(), entry.exp)
}