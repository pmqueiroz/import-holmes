[peam-url]: https://pmqueiroz.com
[src-url]: https://swc.rs/
[issues-url]: https://github.com/pmqueiroz/import-holmes/issues

![import-holmes](./.github/brand.png)

<div align="center">

<samp> **Import Holmes** is a tool to inspect projects imports.</samp>

</div>

## Languages Supported
- [x] TypeScript
- [x] JavaScript
- [x] Kotlin

> **Warning** this tool isn't in a stable version and can change
at any moment, check [issues][issues-url] page to check futures features and current problems

## Getting Started

### Installing

```sh
brew tap pmqueiroz/tap
brew install import-holmes
```

### Cli args

| Options | Default | Meaning | Example |
|---------|:-------:|---------|--|
| `-m`, `--module` | ts/js: `package.json.dependencies` kt: `*`| Filter inspection by module's name | `-m a,b` |
| `-s`, `--specifiers` | - | Filter inspection by specifiers name | `-s Button,Text` |
| `-g`, `--glob` | ts/js: `**/*.{ts,tsx}` kt: `**/*.kt` | Glob to select files to inspect | `-g components/*.{js}` |
| `-o`, `--output` | `table` | Configure the output type | `-o json` `-o csv` |
| `--sort` | `none` | Sort the output by `ocurrences` or `referenced` | `--sort referenced` |
| `--language` | `typescript` | Select which language parser use | `--language kotlin` |

#### Config File

For better configuring you can also set a config file named `.holmesrc.json` following this pattern:

```jsonc
// .holmesrc.json
{
  "$schema": "https://raw.githubusercontent.com/pmqueiroz/import-holmes/main/schema.json",
  "module": ["some-module"],
  "specifiers": ["first", "second"],
  "include": ["**/*.{ts,tsx}"],
  "exclude": ["node_modules/**", "**/*.{spec,test}.{ts,tsx}", "**/*.d.ts"],
  "sortStrategy": "referenced",
  "output": "json",
  "language": "typescript
}
```


## Demo

![demo](./.github/demo.gif)


<div align="center">

<samp>Made with :heart: by [**Peam**][peam-url]</samp> 

</div>
