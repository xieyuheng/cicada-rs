import * as ut from "../ut"

export type De<T> = {
  lit: (n: number) => T
  neg: (x: T) => T
  add: (x: T, y: T) => T
}

// NOTE This means we can have both initial and final encoding.
// - just like `from_present` we can define `from_initial`.
function from_present<T>(de: De<T>, present: ut.Json): T {
  if (typeof present === "number") return de.lit(present)
  present = ut.assert_json_object(present)
  if (present["$lit"]) return de.lit(ut.assert_json_number(present["$lit"]))
  if (present["$neg"]) return de.neg(from_present(de, present["$neg"]))
  if (present["$add"]) {
    const args = ut.assert_json_array(present["$add"])
    return de.add(from_present(de, args[0]), from_present(de, args[1]))
  }
  throw new Error(`Unknown present: ${ut.inspect(present)}`)
}

function tf1<T>(de: De<T>): T {
  // return de.add(de.lit(8), de.neg(de.add(de.lit(1), de.lit(2))))
  return from_present(de, { $add: [8, { $neg: { $add: [1, 2] } }] })
}

const number_de: De<number> = {
  lit: (n: number) => n,
  neg: (x: number) => -x,
  add: (x: number, y: number) => x + y,
}

const string_de: De<string> = {
  lit: (n: number) => `${n}`,
  neg: (x: string) => `(-${x})`,
  add: (x: string, y: string) => `(${x}+${y})`,
}

console.log(tf1(number_de))
console.log(tf1(string_de))

function tfl1<T>(de: De<T>): Array<T> {
  return [de.lit(1), de.add(de.lit(1), de.lit(3))]
}

console.log(tfl1(number_de))
console.log(tfl1(string_de))

export type Mul<T> = {
  mul: (x: T, y: T) => T
}

function tfm1<T>(de: De<T> & Mul<T>): T {
  return de.mul(de.lit(7), de.neg(de.mul(de.lit(1), de.lit(2))))
}

function tfm2<T>(de: De<T> & Mul<T>): T {
  return de.mul(de.lit(7), tf1(de))
}

const number_de_mul: Mul<number> = {
  mul: (x: number, y: number) => x * y,
}

const string_de_mul: Mul<string> = {
  mul: (x: string, y: string) => `(${x}*${y})`,
}

console.log(tfm1({ ...number_de, ...number_de_mul }))
console.log(tfm1({ ...string_de, ...string_de_mul }))

console.log(tfm2({ ...number_de, ...number_de_mul }))
console.log(tfm2({ ...string_de, ...string_de_mul }))
