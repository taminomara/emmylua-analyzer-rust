# Configuration Description

[中文文档](./emmyrc_json_CN.md)

The language server reads the ".emmyrc.json" file in the project root directory. For compatibility, it also reads a ".luarc.json" file. The ".emmyrc.json" format is similar to ".luarc.json" but provides richer options, and any settings in ".emmyrc.json" will override those in ".luarc.json". The two formats are not fully compatible, so unsupported parts in ".luarc.json" are ignored.

It primarily follows this format:
```json
{
  "completion": {
    "enable": true,
    "autoRequire": true,
    "autoRequireFunction": "require",
    "autoRequireNamingConvention": "keep",
    "callSnippet": false,
    "postfix": "@"
  },
  "signature": {
    "detailSignatureHelper": false
  },
  "diagnostics": {
    "disable": [
    ],
    "globals": [],
    "globalsRegex": [],
    "severity": {
    },
    "enables": [
    ]
  },
  "hint": {
    "enable": true,
    "paramHint": true,
    "indexHint": true,
    "localHint": true,
    "overrideHint": true
  },
  "runtime": {
    "version": "Lua5.4",
    "requireLikeFunction": [],
    "frameworkVersions": [],
    "extensions": [],
    "requirePattern": []
  },
  "workspace": {
    "ignoreDir": [

    ],
    "ignoreGlobs": [
    ],
    "library": [],
    "workspaceRoots": [],
    "encoding": "",
    "moduleMap": []
  },
  "resource": {
    "paths": [
    ]
  },
  "codeLens": {
    "enable": true
  },
  "strict": {
    "requirePath": false,
    "typeCall": true
  },
  "hover": {
    "enable": true
  },
  "references": {
    "enable": true,
    "fuzzy_search": true
  }
}
```

To enable automatic completion and IntelliSense for this configuration file, you can add a `"$schema"` field pointing to "resources/schema.json".

## completion
- `enable`: Whether or not to enable completion. Default is `true`.
- `autoRequire`: Whether or not to auto-complete require statements. Default is `true`.
- `autoRequireFunction`: The function name for auto-completing require statements. Default is `require`.
- `autoRequireNamingConvention`: Naming convention for auto-completing require statements. Default is `keep`; possible values are `keep`, `camel-case`, `snake-case`, `pascal-case`.
- `callSnippet`: Whether to expand function calls with snippets. Default is `false`.
- `postfix`: Postfix symbol for completion. Default is `@`.

## signature
- `detailSignatureHelper`: Whether to display detailed function signatures. Default is `false`.

## diagnostics
- `enable`: Whether or not to enable diagnostics. Default is `true`.
- `disable`: A list of diagnostic IDs to be disabled (e.g., `"undefined-global"`).
- `globals`: A list of global variables exempt from "undefined" checks.
- `globalsRegex`: A list of regex patterns for exempting globals from "undefined" checks.
- `severity`: Diagnostic severity mapping, e.g., `"undefined-global": "warning"`. Possible values: `"error"`, `"warning"`, `"information"`, `"hint"`.
- `enables`: A list of diagnostic IDs to enable if they are not already enabled by default (e.g., `"undefined-field"`).

## hint
- `enable`: Whether or not to enable hints. Default is `true`.
- `paramHint`: Whether or not to display parameter hints. Default is `true`.
- `indexHint`: Whether or not to show hints when indexing spans multiple lines. Default is `true`.
- `localHint`: Whether or not to show local variable hints. Default is `true`.
- `overrideHint`: Whether or not to show override hints. Default is `true`.

## runtime
- `version`: Lua runtime version, defaults to `Lua5.4`. Possible values: `Lua5.1`, `Lua5.2`, `Lua5.3`, `Lua5.4`, `LuaJIT`.
- `requireLikeFunction`: Functions treated like require (e.g., `["import"]`).
- `frameworkVersions`: Framework identifiers (e.g., `["love2d"]`) that can work with emmylua doc’s version tag.
- `extensions`: File extensions to treat as Lua files (e.g., `[".lua", ".lua.txt"]`).
- `requirePattern`: Patterns for matching Lua modules (defaults to `["?.lua", "?/init.lua"]`).

## workspace
- `ignoreDir`: Directories to ignore (e.g., `["build", "dist"]`).
- `ignoreGlobs`: Files to ignore based on patterns (e.g., `["*.log", "*.tmp"]`).
- `library`: Directories containing additional libraries (e.g., `["/usr/local/lib"]`).
- `workspaceRoots`: A list of workspace root directories (e.g., `["Assets/script/Lua"]`).
- `preloadFileSize`: Maximum file size for preloading, default `1048576` bytes.
- `encoding`: File encoding for reads, default is `utf-8`.
- `moduleMap`: Module mapping list used to specify module mappings, for example:
```json
{ 
  "pattern" : "^lib(.*)$", 
  "replace" : "script$1"
}
```

This feature is mainly to make `require` work correctly. If you need to map modules starting with `lib` to those starting with `script`, you need to add the mapping relationship here.

## resource
- `paths`: Resource directories to load (e.g., `["Assets/settings"]`). By default, the current workspace directory is used, and emmylua can provide completion and jump-to-definition for file paths within strings.

## codeLens
- `enable`: Whether or not to enable CodeLens. Default is `true`.

## strict
- `requirePath`: Whether or not to enable strict mode for require. Default is `true`.
- `typeCall`: Whether or not to enable strict type calls. Default is `true`.

## hover
- `enable`: Whether or not to enable hover support. Default is `true`.

## references
- `enable`: Whether or not to enable references. Default is `true`.
- `fuzzy_search`: Whether or not to enable fuzzy search in references. Default is `true`.