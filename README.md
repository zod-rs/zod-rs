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

```
