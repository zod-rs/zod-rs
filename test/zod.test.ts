import pkg from "@zod-rs/wasm"
import { describe, expect, test } from "vitest"

const z = pkg.create_zod()

describe("check z object works", () => {
  test("test z.number", () => {
    expect(z.number).toBeDefined()
    expect(() => { z.number() }).not.toThrow()
  })
})

describe("is_number", () => {
  test("test is_number", () => {
    expect(pkg.is_number(1)).toBe(true)
    expect(pkg.is_number("1")).toBe(false)
    expect(pkg.is_number("A")).toBe(false)
  })
})

describe("is_string", () => {
  test("test is_string", () => {
    expect(pkg.is_string(1)).toBe(false)
    expect(pkg.is_string("1")).toBe(true)
    expect(pkg.is_string("A")).toBe(true)
  })
})
