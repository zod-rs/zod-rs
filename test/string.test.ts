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

  test("z.string().min", () => {
    expect(z.string().min).toBeDefined()
    const schema = z.string().min(5)
    expect(() => { schema.parse("12345") }).not.toThrow() // 境界値は許可
    expect(() => { schema.parse("123456") }).not.toThrow() // 最小文字数より多い
    expect(() => { schema.parse("1234") }).toThrow() // 最小文字数未満
    expect(() => { schema.parse(123) }).toThrow() // 文字列以外

    expect(schema.safeParse("12345").success).toBe(true)
    expect(schema.safeParse("1234").success).toBe(false)
  })

  test("z.string().max", () => {
    expect(z.string().max).toBeDefined()
    const schema = z.string().max(5)
    expect(() => { schema.parse("12345") }).not.toThrow() // 境界値は許可
    expect(() => { schema.parse("1234") }).not.toThrow() // 最大文字数より少ない
    expect(() => { schema.parse("123456") }).toThrow() // 最大文字数超過
    expect(() => { schema.parse(123) }).toThrow() // 文字列以外

    expect(schema.safeParse("12345").success).toBe(true)
    expect(schema.safeParse("123456").success).toBe(false)
  })

  test("z.string().length", () => {
    expect(z.string().length).toBeDefined()
    const schema = z.string().length(5)
    expect(() => { schema.parse("12345") }).not.toThrow() // 正確な文字数
    expect(() => { schema.parse("1234") }).toThrow() // 文字数不足
    expect(() => { schema.parse("123456") }).toThrow() // 文字数超過
    expect(() => { schema.parse(12345) }).toThrow() // 文字列以外

    expect(schema.safeParse("12345").success).toBe(true)
    expect(schema.safeParse("1234").success).toBe(false)
    expect(schema.safeParse("123456").success).toBe(false)
  })

  test("z.string().email", () => {
    expect(z.string().email).toBeDefined()
    const schema = z.string().email()
    expect(() => { schema.parse("test@example.com") }).not.toThrow() // 有効なメールアドレス
    expect(() => { schema.parse("invalid-email") }).toThrow() // 無効なメールアドレス
    expect(() => { schema.parse("@example.com") }).toThrow() // 無効なメールアドレス
    expect(() => { schema.parse("test@") }).toThrow() // 無効なメールアドレス
    expect(() => { schema.parse(123) }).toThrow() // 文字列以外

    expect(schema.safeParse("test@example.com").success).toBe(true)
    expect(schema.safeParse("invalid-email").success).toBe(false)
  })

  test("z.string().url", () => {
    expect(z.string().url).toBeDefined()
    const schema = z.string().url()
    expect(() => { schema.parse("https://example.com") }).not.toThrow() // 有効なURL
    expect(() => { schema.parse("http://example.com/path") }).not.toThrow() // パス付きURL
    expect(() => { schema.parse("invalid-url") }).toThrow() // 無効なURL
    expect(() => { schema.parse("example.com") }).toThrow() // プロトコルなしは無効
    expect(() => { schema.parse(123) }).toThrow() // 文字列以外

    expect(schema.safeParse("https://example.com").success).toBe(true)
    expect(schema.safeParse("invalid-url").success).toBe(false)
  })

  test("z.string().regex", () => {
    expect(z.string().regex).toBeDefined()
    const schema = z.string().regex(/^[0-9]+$/)
    expect(() => { schema.parse("12345") }).not.toThrow() // パターンにマッチ
    expect(() => { schema.parse("abc") }).toThrow() // パターンにマッチしない
    expect(() => { schema.parse("123abc") }).toThrow() // パターンにマッチしない
    expect(() => { schema.parse(12345) }).toThrow() // 文字列以外

    expect(schema.safeParse("12345").success).toBe(true)
    expect(schema.safeParse("abc").success).toBe(false)
  })

  test("z.string().includes", () => {
    expect(z.string().includes).toBeDefined()
    const schema = z.string().includes("test")
    expect(() => { schema.parse("this is a test string") }).not.toThrow() // 含む
    expect(() => { schema.parse("test") }).not.toThrow() // 完全一致
    expect(() => { schema.parse("no match here") }).toThrow() // 含まない
    expect(() => { schema.parse(123) }).toThrow() // 文字列以外

    expect(schema.safeParse("this is a test string").success).toBe(true)
    expect(schema.safeParse("no match here").success).toBe(false)
  })

  test("z.string().startsWith", () => {
    expect(z.string().startsWith).toBeDefined()
    const schema = z.string().startsWith("test")
    expect(() => { schema.parse("test string") }).not.toThrow() // 先頭一致
    expect(() => { schema.parse("test") }).not.toThrow() // 完全一致
    expect(() => { schema.parse("no test here") }).toThrow() // 先頭一致しない
    expect(() => { schema.parse(123) }).toThrow() // 文字列以外

    expect(schema.safeParse("test string").success).toBe(true)
    expect(schema.safeParse("no test here").success).toBe(false)
  })

  test("z.string().endsWith", () => {
    expect(z.string().endsWith).toBeDefined()
    const schema = z.string().endsWith("test")
    expect(() => { schema.parse("end with test") }).not.toThrow() // 末尾一致
    expect(() => { schema.parse("test") }).not.toThrow() // 完全一致
    expect(() => { schema.parse("test in middle") }).toThrow() // 末尾一致しない
    expect(() => { schema.parse(123) }).toThrow() // 文字列以外

    expect(schema.safeParse("end with test").success).toBe(true)
    expect(schema.safeParse("test in middle").success).toBe(false)
  })

  test("複合条件：min-max範囲", () => {
    // 5〜10文字の範囲の文字列のみ許可
    const schema = z.string().min(5).max(10)
    expect(() => { schema.parse("12345") }).not.toThrow() // 最小文字数
    expect(() => { schema.parse("1234567890") }).not.toThrow() // 最大文字数
    expect(() => { schema.parse("1234567") }).not.toThrow() // 範囲内
    expect(() => { schema.parse("1234") }).toThrow() // 範囲外（少ない）
    expect(() => { schema.parse("12345678901") }).toThrow() // 範囲外（多い）

    expect(schema.safeParse("12345").success).toBe(true)
    expect(schema.safeParse("1234").success).toBe(false)
    expect(schema.safeParse("12345678901").success).toBe(false)
  })

  test("複合条件：emailとmin", () => {
    // 最小10文字のメールアドレスのみ許可
    const schema = z.string().email().min(10)
    expect(() => { schema.parse("test@example.com") }).not.toThrow() // 有効（条件を満たす）
    expect(() => { schema.parse("a@b.c") }).toThrow() // 有効なメールだが短すぎる
    expect(() => { schema.parse("invalid-email") }).toThrow() // 無効なメール

    expect(schema.safeParse("test@example.com").success).toBe(true)
    expect(schema.safeParse("a@b.c").success).toBe(false)
    expect(schema.safeParse("invalid-email").success).toBe(false)
  })

  test("複合条件：urlとstartsWith", () => {
    // https://で始まるURLのみ許可
    const schema = z.string().url().startsWith("https://")
    expect(() => { schema.parse("https://example.com") }).not.toThrow() // 有効（条件を満たす）
    expect(() => { schema.parse("http://example.com") }).toThrow() // 有効なURLだがhttpsではない
    expect(() => { schema.parse("invalid-url") }).toThrow() // 無効なURL

    expect(schema.safeParse("https://example.com").success).toBe(true)
    expect(schema.safeParse("http://example.com").success).toBe(false)
    expect(schema.safeParse("invalid-url").success).toBe(false)
  })
})
