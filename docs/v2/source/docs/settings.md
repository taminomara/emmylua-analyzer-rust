# Settings

EmmyLua loads settings from `.emmyrc.json` located in project's root.
For settings that aren't listed in project's `.emmyrc.json`, EmmyLua will search
for user-specific defaults.

:::{info} Here's a full list of places where EmmyLua will look
  :collapsible:

  1. local `.emmyrc.json`: a JSON file located in your project's root,
  1. local `.luarc.json`: a JSON file located in your project's root,
  1. a JSON file specified by environment variable `$EMMYLUALS_CONFIG`,
  1. global `<os-specific config-dir>/emmylua_ls/.emmyrc.json`,
  1. global `<os-specific config-dir>/emmylua_ls/.luarc.json`,
  1. global `<os-specific home-dir>/.emmyrc.json`,
  1. global `<os-specific home-dir>/.luarc.json`.

  Depending on your platform, `<os-specific config-dir>` will be different:

  |Platform | Value                                 | Example                                  |
  | ------- | ------------------------------------- | ---------------------------------------- |
  | Linux   | `$XDG_CONFIG_HOME` or `$HOME/.config` | /home/alice/.config                      |
  | macOS   | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support |
  | Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming           |
:::

Additionally, some editors allow overriding project's config values.
See {doc}`installation instructions <installation>` for details.

:::{tip}
  Only add project-specific setting to your project's `.emmyrc.json`. For example,
  it's a good idea to set up {emmyrc:obj}`Lua version <EmmyRc.runtime.version>`
  or {emmyrc:obj}`naming convention for auto-required paths <EmmyRc.completion.autoRequireNamingConvention>`,
  but overriding {emmyrc:obj}`settings for inlay hints <EmmyRc.hint>` would serve
  no purpose.
:::

```{emmyrc:auto} EmmyRc
:title: Full list of config values
:recursive:
:unwrap:
:hide-prefix:
:exclude: EmmyRc $schema, EmmyRc#/$defs/DiagnosticCode
```

----

```{lua:autoobject} nil
```

```{lua:autoobject} boolean
```

```{lua:autoobject} number
```

```{lua:autoobject} integer
```

```{lua:autoobject} userdata
```

```{lua:autoobject} lightuserdata
```

```{lua:autoobject} thread
```

```{lua:autoobject} table
```

```{lua:autoobject} any
```

```{lua:autoobject} unknown
```

```{lua:autoobject} void
```

```{lua:autoobject} self
```
