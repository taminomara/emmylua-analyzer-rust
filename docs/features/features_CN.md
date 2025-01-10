# emmylua语言服务器的特性介绍

## 自动补全

支持正常的代码补全，包括函数、变量、表字段、模块等。除此以外，EmmyLua在补全上花了很多心思，提供了更多的补全功能。
- `auto require` 支持在自动补全列表列出当前有返回值的lua模块, 并在键入Tab时在文件顶部所有require语句之后添加该模块的require语句。
- `alias enum` 会根据当前函数参数的类型是否为alias或者enum类型，来自动补全对应的alias或者enum字段。
- `function` 会根据当前函数参数的类型是否为函数类型，来自动补全对应的lambda表达式
- `namespace` 如果当前变量的类型是namespace类型，EmmyLua会自动补全namespace下的子空间或者类名, 要申明namespace类型，可以使用`---@type namespace<"类名">`
- `module path` 如果当前所在的字符串是require的参数, Emmylua会补全对应的模块路径, 并且同时支持使用`.`和`/`来分割路径
- `file system path` 在任意字符串中, 如果存在 `/` 或者 `\\` , EmmyLua会试图基于相关配置提供文件系统路径补全
- `postfix` 在任意变量后面输入`@`符号, 会自动补全对应的postfix表达式, 该符号还可改为 `.`
- `snippet` 提供基础的代码片段补全功能, 以后会考虑基于文件模板系统支持自定义模板

## 代码提示

支持正常的通过鼠标悬浮显示变量、函数、表字段、模块等的提示信息。除此以外，EmmyLua在提示上也提供了更多的功能。
- `const` 如果当前变量是常量类型, 那么在鼠标悬浮时会显示该变量的值, 如果是常量表达式, 则会计算出表达式的值

## 代码检查

EmmyLua提供了基于EmmyLua doc的丰富的代码检查功能, 支持通过配置文件来禁止某些检查, 或者启用一些额外的检查。或者通过注释来控制检查的行为. 例如:

```lua
---@diagnostic disable: undefined-global
```
通过这一句可以在当前文件禁用undefined-global的检查

```lua
---@diagnostic disable-next-line: undefined-global
```
通过这一句可以在下一行禁用undefined-global的检查

配置相关的检查功能可以在配置文件中进行配置, 例如:

```json
{
  "diagnostics": {
    "disable": ["undefined-global"]
  }
}
```

## 文档符号

EmmyLua支持在文件中显示结构化的文档符号, 在vscode中可通过左侧的OUTLINE看到, 也可以通过快捷键`Ctrl+Shift+O`打开.

## 工作区符号搜索

EmmyLua支持在工作区中搜索符号, 在vscode中可通过快捷键`Ctrl+T`打开搜索框, 输入`@`符号, 然后输入要搜索的符号名称, 即可搜索到对应的符号.

## 重构

EmmyLua支持在变量和字段的rename重构, 在vscode中可通过快捷键`F2`来进行重构.

## 代码格式化

EmmyLua支持在vscode中使用`Format Document`和`Format Selection`来格式化代码, 格式化的功能使用的是[EmmyLuaCodeStyle](https://github.com/CppCXY/EmmyLuaCodeStyle), 相关配置请参考对应的文档.

## 代码折叠

EmmyLua支持所有正常的代码折叠功能, 包括函数、if、for、while等. 另外EmmyLua还支持折叠注释块, 通过`--region`和`--endregion`来标记折叠区域.

## 代码跳转

EmmyLua支持在vscode中使用`Go to Definition`和`Peek Definition`来跳转到定义, 另外可以通过鼠标左键加`ctrl`键来点击跳转到定义.

## 代码引用

EmmyLua支持在vscode中使用`Find All References`来查找引用, 另外可以通过鼠标左键加`ctrl`键来点击查找引用. 特别地, EmmyLua支持一些特殊的引用查找:
- 字符串引用查找功能: 如果选中一个字符串, 可以通过右键菜单中的`Find All References`来查找该字符串的引用, 或者通过`ctrl` + 鼠标左键来查找引用
- 模糊引用查找功能: 如果选中一个变量, 该变量无法找到定义, 则会尝试进行模糊引用查找, 通过配置文件可以控制是否启用模糊引用查找

## Document Color

EmmyLua会试图分析字符串中连续的16进制数字, 如果连续的数字是6位或者8位, 则会尝试将其解析为颜色值, 并在代码中显示颜色块.

## Document Link

EmmyLua会试图分析字符串中可能存在的的文件路径, 并在代码中显示链接, 可以通过鼠标点击来打开链接.

## 语义高亮

EmmyLua支持LSP中规定的`semanticHighlighting`功能, 会分析token的成分并且给与适当的高亮

## EmmyLua Annotator

EmmyLua通过私有协议强化代码渲染, 例如对可变local变量加上下划线

## inlay hints

EmmyLua支持在代码中显示一些提示信息, 例如参数的类型, 变量的类型. 函数是否是对父类的重写, 函数是否是await调用等等. 可以通过配置文件来控制是否启用这些提示信息.

## document highlight

尽管vscode本身的document highlight功能已经基本满足要求了, 但是EmmyLua还是提供了自己的document highlight功能, 用于高亮变量的引用和同组的关键词, 也考虑到其他编辑器可能需要语言服务器提供这个功能.

