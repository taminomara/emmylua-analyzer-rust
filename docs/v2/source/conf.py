# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

import sys
import get_version as _get_version
import pathlib as _pathlib
import datetime as _datetime

_src_root = _pathlib.Path(__file__).parent
_vcs_root = _get_version.find_vcs_root(_src_root)
assert _vcs_root, "failed to find git root"

sys.path.insert(0, str(_src_root / "_extensions"))

project = 'EmmyLua Analyzer'
copyright = f'{_datetime.date.today().year}, CppCXY'
author = 'CppCXY and contributors'
release = _get_version.get_version_from_vcs(_vcs_root)

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "myst_parser",
    "sphinx_immaterial",
    "sphinx_lua_ls",
    "emmyrc",
]

primary_domain = "lua"

lua_ls_project_root = "../../../crates/emmylua_code_analysis/resources/std"
lua_ls_backend = "emmylua"

emmylua_schema = "../../../crates/emmylua_code_analysis/resources/schema.json"

myst_enable_extensions = {"colon_fence"}

html_theme = 'sphinx_immaterial'
html_static_path = ['_static']
html_theme_options = {
    "icon": {
        "repo": "fontawesome/brands/github",
        # "edit": "material/file-edit-outline",
    },
    "site_url": "https://EmmyLuaLs.github.io/emmylua-analyzer-rust/",
    "repo_url": "https://github.com/EmmyLuaLs/emmylua-analyzer-rust/",
    "edit_uri": "blob/main/docs/v2",
    "globaltoc_collapse": True,
    "features": [
        "content.action.edit",
        "content.action.view",
        "content.code.annotate",
        "content.code.copy",
        "content.tabs.link",
        # "content.tooltips",
        "navigation.instant",
        "navigation.sections",
        "navigation.tabs",
        "navigation.top",
        "search.highlight",
        "search.suggest",
        "toc.follow",
        "toc.sticky",
        "announce.dismiss",
    ],
    "palette": [
        {
            "media": "(prefers-color-scheme)",
            "toggle": {
                "icon": "material/brightness-auto",
                "name": "Switch to light mode",
            },
        },
        {
            "media": "(prefers-color-scheme: light)",
            "scheme": "default",
            "primary": "blue",
            "accent": "blue",
            "toggle": {
                "icon": "material/lightbulb",
                "name": "Switch to dark mode",
            },
        },
        {
            "media": "(prefers-color-scheme: dark)",
            "scheme": "slate",
            "primary": "black",
            "accent": "blue",
            "toggle": {
                "icon": "material/lightbulb-outline",
                "name": "Switch to system preference",
            },
        },
    ],
    "toc_title_is_page_title": True,
    "languages": [
        {
            "name": "English",
            "link": "en/",
            "lang": "en",
        },
        {
            "name": "Chinese",
            "link": "cn/",
            "lang": "cn",
        },
    ]
    # # BEGIN: social icons
    # "social": [
    #     {
    #         "icon": "fontawesome/brands/github",
    #         "link": "https://github.com/jbms/sphinx-immaterial",
    #         "name": "Source on github.com",
    #     },
    #     {
    #         "icon": "fontawesome/brands/python",
    #         "link": "https://pypi.org/project/sphinx-immaterial/",
    #     },
    # ],
    # # END: social icons
}
