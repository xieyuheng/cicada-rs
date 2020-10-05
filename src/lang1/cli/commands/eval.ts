import * as Exp from "../../exp"
import * as frontend from "../../frontend"
import * as Ty from "../../ty"
import * as Ctx from "../../ctx"
import * as Env from "../../env"
import * as Trace from "../../../trace"
import * as pt from "../../../partech"
import * as ut from "../../../ut"
import fs from "fs"
import strip_ansi from "strip-ansi"

export const command = "eval <input>"

export const aliases = ["$0"]

export const description = "Eval a file"

export const builder = {
  nocolor: { type: "boolean", default: false },
}

interface Argv {
  input: string
  nocolor: boolean
}

export const handler = async (argv: Argv) => {
  const text = fs.readFileSync(argv.input, { encoding: "utf-8" })

  try {
    const exp = frontend.parse_exp(text)
    const ctx = Ctx.init()
    const t = Exp.infer(ctx, exp)
    const env = Env.init()
    console.log(`${Exp.repr(Exp.normalize(exp))}: ${Ty.repr(t)}`)
  } catch (error) {
    if (error instanceof Trace.Trace) {
      const trace = error
      console.error(Trace.repr(trace, Exp.repr))
      process.exit(1)
    }
    if (error instanceof pt.ParsingError) {
      let message = error.message
      message += "\n"
      message += pt.Span.report(error.span, text)
      console.error(argv.nocolor ? strip_ansi(message) : message)
      process.exit(1)
    } else {
      throw error
    }
  }
}