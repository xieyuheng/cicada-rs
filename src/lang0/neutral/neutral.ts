import * as Value from "../value"

export type Neutral = v | ap

type v = {
  kind: "Neutral.v"
  name: string
}

export const v = (name: string): v => ({ kind: "Neutral.v", name })

type ap = {
  kind: "Neutral.ap"
  target: Neutral
  arg: Value.Value
}

export const ap = (target: Neutral, arg: Value.Value): ap => ({
  kind: "Neutral.ap",
  target,
  arg,
})
