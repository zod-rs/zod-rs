import { create_zod } from "@zod-rs/wasm"
import { describe, expect, test } from "vitest"

const z = create_zod()

describe("test z.bigint", () => {
  test("z.bigint", () => {
    expect(z.bigint).toBeDefined()
    expect(() => { z.bigint() }).not.toThrow()
    const schema = z.bigint()
    expect(schema).toBeDefined()
    expect(() => { schema.parse(2n) }).not.toThrow()
    expect(() => { schema.parse(BigInt(2)) }).not.toThrow()
    expect(() => { schema.parse(2) }).toThrow() // 数値型はエラー
    expect(() => { schema.parse("2") }).toThrow() // 文字列はエラー
    expect(() => { schema.parse("A") }).toThrow()
  })
  
  test("z.bigint().gt", () => {
    expect(z.bigint().gt).toBeDefined()
    const schema = z.bigint().gt(5n)
    expect(() => { schema.parse(6n) }).not.toThrow()
    expect(() => { schema.parse(10n) }).not.toThrow()
    expect(() => { schema.parse(5n) }).toThrow() // 境界値はエラー
    expect(() => { schema.parse(4n) }).toThrow()
    // 数値型引数のテスト
    const numberSchema = z.bigint().gt(5)
    expect(() => { numberSchema.parse(6n) }).not.toThrow()
    expect(() => { numberSchema.parse(5n) }).toThrow()
  })
  
  test("z.bigint().gte", () => {
    expect(z.bigint().gte).toBeDefined()
    const schema = z.bigint().gte(5n)
    expect(() => { schema.parse(6n) }).not.toThrow()
    expect(() => { schema.parse(10n) }).not.toThrow()
    expect(() => { schema.parse(5n) }).not.toThrow() // 境界値は許可
    expect(() => { schema.parse(4n) }).toThrow()
    // 数値型引数のテスト
    const numberSchema = z.bigint().gte(5)
    expect(() => { numberSchema.parse(5n) }).not.toThrow()
    expect(() => { numberSchema.parse(4n) }).toThrow()
  })
  
  test("z.bigint().lt", () => {
    expect(z.bigint().lt).toBeDefined()
    const schema = z.bigint().lt(5n)
    expect(() => { schema.parse(4n) }).not.toThrow()
    expect(() => { schema.parse(0n) }).not.toThrow()
    expect(() => { schema.parse(5n) }).toThrow() // 境界値はエラー
    expect(() => { schema.parse(6n) }).toThrow()
    // 数値型引数のテスト
    const numberSchema = z.bigint().lt(5)
    expect(() => { numberSchema.parse(4n) }).not.toThrow()
    expect(() => { numberSchema.parse(5n) }).toThrow()
  })
  
  test("z.bigint().lte", () => {
    expect(z.bigint().lte).toBeDefined()
    const schema = z.bigint().lte(5n)
    expect(() => { schema.parse(4n) }).not.toThrow()
    expect(() => { schema.parse(0n) }).not.toThrow()
    expect(() => { schema.parse(5n) }).not.toThrow() // 境界値は許可
    expect(() => { schema.parse(6n) }).toThrow()
    // 数値型引数のテスト
    const numberSchema = z.bigint().lte(5)
    expect(() => { numberSchema.parse(5n) }).not.toThrow()
    expect(() => { numberSchema.parse(6n) }).toThrow()
  })

  test("z.bigint().{min,max} (alias)", () => {
    expect(z.bigint().min).toBeDefined()
    expect(z.bigint().max).toBeDefined()
    const schema = z.bigint().min(5n).max(10n)
    expect(() => { schema.parse(5n) }).not.toThrow() // 最小境界値
    expect(() => { schema.parse(7n) }).not.toThrow() // 範囲内
    expect(() => { schema.parse(10n) }).not.toThrow() // 最大境界値
    expect(() => { schema.parse(4n) }).toThrow() // 範囲外
    expect(() => { schema.parse(11n) }).toThrow() // 範囲外
    // 数値型引数のテスト
    const numberSchema = z.bigint().min(5).max(10)
    expect(() => { numberSchema.parse(5n) }).not.toThrow()
    expect(() => { numberSchema.parse(4n) }).toThrow()
  })
  
  test("複合条件：min〜max範囲", () => {
    // 5〜10の範囲の値のみ許可
    const schema = z.bigint().gte(5n).lte(10n)
    expect(() => { schema.parse(5n) }).not.toThrow() // 最小境界値
    expect(() => { schema.parse(7n) }).not.toThrow() // 範囲内
    expect(() => { schema.parse(10n) }).not.toThrow() // 最大境界値
    expect(() => { schema.parse(4n) }).toThrow() // 範囲外
    expect(() => { schema.parse(11n) }).toThrow() // 範囲外
  })
  
  test("z.bigint().positive", () => {
    expect(z.bigint().positive).toBeDefined()
    const schema = z.bigint().positive()
    expect(() => { schema.parse(1n) }).not.toThrow() // 正の数
    expect(() => { schema.parse(BigInt(Number.MAX_SAFE_INTEGER)) }).not.toThrow() // 大きな値
    expect(() => { schema.parse(0n) }).toThrow() // 境界値（0）はエラー
    expect(() => { schema.parse(-1n) }).toThrow() // 負の数
  })
  
  test("z.bigint().nonnegative", () => {
    expect(z.bigint().nonnegative).toBeDefined()
    const schema = z.bigint().nonnegative()
    expect(() => { schema.parse(1n) }).not.toThrow() // 正の数
    expect(() => { schema.parse(0n) }).not.toThrow() // 境界値（0）は許可
    expect(() => { schema.parse(-1n) }).toThrow() // 負の数
  })
  
  test("z.bigint().negative", () => {
    expect(z.bigint().negative).toBeDefined()
    const schema = z.bigint().negative()
    expect(() => { schema.parse(-1n) }).not.toThrow() // 負の数
    expect(() => { schema.parse(BigInt(-Number.MAX_SAFE_INTEGER)) }).not.toThrow() // 大きな負の値
    expect(() => { schema.parse(0n) }).toThrow() // 境界値（0）はエラー
    expect(() => { schema.parse(1n) }).toThrow() // 正の数
  })
  
  test("z.bigint().nonpositive", () => {
    expect(z.bigint().nonpositive).toBeDefined()
    const schema = z.bigint().nonpositive()
    expect(() => { schema.parse(-1n) }).not.toThrow() // 負の数
    expect(() => { schema.parse(0n) }).not.toThrow() // 境界値（0）は許可
    expect(() => { schema.parse(1n) }).toThrow() // 正の数
  })
  
  test("z.bigint().multipleOf", () => {
    expect(z.bigint().multipleOf).toBeDefined()
    const schema = z.bigint().multipleOf(5n)
    expect(() => { schema.parse(0n) }).not.toThrow() // 0は任意の数の倍数
    expect(() => { schema.parse(5n) }).not.toThrow() // 5自身
    expect(() => { schema.parse(10n) }).not.toThrow() // 5の倍数
    expect(() => { schema.parse(15n) }).not.toThrow() // 5の倍数
    expect(() => { schema.parse(1n) }).toThrow() // 5の倍数ではない
    expect(() => { schema.parse(7n) }).toThrow() // 5の倍数ではない
    
    // 数値型引数のテスト
    const numberSchema = z.bigint().multipleOf(5)
    expect(() => { numberSchema.parse(10n) }).not.toThrow()
    expect(() => { numberSchema.parse(7n) }).toThrow()
  })
  
  test("複合条件：範囲と倍数", () => {
    // 1〜20の範囲内で、4の倍数のみ許可
    const schema = z.bigint().gte(1n).lte(20n).multipleOf(4n)
    expect(() => { schema.parse(4n) }).not.toThrow() // 範囲内の倍数
    expect(() => { schema.parse(8n) }).not.toThrow() // 範囲内の倍数
    expect(() => { schema.parse(20n) }).not.toThrow() // 最大値も4の倍数
    expect(() => { schema.parse(0n) }).toThrow() // 範囲外
    expect(() => { schema.parse(24n) }).toThrow() // 範囲外
    expect(() => { schema.parse(10n) }).toThrow() // 範囲内だが倍数ではない
  })
})
