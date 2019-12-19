package xieyuheng.cicada

import collection.immutable.ListMap

import eval._
import infer._
import subtype._
import pretty._

object check {

  def check(env: Env, ctx: Ctx, exp: Exp, t: Value): Unit = {
    try {
      exp match {
        case Obj(value_map: ListMap[String, Exp]) =>
          t match {
            case cl: ValueCl =>
              check_telescope(env, ctx, value_map, cl.type_map, cl.env)

            case _ =>
              throw Report(List(
                s"expecting class type but found: ${t}\n"
              ))
          }

        case _ =>
          val s = infer(env, ctx, exp)
          subtype(ctx, s, t)
      }
    } catch {
      case report: Report =>
        throw report.prepend(
          s"check fail\n" +
            s"exp: ${pretty_exp(exp)}\n" +
            s"t: ${pretty_value(t)}\n"
        )
    }
  }

  def check_telescope(
    env: Env,
    ctx: Ctx,
    arg_exp_map: ListMap[String, Exp],
    type_map: ListMap[String, Exp],
    init_telescope_env: Env,
  ): (Env, Ctx) = {
    var local_env = init_telescope_env
    var local_ctx = ctx
    type_map.foreach {
      case (name, t_exp) =>
        val t_value = eval(local_env, t_exp)
        val v_exp = arg_exp_map.get(name) match {
          case Some(v_exp) => v_exp
          case None =>
            throw Report(List(
              s"check_telescope fail\n" +
                s"can not find a field of object in class\n" +
                s"field: ${name}\n"
            ))
        }
        check(local_env, local_ctx, v_exp, t_value)
        val v_value = eval(env, v_exp)
        local_env = local_env.ext(name, v_value)
        local_ctx = local_ctx.ext(name, t_value)
    }
    (local_env, local_ctx)
  }

}
