# CHANGELOG

# 0.3.4 (unreleased)

# 0.3.3

`NEW` Support release to crates.io

`NEW` Add Develop Guide

`NEW` support `workspace/didChangeConfiguration` notification for neovim

`CHG` refactor `semantic token`

# 0.3.2

`FIX` Fixed some multiple return value inference errors

`FIX` Removed redundant `@return` in hover

`NEW` Language server supports locating resource files through the `$EMMYLUA_LS_RESOURCES` variable

# 0.3.1

`FIX` Fixed a potential issue where indexing could not be completed

`FIX` Fixed an issue where type checking failed when passing subclass parameters to a parent class

# 0.3.0

`NEW` Add progress notifications for non-VSCode platforms

`NEW` Add nix flake for installation by nix users, refer to PR#4

`NEW` Support displaying parameter and return descriptions in hover

`NEW` Support viewing consecutive require statements as import blocks, automatically folded by VSCode if the file only contains require statements

`FIX` Fix spelling error `interger` -> `integer`

`FIX` Fix URL parsing issue when the first directory under a drive letter is in Chinese

`FIX` Fix type checking issues related to tables

`FIX` Modify the implementation of document color, requiring continuous words, and provide an option to disable the document color feature

`FIX` Fix type inference issue when `self` is used as an explicit function parameter