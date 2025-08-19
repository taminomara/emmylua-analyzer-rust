from __future__ import annotations

from emmyrc.auto import AutodocDirective
from emmyrc.domain import EmmyRcDomain
from sphinx.application import Sphinx


def setup(app: Sphinx):
    app.add_domain(EmmyRcDomain)
    app.add_directive_to_domain("emmyrc", "auto", AutodocDirective)
    app.add_config_value("emmylua_schema", None, "html", str)
    return {
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }
