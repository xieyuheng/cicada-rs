package xieyuheng.cicada

import collection.immutable.ListMap
import scala.util.{ Try, Success, Failure }

import eval._
import check._
import subtype._
import readback._
import pretty._

object infer {

  def infer(env: Env, ctx: Ctx, exp: Exp): Value = {
    try {
      exp match {
        case Var(name: String) =>
          ctx.lookup_type(name) match {
            case Some(t) => t
            case None =>
              throw Report(List(
                s"can not find var: ${name} in ctx\n"
              ))
          }

        case Type() =>
          ValueType()

        case StrType() =>
          ValueType()

        case Str(str: String) =>
          ValueStrType()

        case Pi(type_map: ListMap[String, Exp], return_type: Exp) =>
          var local_ctx = ctx
          type_map.foreach {
            case (name, exp) =>
              check(env, local_ctx, exp, ValueType())
              local_ctx = local_ctx.ext(name, eval(env, exp))
          }
          check(env, local_ctx, return_type, ValueType())
          ValueType()

        case Fn(type_map: ListMap[String, Exp], body: Exp) =>
          var local_ctx = ctx
          type_map.foreach {
            case (name, exp) =>
              check(env, local_ctx, exp, ValueType())
              local_ctx = local_ctx.ext(name, eval(env, exp))
          }
          val return_type_value = infer(env, local_ctx, body)
          val return_type = readback(return_type_value)
          ValuePi(Telescope(type_map, env), return_type)

        case Cl(defined, type_map: ListMap[String, Exp]) =>
          var local_env = env
          var local_ctx = ctx
          defined.foreach {
            case (name, (t, v)) =>
              check(local_env, local_ctx, t, ValueType())
              val t_value = eval(local_env, t)
              check(local_env, local_ctx, v, t_value)
              val v_value = eval(local_env, v)
              local_ctx = local_ctx.ext(name, t_value)
              local_env = local_env.ext(name, v_value)
          }
          type_map.foreach {
            case (name, t) =>
              check(local_env, local_ctx, t, ValueType())
              local_ctx = local_ctx.ext(name, eval(env, t))
          }
          ValueType()

        case Obj(value_map: ListMap[String, Exp]) =>
          ValueClInferedFromObj(value_map.map {
            case (name, exp) =>
              val value = eval(env, exp)
              (name, infer(env, ctx, readback(value)))
          })

        case Ap(target: Exp, arg_list: List[Exp]) =>
          infer(env, ctx, target) match {
            case ValuePi(telescope: Telescope, return_type: Exp) =>
              val value_list = arg_list.map { eval(env, _) }
              val (new_defined, new_telescope) = telescope_apply(telescope, value_list)
              eval(new_telescope.env, return_type)

            case ValueType() =>
              eval(env, target) match {
                case ValueCl(defined, telescope) =>
                  val name_list = telescope.name_list
                  val arg_map = ListMap(name_list.zip(arg_list): _*)
                  val value_map = arg_map.map {
                    case (name, exp) => (name, eval(env, exp))
                  }
                  telescope_check(ctx, value_map, telescope)
                  ValueType()

                case t =>
                  throw Report(List(
                    s"expecting ValueCl but found: ${t}\n"
                  ))
              }

            case t =>
              throw Report(List(
                s"expecting ValuePi type but found: ${t}\n"
              ))
          }

        case Dot(target: Exp, field: String) =>
          val t_infered = infer(env, ctx, target)

          t_infered match {
            case ValueClInferedFromObj(type_map: ListMap[String, Value]) =>
              type_map.get(field) match {
                case Some(t) => t
                case None =>
                  throw Report(List(
                    s"infer fail\n" +
                      s"on ValueClInferedFromObj\n" +
                      s"target exp: ${pretty_exp(target)}\n" +
                      s"infered target type: ${pretty_value(t_infered)}\n" +
                      s"can not find field for dot: ${field}\n"
                  ))
              }

            case ValueCl(defined, telescope) =>
              defined.get(field) match {
                case Some((t, v)) =>
                  infer(env, ctx, readback(v))
                case None =>
                  util.telescope_force(telescope, telescope.name_list).get(field) match {
                    case Some(t) => t
                    case None =>
                      throw Report(List(
                        s"infer fail\n" +
                          s"on ValueCl\n" +
                          s"target exp: ${pretty_exp(target)}\n" +
                          s"infered target type: ${pretty_value(t_infered)}\n" +
                          s"can not find field for dot: ${field}\n"
                      ))
                  }
              }

            case t =>
              throw Report(List(
                s"expecting class\n" +
                  s"found type: ${pretty_value(t)}\n" +
                  s"target: ${pretty_exp(target)}\n"
              ))
          }

        case Union(type_list: List[Exp]) =>
          type_list.foreach {
            case (exp) =>
              check(env, ctx, exp, ValueType())
          }
          ValueType()

        case Switch(name: String, cases: List[(Exp, Exp)]) =>
          ctx.lookup_type(name) match {
            case Some(r) =>
              val type_map = cases.map {
                case (s, v) =>
                  val s_value = eval(env, s)
                  Try {
                    subtype(ctx, s_value, r)
                  } match {
                    case Success(()) =>
                      infer(env, ctx.ext(name, s_value), v)
                    case Failure(error) =>
                      throw Report(List(
                        s"at compile time, we know type of ${name} is ${pretty_value(r)}\n" +
                          s"in a case of switch\n" +
                          s"the type ${pretty_value(s_value)} is not a subtype of the abvoe type\n" +
                          s"it is meaningless to write this case\n" +
                          s"because we know it will never be matched\n"
                      ))
                  }
              }
              ValueUnion(type_map)

            case None =>
              val type_map = cases.map {
                case (s, v) =>
                  infer(env, ctx.ext(name, eval(env, s)), v)
              }
              ValueUnion(type_map)
          }

        case Block(block_entry_map: ListMap[String, BlockEntry], body: Exp) =>
          var local_ctx = ctx
          block_entry_map.foreach {
            case (name, BlockEntryLet(exp)) =>
              local_ctx = local_ctx.ext(name, eval(env, exp))
            case (name, BlockEntryDefine(_t, exp)) =>
              local_ctx = local_ctx.ext(name, eval(env, exp))
          }
          infer(env, local_ctx, body)
      }
    } catch {
      case report: Report =>
        report.throw_prepend(
          s"infer fail\n" +
            s"exp: ${pretty_exp(exp)}\n"
        )
    }
  }

}
