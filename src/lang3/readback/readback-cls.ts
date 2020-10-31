import * as Readback from "../readback"
import * as Evaluate from "../evaluate"
import * as Value from "../value"
import * as Neutral from "../neutral"
import * as Exp from "../exp"
import * as Ctx from "../ctx"
import * as Env from "../env"
import * as Mod from "../mod"

export function readback_cls(
  mod: Mod.Mod,
  ctx: Ctx.Ctx,
  cls: Value.cls
): Exp.cls {
  ctx = Ctx.clone(ctx)
  // NOTE side-effect on ctx
  const norm_sat = readback_sat(mod, ctx, cls.sat)
  // NOTE `tel` has its own `tel.mod`
  const norm_scope = readback_scope(ctx, cls.tel)
  return Exp.cls(norm_sat, norm_scope)
}

function readback_sat(
  mod: Mod.Mod,
  ctx: Ctx.Ctx,
  sat: Array<{ name: string; t: Value.Value; value: Value.Value }>
): Array<{ name: string; t: Exp.Exp; exp: Exp.Exp }> {
  const norm_sat = new Array()
  for (const entry of sat) {
    const name = entry.name
    const t = Readback.readback(mod, ctx, Value.type, entry.t)
    const exp = Readback.readback(mod, ctx, entry.t, entry.value)
    norm_sat.push({ name, t, exp })
    Ctx.update(ctx, name, entry.t, entry.value)
  }
  return norm_sat
}

function readback_scope(
  ctx: Ctx.Ctx,
  tel: Value.Telescope.Telescope
): Array<{ name: string; t: Exp.Exp }> {
  const norm_scope = new Array()
  const env = Env.clone(tel.env)
  if (tel.next !== undefined) {
    const name = tel.next.name
    const t = Readback.readback(tel.mod, ctx, Value.type, tel.next.t)
    norm_scope.push({ name, t })
    Ctx.update(ctx, name, tel.next.t)
    Env.update(env, name, Value.not_yet(tel.next.t, Neutral.v(name)))
  }
  for (const entry of tel.scope) {
    const name = entry.name
    const mod = Mod.clone(tel.mod)
    const t_value = Evaluate.evaluate(mod, env, entry.t, {
      mode: Evaluate.EvaluationMode.mute_recursive_exp_in_mod,
    })
    const t = Readback.readback(mod, ctx, Value.type, t_value)
    norm_scope.push({ name, t })
    Ctx.update(ctx, name, t_value)
    Env.update(env, name, Value.not_yet(t_value, Neutral.v(name)))
  }
  return norm_scope
}