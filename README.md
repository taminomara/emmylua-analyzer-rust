# EmmyLuaAnalyzer-Rust

We welcome your feedback and contributions. Please feel free to submit pull requests (PRs) and report issues to help shape the project's direction.

## Crates

| Crate | Badge | Description |
| ----- | ----- | ----------- |
| [emmylua_parser](./crates/emmylua_parser) | [![emmylua_parser](https://img.shields.io/crates/v/emmylua_parser.svg)](https://crates.io/crates/emmylua_parser) | A Rust-based Lua parser built for efficiency and accuracy. It serves as the foundation for advanced code analysis and the language server. |
| [emmylua_code_analysis](./crates/emmylua_code_analysis) | [![emmylua_code_analysis](https://img.shields.io/crates/v/emmylua_code_analysis.svg)](https://crates.io/crates/emmylua_code_analysis) | Provides Lua code analysis by leveraging emmylua_parser. |
| [emmylua_ls](./crates/emmylua_ls) | [![emmylua_ls](https://img.shields.io/crates/v/emmylua_ls.svg)](https://crates.io/crates/emmylua_ls) | The language server for Lua, offering extensive features for different Lua versions. |
| [emmylua_doc_cli](./crates/emmylua_doc_cli/) | [![emmylua_doc_cli](https://img.shields.io/crates/v/emmylua_doc_cli.svg)](https://crates.io/crates/emmylua_doc_cli) | A command-line tool to effortlessly generate Lua API documentation. |

## Features

- [x] Support for Lua 5.1, 5.2, 5.3, 5.4, and LuaJIT.
- [x] Support Luacats/emmylua annotations.
- [x] Support almost lsp features.

## Documentation

- [Features](./docs/features/features_EN.md)
- [Emmyrc Config](./docs/config/emmyrc_json_EN.md)
- [Formatting Config](https://github.com/CppCXY/EmmyLuaCodeStyle/blob/master/README_EN.md)

## Install

if you want to install emmylua_ls and emmylua_doc_cli, you can use the following command:
```shell
# install emmylua_ls 
cargo install emmylua_ls
# install emmylua_doc_cli
cargo install emmylua_doc_cli
```

if you are using vscode, you can install the vscode extension [EmmyLua](https://marketplace.visualstudio.com/items?itemName=tangzx.emmylua) to get a better experience.

## Usage

### emmylua_ls

If you have installed emmylua_ls using cargo install, you can simply run emmylua_ls to start the language server without any additional parameters.

### emmylua_doc_cli

If you have installed emmylua_doc_cli using cargo install, you can simply run emmylua_doc_cli to generate documentation. You can use the --input parameter to specify the directory of Lua files and the --output parameter to specify the output directory for the generated documentation.

```shell
emmylua_doc_cli --input ./tests/lua --output ./tests/doc
```

## Build

```shell
cargo build --release -p emmylua_ls
```

## Develop

The language server supports both stdio and TCP communication, with stdio communication as the default. It has several startup parameters:
- `-c` specifies the communication method. Acceptable values are `stdio` and `tcp`, with the default being `stdio`.
- `--port` When the `-c` parameter is set to `tcp`, the `--port` parameter can specify the port number, with the default value of `5007`.
- `--log-level` specifies the log level. Acceptable values are `debug`, `info`, `warn`, `error`, with the default being `info`.
- `--log-path` specifies the directory path for the log files.

For example:

```shell
emmylua_ls -c tcp --port 5007 --log-level debug
# Without parameters, it uses stdio communication
emmylua_ls
```

## License

[MIT](./LICENSE)
