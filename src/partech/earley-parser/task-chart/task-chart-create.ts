import * as TaskChart from "../task-chart"
import * as Token from "../../token"

export function create(tokens: Array<Token.Token>): TaskChart.TaskChart {
  const length = chart_length_from_tokens_length(tokens.length)

  const task_maps = new Array()
  for (let i = 0; i < length; i++) {
    task_maps.push(new Map())
  }

  const resumable_indexing_sets = new Array()
  for (let i = 0; i < length; i++) {
    resumable_indexing_sets.push(new Map())
  }

  return {
    task_maps,
    resumable_indexing_sets,
  }
}

function chart_length_from_tokens_length(tokens_length: number): number {
  // NOTE The length of chart is one greater then the length of tokens.
  return tokens_length + 1
}
