import * as Value from "../value"
import * as Exp from "../exp"
import * as Ctx from "../ctx"

export function conversion(
  ctx: Ctx.Ctx,
  t: Value.Value,
  x: Value.Value,
  y: Value.Value
): boolean {
  return Exp.alpha_equivalent(
    Value.readback(ctx, t, x),
    Value.readback(ctx, t, y)
  )
}