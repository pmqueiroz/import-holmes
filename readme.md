[peam-url]: https://pmqueiroz.com
[src-url]: https://swc.rs/
[issues-url]: https://github.com/pmqueiroz/import-holmes/issues

![import-holmes](./.github/brand.png)

<div align="center">

<samp> **Import Holmes** is a tool to inspect Javascript/Typescript projects imports</samp> 

</div>

> **Warning** this tool isn't in a stable version and can change
at any moment, check [issues][issues-url] page to check futures features and current problems

## Getting Started

### Installing

```sh
brew tap pmqueiroz/tap
brew install import-holmes
```

### Configuring

| Options | Default | Meaning | Example |
|---------|:-------:|---------|--|
| `-m`, `--module` | `package.json.dependencies`| Filter inspection by module's name | `-m a,b` |
| `-s`, `--specifier` | - | Filter inspection by specifier name | `-s Button` |
| `-g`, `--glob` | `**/*.{ts,tsx}` | Glob to select files to inspect | `-g components/*.{js}` |

#### Config File

For better configuring you can also set a config file named `.holmesrc.json` following this pattern:

```jsonc
// .holmesrc.json
{
  "$schema": "https://raw.githubusercontent.com/pmqueiroz/import-holmes/main/schema.json",
  "module": ["some-module"],
  "specifier": ["first", "second"],
  "include": "**/*.{ts,tsx}",
  "exclude": ["node_modules/**", "**/*.{spec,test}.{ts,tsx}", "**/*.d.ts"],
  "sortStrategy": "referenced"
}
```

### Core

```rs
use inspect_core::{inspect_module};
use std::fs;

fn main() {
  let module = fs::read_to_string("index.js").expect("File does not exits");
  let inspect: inspect_core::Inspect = inspect_module(&module);

  println!("{:#?}", inspect)
}

```

### Known Issues

[Import alias](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#aliasn) will no count as referenced.

For example this module:

```tsx
import { Button as DesignSystemButton } from 'design-system'

export function Page() {
  return (
    <div>
      <DesignSystemButton>One</DesignSystemButton>
      <DesignSystemButton>Two</DesignSystemButton>
    </div>
  )
}
```
will result in:

```rs
Inspect {
  raw: RawInspect {
    specifier: "Button",
    module_name: "design-system"
  },
  referenced: 0,
  occurrences: 1
}
```

<div align="center">

<samp>Made with :heart: by [**Peam**][peam-url]</samp> 

</div>
