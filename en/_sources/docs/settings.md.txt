# Settings

EmmyLua loads settings from `.emmyrc.json` located in project's root.
For settings that aren't listed in project's `.emmyrc.json`, EmmyLua will search
for user-specific defaults.

:::{spoiler} Here's a full list of places where EmmyLua will look
  :collapsible:

  1. local `.emmyrc.json`: a JSON file located in your project's root,
  2. local `.luarc.json`: a JSON file located in your project's root,
  3. a JSON file specified by environment variable `$EMMYLUALS_CONFIG`,
  4. global `<os-specific config-dir>/emmylua_ls/.emmyrc.json`,
  5. global `<os-specific config-dir>/emmylua_ls/.luarc.json`,
  6. global `<os-specific home-dir>/.emmyrc.json`,
  7. global `<os-specific home-dir>/.luarc.json`.

  Depending on your platform, `<os-specific config-dir>` will be different:

  | Platform | Value                                 | Example                                  |
  |----------|---------------------------------------|------------------------------------------|
  | Linux    | `$XDG_CONFIG_HOME` or `$HOME/.config` | /home/alice/.config                      |
  | macOS    | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support |
  | Windows  | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming           |
:::

Additionally, some editors allow overriding project's config values.
See [installation instructions](installation) for details.

:::{tip}
  Only add project-specific setting to your project's `.emmyrc.json`. For example,
  it's a good idea to set up [Lua version](#EmmyRc.runtime.version)
  or [naming convention for auto-required paths](#EmmyRc.completion.autoRequireNamingConvention),
  but overriding [settings for inlay hints](#EmmyRc.hint) would serve
  no purpose.
:::

## Template

You can use one of these templates to kick-start the configuration process.
Each of them contains all options relevant to the use-case,
along with current defaults.

:::{spoiler} A template for project-specific `.emmyrc.json`
  :collapsible:

  ```{emmyrc:auto-example} EmmyRc
  :kind: project
  ```
:::

:::{spoiler} A template for user-specific `.emmyrc.json`
  :collapsible:

  ```{emmyrc:auto-example} EmmyRc
  :kind: user
  ```
:::

## Schema

To enable intelligent completion and validation for configuration files,
you can add a schema reference to your configuration file:

```json
{
  "$schema": "https://raw.githubusercontent.com/EmmyLuaLs/emmylua-analyzer-rust/refs/heads/main/crates/emmylua_code_analysis/resources/schema.json"
}
```

## Full list of config values

```{emmyrc:auto} EmmyRc
:recursive:
:unwrap:
:hide-prefix:
:exclude: EmmyRc $schema, EmmyRc#/$defs/DiagnosticCode
```
