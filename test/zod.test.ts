import pkg, { type ZodNumber } from "@zod-rs/wasm"
import { describe, expect, test } from "vitest"

const z = pkg.create_zod()

describe("check z object works", () => {
  test("test z.number", () => {
    expect(z.number).toBeDefined()
    expect(() => { z.number() }).not.toThrow()
    const numberSchema: ZodNumber = z.number()
    expect(numberSchema).toBeDefined()
    expect(() => { numberSchema.parse(2) }).not.toThrow()
    expect(() => { numberSchema.parse("2") }).toThrow()
    expect(() => { numberSchema.parse("A") }).toThrow()
  })

  test("test z.string", () => {
    expect(z.string).toBeDefined()
    expect(() => { z.string() }).not.toThrow()
    const stringSchema = z.string()
    expect(stringSchema).toBeDefined()
    expect(() => { stringSchema.parse("A") }).not.toThrow()
    expect(() => { stringSchema.parse("2") }).not.toThrow()
    expect(() => { stringSchema.parse(2) }).toThrow()
  })
})
