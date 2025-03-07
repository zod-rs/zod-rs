import pkg, { type ZodNumber } from "@zod-rs/wasm"
import { describe, expect, test } from "vitest"

const z = pkg.create_zod()

describe("test z.number", () => {
  test("z.number", () => {
    expect(z.number).toBeDefined()
    expect(() => { z.number() }).not.toThrow()
    const numberSchema: ZodNumber = z.number()
    expect(numberSchema).toBeDefined()
    expect(() => { numberSchema.parse(2) }).not.toThrow()
    expect(() => { numberSchema.parse("2") }).toThrow()
    expect(() => { numberSchema.parse("A") }).toThrow()
  })
  
  test("z.number().gt", () => {
    expect(z.number().gt).toBeDefined()
    const schema = z.number().gt(5)
    expect(() => { schema.parse(6) }).not.toThrow()
    expect(() => { schema.parse(10) }).not.toThrow()
    expect(() => { schema.parse(5) }).toThrow() // 境界値はエラー
    expect(() => { schema.parse(4) }).toThrow()
    expect(() => { schema.parse("10") }).toThrow() // 数値型以外はエラー
  })
  
  test("z.number().gte", () => {
    expect(z.number().gte).toBeDefined()
    const schema = z.number().gte(5)
    expect(() => { schema.parse(6) }).not.toThrow()
    expect(() => { schema.parse(10) }).not.toThrow()
    expect(() => { schema.parse(5) }).not.toThrow() // 境界値は許可
    expect(() => { schema.parse(4) }).toThrow()
    expect(() => { schema.parse("5") }).toThrow() // 数値型以外はエラー
  })
  
  test("z.number().lt", () => {
    expect(z.number().lt).toBeDefined()
    const schema = z.number().lt(5)
    expect(() => { schema.parse(4) }).not.toThrow()
    expect(() => { schema.parse(0) }).not.toThrow()
    expect(() => { schema.parse(5) }).toThrow() // 境界値はエラー
    expect(() => { schema.parse(6) }).toThrow()
    expect(() => { schema.parse("4") }).toThrow() // 数値型以外はエラー
  })
  
  test("z.number().lte", () => {
    expect(z.number().lte).toBeDefined()
    const schema = z.number().lte(5)
    expect(() => { schema.parse(4) }).not.toThrow()
    expect(() => { schema.parse(0) }).not.toThrow()
    expect(() => { schema.parse(5) }).not.toThrow() // 境界値は許可
    expect(() => { schema.parse(6) }).toThrow()
    expect(() => { schema.parse("5") }).toThrow() // 数値型以外はエラー
  })
  
  test("複合条件：min〜max範囲", () => {
    // 5〜10の範囲の値のみ許可
    const schema = z.number().gte(5).lte(10)
    expect(() => { schema.parse(5) }).not.toThrow() // 最小境界値
    expect(() => { schema.parse(7) }).not.toThrow() // 範囲内
    expect(() => { schema.parse(10) }).not.toThrow() // 最大境界値
    expect(() => { schema.parse(4.9) }).toThrow() // 範囲外
    expect(() => { schema.parse(10.1) }).toThrow() // 範囲外
  })
})
