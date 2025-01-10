# Introduction to the features of the EmmyLua Language Server

[中文介绍](./features_CN.md)

## Auto Completion

Supports standard code completion, including functions, variables, table fields, and modules. In addition, EmmyLua includes more advanced completion features:
- `auto require` Shows Lua modules that have return values in the completion list. When pressing Tab, it automatically adds the require statement for that module at the top of the file after existing require statements.
- `alias enum` Automatically completes corresponding alias or enum fields based on the parameter type in the current function.
- `function` Suggests lambda expressions if the current function parameter is a function type.
- `namespace` If the current variable is a namespace type, EmmyLua automatically completes sub-namespaces or class names. Declare a namespace type using `---@type namespace<"ClassName">`.
- `module path` When typing within the argument of require, EmmyLua completes module paths, supporting both '.' and '/' as separators.
- `file system path` In any string containing '/' or '\\', EmmyLua tries to complete file system paths based on relevant settings.
- `postfix` When typing the '@' symbol after any variable, the corresponding postfix expressions are suggested. This symbol can also be replaced with '.'.
- `snippet` Provides basic code snippet completions. Future releases may support custom templates via a file template system.

## Code Hints

Displays traditional hover hints for variables, functions, table fields, and modules. EmmyLua also offers additional features for code hints:
- `const` If the current variable is a constant type, hovering over it displays the variable’s value. For constant expressions, EmmyLua calculates and shows the expression result.

## Code Checks

EmmyLua leverages EmmyLua doc for comprehensive code checks. These checks can be disabled or enabled via configuration files, or controlled using annotations such as:
```lua
---@diagnostic disable: undefined-global
```
This disables undefined-global checks in the current file.

```lua
---@diagnostic disable-next-line: undefined-global
```
This disables undefined-global checks for the following line.

You can configure these checks in a config file, for example:
```json
{
  "diagnostics": {
    "disable": ["undefined-global"]
  }
}
```

## Document Symbols

EmmyLua supports structured document symbols. In VSCode, these can be viewed in the OUTLINE panel on the left or by pressing Ctrl+Shift+O.

## Workspace Symbol Search

EmmyLua supports workspace-wide symbol searches. In VSCode, press Ctrl+T, type '@', then the symbol name to search for the corresponding symbol.

## Refactoring

EmmyLua supports variable and field rename refactoring. In VSCode, use the F2 shortcut for this operation.

## Code Formatting

EmmyLua supports the "Format Document" and "Format Selection" features in VSCode, using [EmmyLuaCodeStyle](https://github.com/CppCXY/EmmyLuaCodeStyle). Refer to its documentation for related configurations.

## Code Folding

EmmyLua supports standard code folding for functions, if-statements, for-loops, while-loops, etc. Additionally, it supports folding for comment blocks marked by `--region` and `--endregion`.

## Go to Definition

In VSCode, EmmyLua supports "Go to Definition" and "Peek Definition." You can also Ctrl+click with the mouse to jump to definition.

## Find References

EmmyLua supports "Find All References" in VSCode. You can also Ctrl+click with the mouse to find references. Special cases include:
- String reference search: If you select a string, you can use "Find All References" or Ctrl+click to find all references of that string.
- Fuzzy references: If a selected variable has no definition, EmmyLua attempts a fuzzy reference search. This can be enabled or disabled via the configuration file.

## Document Color

EmmyLua attempts to detect sequences of six or eight hexadecimal digits in a string and interpret them as color values, displaying a color preview.

## Document Link

EmmyLua tries to detect possible file paths in strings and displays clickable links to open those paths.

## Semantic Highlighting

EmmyLua supports `semanticHighlighting` as defined by the LSP, assigning appropriate highlighting based on token analysis.

## EmmyLua Annotator

EmmyLua extends code rendering through a private protocol, for example, underlining mutable local variables.

## Inlay Hints

EmmyLua can display additional hints, such as parameter types, variable types, and whether a function overrides a parent class method or is an await call. These hints can be enabled or disabled via configuration.

## Document Highlight

Although VSCode’s built-in document highlight is sufficient, EmmyLua also provides its own highlight feature for variable references and related keywords. This is especially useful for editors that require language server-based highlighting.

