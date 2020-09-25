import * as Schedule from "../schedule"
import * as Task from "../task"
import * as Token from "../../../token"
import * as Value from "../../../value"
import * as ut from "../../../../ut"

export function recognize(
  tokens: Array<Token.Token>,
  grammar: Value.Value,
  opts: Schedule.Opts = Schedule.DEFAULT_OPTS
): boolean {
  if (grammar.kind !== "Value.grammar") {
    throw new Error(
      `expecting grammar to be Value.grammar.\n` +
        `grammar: ${ut.inspect(grammar)}\n`
    )
  }

  const schedule = Schedule.init(tokens, grammar)
  Schedule.run(schedule, opts)
  const end = schedule.chart[schedule.chart.length - 1]

  for (const task of end.values()) {
    if (
      grammar.name === task.grammar_name &&
      Task.current_index(task) === tokens.length
    ) {
      return true
    }
  }

  return false
}
