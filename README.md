# `zod-rs`

**This project is under construction.**

Zod-compatible schema validator

## Usage

```ts
import { z } from "zod-rs";

const schema = z.object({
  name: z.string(),
  age: z.number(),
});

const data = {
  name: "Alice",
  age: 30,
};

const parsed = schema.safeParse(data);

if (!parsed.success) {
  console.error("Validation errors:", parsed.error);
} else {
  console.log("Validated data:", parsed.data);
}
```

## Building

```sh
corepack pnpm build:vendor
# then wasm-bindgen package will be built at vendor/pkg
```

## Roadmap

Currently, this project aims to be compatible with zod@3.24.2.

- Primitives
  - [ ] `z.string()`
  - [*] `z.number()`
  - [ ] `z.bigint()`
  - [ ] `z.boolean()`
  - [ ] `z.date()`
  - [ ] `z.symbol()`
  - [ ] `z.undefined()`
  - [ ] `z.null()`
  - [ ] `z.void()`
  - [ ] `z.any()`
  - [ ] `z.unknown()`
  - [ ] `z.never()`
- Coercion for primitives
  - [ ] `z.coerce.string()`
  - [ ] `z.coerce.number()`
  - [ ] `z.coerce.boolean()`
  - [ ] `z.coerce.bigint()`
  - [ ] `z.coerce.date()`
- `Literals`
  - [ ] `z.literal(value)`
- Strings
  - validations
    - [ ] `z.string().max(5)`
    - [ ] `z.string().min(5)`
    - [ ] `z.string().length(5)`
    - [ ] `z.string().email()`
    - [ ] `z.string().url()`
    - [ ] `z.string().emoji()`
    - [ ] `z.string().uuid()`
    - [ ] `z.string().nanoid()`
    - [ ] `z.string().cuid()`
    - [ ] `z.string().cuid2()`
    - [ ] `z.string().ulid()`
    - [ ] `z.string().regex(regex)`
    - [ ] `z.string().includes(string)`
    - [ ] `z.string().startsWith(string)`
    - [ ] `z.string().endsWith(string)`
    - [ ] `z.string().datetime()`
      - [ ] `z.string().datetime({ offset: true })`
      - [ ] `z.string().datetime({ local: true })`
      - [ ] `z.string().datetime({ precision: 3 })`
      - ISO 8601  
        by default only `Z` timezone allowed
    - [ ] `z.string().ip()`
      - [ ] `z.string().ip({ version: "v4" })`
      - [ ] `z.string().ip({ version: "v6" })`
      - defaults to allow both IPv4 and IPv6
    - [ ] `z.string().cidr()`
      - [ ] `z.string().cidr({ version: "v4" })`
      - [ ] `z.string().cidr({ version: "v6" })`
      - defaults to allow both IPv4 and IPv6
    - [ ] `z.string().date()`
    - [ ] `z.string().time()`
      - [ ] `z.string().time({ precision: 3 })`
    - [ ] `z.string().duration()`
    - [ ] `z.string().base64()`
  - transforms
    - [ ] `z.string().trim()`
    - [ ] `z.string().toLowerCase()`
    - [ ] `z.string().toUpperCase()`
- Numbers
  - [x] `z.number().gt(5)`
  - [x] `z.number().gte(5)`
    - [x] alias `.min(5)`
  - [x] `z.number().lt(5)`
  - [x] `z.number().lte(5)`
    - [x] alias `.max(5)`
  - [x] `z.number().int()`
  - [*] `z.number().positive()`
    - `> 0`
  - [*] `z.number().nonnegative()`
    - `>= 0`
  - [*] `z.number().negative()`
    - `< 0`
  - [*] `z.number().nonpositive()`
    - `<= 0`
  - [*] `z.number().multipleOf(5)`
    - Evenly divisible by 5
    - [*] alias `.step(5)`
  - [*] `z.number().finite()`
    - value must be finite, not Infinity or -Infinity
  - [*] `z.number().safe()`
    - value must be between `Number.MIN_SAFE_INTEGER` and `Number.MAX_SAFE_INTEGER`
- BigInts
  - same as `z.number()`
  - [ ] `z.bigint().gt(5n)`
  - [ ] `z.bigint().gte(5n)`
  - [ ] `z.bigint().lt(5n)`
  - [ ] `z.bigint().lte(5n)`
  - [ ] `z.bigint().positive()`
  - [ ] `z.bigint().nonnegative()`
  - [ ] `z.bigint().negative()`
  - [ ] `z.bigint().nonpositive()`
  - [ ] `z.bigint().multipleOf(5n)`
- NaNs
  - `z.nan()`
- Booleans
  - `z.boolean()`
- Dates
  - `z.date()`
  - `z.date().min(new Date("..."))`
  - `z.date().max(new Date("..."))`
- Zod enums
  - `TODO`
- Native enums
  - `TODO`
- Optionals
  - [ ] `z.optional(z.string())`
  - [ ] `z.string().optional()`
- Nullables
  - [ ] `z.nullable(z.string())`
  - [ ] `z.string().nullable()`
- Objects
  - [ ] `z.object({ name: z.string(), age: z.number() })`
  - [ ] `z.object(...).shape.name` -> `z.string()`
  - [ ] `z.object(...).keyof()` -> `z.enum([ "name", "age" ])`
  - [ ] `z.object(...).extend({...})`
  - [ ] `z.object(...).merge(z.object(...))`
  - [ ] `z.object(...).pick({ name: true })` -> `z.object({ name: z.string() })`
  - [ ] `z.object(...).omit({ name: true })` -> `z.object({ age: z.number() })`
  - [ ] `z.object(...).partial()`
  - [ ] `z.object(...).deepPartial()`
  - [ ] `z.object(...).partial().required()`
  - [ ] `z.object(...).partial().required({ name: true })`
  - [ ] `z.object(...).passthrough()`
    - extraKey has not been stripped
  - [ ] `z.object(...).strict()`
    - extraKey will cause validation error
  - [ ] `z.object(...).strip()`
    - extraKey will be stripped (default)
  - [ ] `z.object(...).catchall(z.number())`
    - extraKey will be validated by the provided schema
- Arrays
  - [ ] `z.array(z.string())`
  - [ ] `z.string().array()`
  - [ ] `z.array(...).element()` -> `z.string()`
  - [ ] `z.array(...).nonempty()`
  - [ ] `z.array(...).min(5)`
  - [ ] `z.array(...).max(10)`
  - [ ] `z.array(...).length(8)`
- Tuples
  - [ ] `z.tuple([ z.string(), z.number() ])`
  - [ ] `z.tuple([ z.string() ]).rest(z.number())`
- Unions
  - [ ] `z.union([z.string(), z.number()])`
  - [ ] `z.string().or(z.number())`
- Discriminated unions
  - `TODO`
- Records
  - [ ] `z.record(z.string(), z.object(...))`
    - will be like `Record<string, {...}>` in TS
- Maps
  - [ ] `z.map(z.string(), z.number())`
- Sets
  - [ ] `z.set(z.string())`
  - [ ] `z.set(...).nonempty()`
  - [ ] `z.set(...).min(5)`
  - [ ] `z.set(...).max(10)`
  - [ ] `z.set(...).size(8)`
- Intersections
  - [ ] `z.intersection(zObj1, zObj2)`
    - will be like `zObj1.and(zObj2)`
  - [ ] `z.intersection(zUnion1, zUnion2)`
    - will be like `zUnion1.and(zUnion2)`
- Recursive types
- ZodEffects
- JSON type
- Cyclical objects
  - `TODO`
- Promises
  - [ ] `z.promise(z.string())`
  - [ ] `z.string().promise()`
- Instanceof
  - [ ] `z.instanceof(AnyClass)`
- Functions
  - [ ] `z.function()`
  - [ ] `z.function().args(...)`
  - [ ] `z.function().returns(...)`
  - [ ] `z.function(..., ...)`
  - [ ] `z.function(..., ...).implement(...)`
  - [ ] `z.function(..., ...).parameters()`
  - [ ] `z.function(..., ...).returnType()`
- Preprocess
  - [ ] `z.preprocess((val) => String(val), z.string())`
- Custom schemas
  - [ ] `z.custom<｀${number}px｀>((val) => typeof val === "string" && /^\d+px$/.test(val))`
- Schema methods
  - [ ] `.parse`
  - [ ] `.parseAsync`
  - [ ] `.safeParse`
  - [ ] `.safeParseAsync`
  - [ ] `.refine`
  - [ ] `.superRefine`
  - [ ] `.transform`
    - [ ] `.pipe`
  - [ ] `.default`
  - [ ] `.describe`
  - [ ] `.catch`
  - [ ] `.optional`
  - [ ] `.nullable`
  - [ ] `.nullish`
  - [ ] `.array`
  - [ ] `.promise`
  - [ ] `.or`
  - [ ] `.and`
  - [ ] `.brand`
  - [ ] `.readonly`
- TS Support
  - Not currently planned
  - There is a possibility of reusing Zod type definitions
