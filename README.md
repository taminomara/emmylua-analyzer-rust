# EmmyLuaAnalyzer-Rust

We welcome your feedback and contributions. Please feel free to submit pull requests (PRs) and report issues to help shape the project's direction.

## crates

- [`emmylua_parser`](./crates/emmylua_parser): A Lua parser written in Rust, designed to provide efficient and accurate parsing of Lua scripts. This crate serves as the foundation for other tools in the project, enabling robust code analysis and language server functionalities.
- [`emmylua_code_analysis`](./crates/emmylua_code_analysis): lua code analysis base on emmylua_parser.
- [`emmylua_ls`](./crates/emmylua_ls): language server for Lua.
- [`emmylua_doc_cli`](./crates/emmylua_doc_cli/): A command-line tool for generating Lua API documentation.

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

## Develop

The language service supports both stdio and TCP communication, with stdio communication as the default. It has several startup parameters:
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

However, having only the executable is not enough. It needs to load some resource files, which are located in the project's `resources` directory. By default, it will first look for the `resources` directory in the current directory or its parent directories. Alternatively, you can specify the path to the resources directory through the `EMMYLUA_LS_RESOURCES` environment variable.


## License

[MIT](./LICENSE)
