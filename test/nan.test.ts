import { create_zod } from "@zod-rs/wasm"
import { describe, expect, test } from "vitest"

const z = create_zod()

describe("test z.nan", () => {
  test("z.nan", () => {
    expect(z.nan).toBeDefined()
    expect(() => { z.nan() }).not.toThrow()
    const schema = z.nan()
    expect(schema).toBeDefined()
    expect(() => { schema.parse(NaN) }).not.toThrow()
    
    // 以下は全てエラーになるべき
    expect(() => { schema.parse(0) }).toThrow()
    expect(() => { schema.parse(1) }).toThrow()
    expect(() => { schema.parse(-1) }).toThrow()
    expect(() => { schema.parse(Infinity) }).toThrow()
    expect(() => { schema.parse(-Infinity) }).toThrow()
    expect(() => { schema.parse("NaN") }).toThrow() // 文字列はエラー
    expect(() => { schema.parse(true) }).toThrow() // ブール値はエラー
    expect(() => { schema.parse(null) }).toThrow() // nullはエラー
    expect(() => { schema.parse(undefined) }).toThrow() // undefinedはエラー
    expect(() => { schema.parse({}) }).toThrow() // オブジェクトはエラー
    expect(() => { schema.parse([]) }).toThrow() // 配列はエラー
  })
})
