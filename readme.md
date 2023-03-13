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

#### Config File

For better configuring you can also set a config file named `.holmesrc.json` following this pattern:

```jsonc
// .holmesrc.json
{
  "$schema": "https://raw.githubusercontent.com/pmqueiroz/import-holmes/main/schema.json",
  "module": "some-module", // you can pass string[] as well
  "specifier": ["first", "second"], // you can pass a sting
  "glob": "**/*.{ts,tsx}",
  "globIgnore": ["node_modules/**", "**/*.{spec,test}.{ts,tsx}", "**/*.d.ts"],
  "parseConfig": { // refer to https://swc.rs/docs/usage/core#parse
    "syntax": "typescript",
    "tsx": true
  }
}
```

### Core

This package also provides the core function under the cli.

#### inspectModule

inspects a typescript/javascript module searching for import declarations and returns results from imports 

```ts
import { inspectModule } from 'import-holmes'

const someCode = `\
import A from 'b'
import { c } from 'd'
import { e as f } from 'g'

new A()

c()

const h = f
`

const inspect = inspectModule(someCode)

// outputs
[
  { specifier: 'A', moduleName: 'b', referenced: 1 },
  { specifier: 'c', moduleName: 'd', referenced: 1 },
  { specifier: 'e', moduleName: 'g', referenced: 1 }
]
```
There are some available options in inspect module

```ts
inspectModule('source code', {
   filename?: string
   modulesFilter?: string | string[]
   specifiersFilter?: string | string[]
   parseConfig?: ParserConfig // refer to https://swc.rs/docs/usage/core#parse
})
```

### known Issues

Referenced column may not work well with some typings, such as type referenced in union type

<div align="center">

<samp>Made with :heart: by [**Peam**][peam-url]</samp> 

</div>
