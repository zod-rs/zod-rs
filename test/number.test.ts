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

  test("z.number().{min,max} (alias)", () => {
    expect(z.number().min).toBeDefined()
    expect(z.number().max).toBeDefined()
    const schema = z.number().min(5).max(10)
    expect(() => { schema.parse(5) }).not.toThrow() // 最小境界値
    expect(() => { schema.parse(7) }).not.toThrow() // 範囲内
    expect(() => { schema.parse(10) }).not.toThrow() // 最大境界値
    expect(() => { schema.parse(4.9) }).toThrow() // 範囲外
    expect(() => { schema.parse(10.1) }).toThrow() // 範囲外
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
  
  test("z.number().int", () => {
    expect(z.number().int).toBeDefined()
    const schema = z.number().int()
    expect(() => { schema.parse(1) }).not.toThrow() // 整数
    expect(() => { schema.parse(0) }).not.toThrow() // 0
    expect(() => { schema.parse(-5) }).not.toThrow() // 負の整数
    expect(() => { schema.parse(1.5) }).toThrow() // 小数
    expect(() => { schema.parse(0.1) }).toThrow() // 小さい小数
    expect(() => { schema.parse(-1.2) }).toThrow() // 負の小数
    expect(() => { schema.parse("5") }).toThrow() // 数値型以外
  })
  
  test("複合条件：整数範囲", () => {
    // 1〜5の整数のみ許可
    const schema = z.number().int().gte(1).lte(5)
    expect(() => { schema.parse(1) }).not.toThrow() // 最小値
    expect(() => { schema.parse(3) }).not.toThrow() // 範囲内
    expect(() => { schema.parse(5) }).not.toThrow() // 最大値
    expect(() => { schema.parse(0) }).toThrow() // 範囲外
    expect(() => { schema.parse(6) }).toThrow() // 範囲外
    expect(() => { schema.parse(1.5) }).toThrow() // 小数
    expect(() => { schema.parse("3") }).toThrow() // 数値型以外
  })
  
  test("z.number().positive", () => {
    expect(z.number().positive).toBeDefined()
    const schema = z.number().positive()
    expect(() => { schema.parse(1) }).not.toThrow() // 正の数
    expect(() => { schema.parse(0.1) }).not.toThrow() // 小さい正の数
    expect(() => { schema.parse(Number.MAX_VALUE) }).not.toThrow() // 最大値
    expect(() => { schema.parse(0) }).toThrow() // 境界値（0）はエラー
    expect(() => { schema.parse(-1) }).toThrow() // 負の数
    expect(() => { schema.parse("1") }).toThrow() // 数値型以外
  })
  
  test("z.number().nonnegative", () => {
    expect(z.number().nonnegative).toBeDefined()
    const schema = z.number().nonnegative()
    expect(() => { schema.parse(1) }).not.toThrow() // 正の数
    expect(() => { schema.parse(0) }).not.toThrow() // 境界値（0）は許可
    expect(() => { schema.parse(-0.1) }).toThrow() // 小さい負の数
    expect(() => { schema.parse(-10) }).toThrow() // 負の数
    expect(() => { schema.parse("0") }).toThrow() // 数値型以外
  })
  
  test("z.number().negative", () => {
    expect(z.number().negative).toBeDefined()
    const schema = z.number().negative()
    expect(() => { schema.parse(-1) }).not.toThrow() // 負の数
    expect(() => { schema.parse(-0.1) }).not.toThrow() // 小さい負の数
    expect(() => { schema.parse(-Number.MAX_VALUE) }).not.toThrow() // 最小値
    expect(() => { schema.parse(0) }).toThrow() // 境界値（0）はエラー
    expect(() => { schema.parse(1) }).toThrow() // 正の数
    expect(() => { schema.parse("-1") }).toThrow() // 数値型以外
  })
  
  test("z.number().nonpositive", () => {
    expect(z.number().nonpositive).toBeDefined()
    const schema = z.number().nonpositive()
    expect(() => { schema.parse(-1) }).not.toThrow() // 負の数
    expect(() => { schema.parse(0) }).not.toThrow() // 境界値（0）は許可
    expect(() => { schema.parse(0.1) }).toThrow() // 小さい正の数
    expect(() => { schema.parse(10) }).toThrow() // 正の数
    expect(() => { schema.parse("-1") }).toThrow() // 数値型以外
  })
  
  test("複合条件：正の整数", () => {
    // 正の整数のみ許可
    const schema = z.number().int().positive()
    expect(() => { schema.parse(1) }).not.toThrow() // 最小の正の整数
    expect(() => { schema.parse(100) }).not.toThrow() // 大きい値
    expect(() => { schema.parse(0) }).toThrow() // 0はエラー
    expect(() => { schema.parse(-1) }).toThrow() // 負の数
    expect(() => { schema.parse(1.5) }).toThrow() // 小数
  })
  
  test("複合条件：非正の整数", () => {
    // 0以下の整数のみ許可
    const schema = z.number().int().nonpositive()
    expect(() => { schema.parse(0) }).not.toThrow() // 0は許可
    expect(() => { schema.parse(-1) }).not.toThrow() // 負の整数
    expect(() => { schema.parse(-100) }).not.toThrow() // 大きい負の値
    expect(() => { schema.parse(1) }).toThrow() // 正の数
    expect(() => { schema.parse(-1.5) }).toThrow() // 負の小数
  })
  
  test("z.number().multipleOf", () => {
    expect(z.number().multipleOf).toBeDefined()
    const schema = z.number().multipleOf(5)
    expect(() => { schema.parse(0) }).not.toThrow() // 0は任意の数の倍数
    expect(() => { schema.parse(5) }).not.toThrow() // 5自身
    expect(() => { schema.parse(10) }).not.toThrow() // 5の倍数
    expect(() => { schema.parse(15) }).not.toThrow() // 5の倍数
    expect(() => { schema.parse(1) }).toThrow() // 5の倍数ではない
    expect(() => { schema.parse(7) }).toThrow() // 5の倍数ではない
    expect(() => { schema.parse("10") }).toThrow() // 数値型以外
  })
  
  test("z.number().step (multipleOfのエイリアス)", () => {
    expect(z.number().step).toBeDefined()
    const schema = z.number().step(5)
    expect(() => { schema.parse(0) }).not.toThrow() // 0は任意の数の倍数
    expect(() => { schema.parse(5) }).not.toThrow() // 5自身
    expect(() => { schema.parse(10) }).not.toThrow() // 5の倍数
    expect(() => { schema.parse(3) }).toThrow() // 5の倍数ではない
  })
  
  test("複合条件：整数かつ特定値の倍数", () => {
    // 3の倍数である整数のみ許可
    const schema = z.number().int().multipleOf(3)
    expect(() => { schema.parse(0) }).not.toThrow() // 0は任意の数の倍数
    expect(() => { schema.parse(3) }).not.toThrow() // 3の倍数
    expect(() => { schema.parse(6) }).not.toThrow() // 3の倍数
    expect(() => { schema.parse(3.0) }).not.toThrow() // 整数表現でも可
    expect(() => { schema.parse(1) }).toThrow() // 3の倍数ではない
    expect(() => { schema.parse(3.1) }).toThrow() // 3の倍数だが整数ではない
  })
  
  test("複合条件：範囲と倍数", () => {
    // 1〜20の範囲内で、4の倍数のみ許可
    const schema = z.number().gte(1).lte(20).multipleOf(4)
    expect(() => { schema.parse(4) }).not.toThrow() // 範囲内の倍数
    expect(() => { schema.parse(8) }).not.toThrow() // 範囲内の倍数
    expect(() => { schema.parse(20) }).not.toThrow() // 最大値も4の倍数
    expect(() => { schema.parse(0) }).toThrow() // 範囲外
    expect(() => { schema.parse(24) }).toThrow() // 範囲外
    expect(() => { schema.parse(10) }).toThrow() // 範囲内だが倍数ではない
  })
  
  test("z.number().finite", () => {
    expect(z.number().finite).toBeDefined()
    const schema = z.number().finite()
    expect(() => { schema.parse(0) }).not.toThrow() // 通常の数値
    expect(() => { schema.parse(42) }).not.toThrow() // 通常の数値
    expect(() => { schema.parse(-1) }).not.toThrow() // 負の数値
    expect(() => { schema.parse(1.5) }).not.toThrow() // 小数値
    
    // Infinityはエラーになるべき
    // biome-ignore lint/style/useNumberNamespace: <explanation>
    expect(() => { schema.parse(Infinity) }).toThrow()
    // biome-ignore lint/style/useNumberNamespace: <explanation>
    expect(() => { schema.parse(-Infinity) }).toThrow()
    
    // NaNはエラーになるべき
    // biome-ignore lint/style/useNumberNamespace: <explanation>
    expect(() => { schema.parse(NaN) }).toThrow()
  })
  
  test("z.number().safe", () => {
    expect(z.number().safe).toBeDefined()
    const schema = z.number().safe()
    expect(() => { schema.parse(0) }).not.toThrow() // 通常の数値
    expect(() => { schema.parse(42) }).not.toThrow() // 通常の数値
    expect(() => { schema.parse(-1) }).not.toThrow() // 負の数値
    expect(() => { schema.parse(1.5) }).not.toThrow() // 小数値
    expect(() => { schema.parse(Number.MAX_SAFE_INTEGER) }).not.toThrow() // 最大安全整数
    expect(() => { schema.parse(Number.MIN_SAFE_INTEGER) }).not.toThrow() // 最小安全整数
    
    // 安全な整数範囲外はエラーになるべき
    expect(() => { schema.parse(Number.MAX_SAFE_INTEGER + 1) }).toThrow()
    expect(() => { schema.parse(Number.MIN_SAFE_INTEGER - 1) }).toThrow()
  })
  
  test("複合条件：整数＋安全範囲", () => {
    // 整数かつ安全な整数範囲内であることを検証
    const schema = z.number().int().safe()
    expect(() => { schema.parse(0) }).not.toThrow() // 整数かつ安全
    expect(() => { schema.parse(42) }).not.toThrow() // 整数かつ安全
    expect(() => { schema.parse(-100) }).not.toThrow() // 負の整数かつ安全
    expect(() => { schema.parse(1.5) }).toThrow() // 整数ではない
    expect(() => { schema.parse(Number.MAX_SAFE_INTEGER + 1) }).toThrow() // 安全でない
  })
  
  test("複合条件：有限数＋正の数", () => {
    // 有限かつ正の数であることを検証
    const schema = z.number().finite().positive()
    expect(() => { schema.parse(1) }).not.toThrow() // 有限かつ正
    expect(() => { schema.parse(0.1) }).not.toThrow() // 有限かつ正
    expect(() => { schema.parse(0) }).toThrow() // 0は正ではない
    expect(() => { schema.parse(-1) }).toThrow() // 負の数は正ではない
    // biome-ignore lint/style/useNumberNamespace: <explanation>
    expect(() => { schema.parse(Infinity) }).toThrow() // 無限大は有限ではない
  })
})
