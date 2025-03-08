import { create_zod } from "@zod-rs/wasm"
import { describe, expect, test } from "vitest"

const z = create_zod()

describe("test z.boolean", () => {
  test("z.boolean", () => {
    expect(z.boolean).toBeDefined()
    expect(() => { z.boolean() }).not.toThrow()
    const schema = z.boolean()
    expect(schema).toBeDefined()
    
    // ブール値は問題なくパースできる
    expect(() => { schema.parse(true) }).not.toThrow()
    expect(() => { schema.parse(false) }).not.toThrow()
    
    // 結果値のテスト
    expect(schema.parse(true)).toBe(true)
    expect(schema.parse(false)).toBe(false)
    
    // ブール値以外はエラーになるべき
    expect(() => { schema.parse(0) }).toThrow()
    expect(() => { schema.parse(1) }).toThrow()
    expect(() => { schema.parse("true") }).toThrow() // 文字列はエラー
    expect(() => { schema.parse("false") }).toThrow() // 文字列はエラー
    expect(() => { schema.parse(null) }).toThrow() // nullはエラー
    expect(() => { schema.parse(undefined) }).toThrow() // undefinedはエラー
    expect(() => { schema.parse({}) }).toThrow() // オブジェクトはエラー
    expect(() => { schema.parse([]) }).toThrow() // 配列はエラー
  })
})
