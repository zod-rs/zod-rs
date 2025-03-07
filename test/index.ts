import pkg from "@zod-rs/wasm"
import { z as Zod } from "zod"
// import pkg from "../vendor/pkg"


(async () => {
  // (await pkg()).greet()
  
  console.log(pkg.is_number(1))
  console.log(pkg.is_number("1"))
  console.log(pkg.is_number("A"))

})()


// const schema = z.object({
//   name: z.string(),
//   age: z.number(),
// });

// const data = {
//   name: "Alice",
//   age: 30,
// };

// const parsed = schema.safeParse(data);

// if (!parsed.success) {
//   console.error("Validation errors:", parsed.error);
// } else {
//   console.log("Validated data:", parsed.data);
// }
