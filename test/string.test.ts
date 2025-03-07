import pkg from "@zod-rs/wasm"
import { describe, expect, test } from "vitest"

const z = pkg.create_zod()

describe("test z.string", () => {
  test("z.string", () => {
    expect(z.string).toBeDefined()
    expect(() => { z.string() }).not.toThrow()
    const stringSchema = z.string()
    expect(stringSchema).toBeDefined()
    expect(() => { stringSchema.parse("A") }).not.toThrow()
    expect(() => { stringSchema.parse("2") }).not.toThrow()
    expect(() => { stringSchema.parse(2) }).toThrow()
  })
})
