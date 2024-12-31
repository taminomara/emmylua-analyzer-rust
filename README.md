# EmmyLuaAnalyzer-Rust

This project is in its early stages but already offers basic functionality. Future development will focus on expanding features, improving stability, and refining its overall usability. Your feedback and contributions are welcome to help shape its direction.

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

Todo: add documentation.

## Build

```shell
cargo build --release -p emmylua_ls
```

## License

[MIT](./LICENSE)
