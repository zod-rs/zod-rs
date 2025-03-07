import pkg from "@zod-rs/wasm"
import { expect, test, describe } from "vitest"

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
