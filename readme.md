[peam-url]: https://pmqueiroz.com
[src-url]: https://swc.rs/
[issues-url]: https://github.com/pmqueiroz/import-holmes/issues

![import-holmes](./.github/brand.png)

<div align="center">

<samp> **Import Holmes** is a tool to inspect Javascript/Typescript projects imports</samp> 

</div>

> **Warning** this tool isn't in a stable version, check [issues][issues-url] page to check futures features and current problems

## Getting Started

### Installing

```sh
# recommended
npx import-holmes
# or
pnpm add -g import-holmes
# or
yarn global add import-holmes
# or
npm add --location=global import-holmes
```
> **Warning** if your intention is to use the core functions in your project consider not using global install

### Cli

```sh
import-holmes
# or
import-holmes inspect
```

| Options | Default | Meaning |
|---------|---------|---------|
| `-m`, `--module` | all `dependencies` and `devDependencies` | Filter inspection by module's name |
| `-s`, `--specifier` | `undefined` | Filter inspection by specifier name |
| `-g`, `--glob` | `**/*.{ts,tsx}` | Glob to select files to inspect |


### Core

This package also provides the core function under the cli.

#### inspectModule

inspects a typescript/javascript module searching for import declarations and returns results from imports 

```ts
import { inspectModule } from 'import-holmes'

const someCode = `\
import a from 'b'
import { c } from 'd'
import { e as f } from 'g'
`

const inspect = inspectModule(someCode)

// outputs
[
  { specifier: 'a', moduleName: 'b' },
  { specifier: 'c', moduleName: 'd' },
  { specifier: 'e', moduleName: 'g' }
]
```
There are some available options in inspect module

```ts
inspectModule('source code', {
   modulesFilter?: string | string[]
   specifiersFilter?: string | string[]
})
```

<div align="center">

<samp>Made with :heart: by [**Peam**][peam-url]</samp> 

</div>
