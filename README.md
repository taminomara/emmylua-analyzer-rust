# EmmyLuaAnalyzer-Rust

We welcome your feedback and contributions. Please feel free to submit pull requests (PRs) and report issues to help shape the project's direction.

## crates

- [`emmylua_parser`](./crates/emmylua_parser): A Lua parser written in Rust, designed to provide efficient and accurate parsing of Lua scripts. This crate serves as the foundation for other tools in the project, enabling robust code analysis and language server functionalities.
- [`code_analysis`](./crates/code_analysis): lua code analysis base on emmylua_parser.
- [`emmylua_ls`](./crates/emmylua_ls): language server for Lua.
- [`meta_text`](./crates/meta_text): A library for manipulating text with meta information.

## Features

- [x] Support for Lua 5.1, 5.2, 5.3, 5.4, and LuaJIT.
- [x] Support Luacats/emmylua annotations.
- [x] Support almost lsp features.

## Documentation

- [Features](./docs/features/features_EN.md)
- [Emmyrc Config](./docs/config/emmyrc_json_EN.md)
- [Formatting Config](https://github.com/CppCXY/EmmyLuaCodeStyle/blob/master/README_EN.md)

## Build

```shell
cargo build --release -p emmylua_ls
```

## License

[MIT](./LICENSE)
