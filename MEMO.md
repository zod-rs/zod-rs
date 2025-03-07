# Memo

## Copy test codes from `colinhacks/zod` to `zod-rs`

```sh
ZOD_VERSION=v3.24.2
git clone --filter=tree:0 --no-checkout --sparse -b $ZOD_VERSION git@github.com:colinhacks/zod.git
cd zod
git sparse-checkout add src/__tests__/
git sparse-checkout add src/helpers/
git sparse-checkout add src/errors.ts
git sparse-checkout add src/locales/
git checkout $ZOD_VERSION src/__tests__ src/helpers src/errors src/locales
cp -r src/__tests__ ../zod-rs/test/
cp -r src/helpers ../zod-rs/test/
cp src/errors.ts ../zod-rs/test/
cp -r src/locales ../zod-rs/test/
```
