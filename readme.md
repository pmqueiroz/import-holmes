[peam-url]: https://pmqueiroz.com
[src-url]: https://swc.rs/

![import-holmes](./.github/brand.png)


<div align="center">

<samp> **Import Holmes** is a tool to inspect Javascript/Typescript projects imports</samp> 

</div>

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
> **Warning** if your intention is use the core functions in your project consider not using global install

### Cli

```sh
import-holmes -m react # search all imports from react library
import-holmes -s useState # search all 'useState' specifier import
```

### Core

```ts
import { parseModule } from 'import-holmes'

const someCode = `\
import a from 'b'
import { c } from 'd'
import { e as f } from 'g'
`

const inspect = parseModule(someCode)

// outputs
[
  { specifier: 'a', moduleName: 'b' },
  { specifier: 'c', moduleName: 'd' },
  { specifier: 'e', moduleName: 'g' }
]
```

<div align="center">

<samp>Made with :heart: by [**Peam**][peam-url]</samp> 

</div>
