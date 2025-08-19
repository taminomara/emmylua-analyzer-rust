from __future__ import annotations

import contextlib
import dataclasses
import json
import pathlib
import typing as _t
from dataclasses import dataclass
from urllib.parse import urldefrag, urljoin

import referencing
import sphinx.application
from frozendict import deepfreeze, frozendict
from sphinx.locale import _


@dataclass(frozen=True, kw_only=True)
class Type:
    id: str = dataclasses.field(default="", compare=False, hash=False)
    title: str | None = dataclasses.field(default=None, compare=False, hash=False)
    description: str | None = dataclasses.field(default=None, compare=False, hash=False)
    deprecated: bool = dataclasses.field(default=False, compare=False, hash=False)
    required: bool = dataclasses.field(default=False, compare=False, hash=False)
    default: _t.Any | None = dataclasses.field(default=None, compare=False, hash=False)
    const: _t.Any | None = None

    def __str__(self) -> str:
        raise NotImplementedError("use `print` instead")

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        raise NotImplementedError("override `print` in subclass")

    def unwrap_optional(self) -> Type:
        return self

    def find_refs(self) -> list[Ref]:
        return []

    def can_inline(self) -> bool:
        return True


@dataclass(frozen=True, kw_only=True)
class Object(Type):
    properties: Type | None = None
    named_children: frozendict[str, Type] = dataclasses.field(
        default_factory=lambda: frozendict()
    )

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        items = ", ".join(
            f"{name!r}: {ch.print(loader)}" for name, ch in self.named_children.items()
        )
        if items:
            return f"{{{items}}}"
        elif self.properties:
            return f"{{string: {self.properties.print(loader)}}}"
        else:
            return "{}"

    def find_refs(self) -> list[Ref]:
        res = [
            ref for ch in self.named_children.values() for ref in ch.find_refs()
        ]
        if self.properties:
            res.extend(self.properties.find_refs())
        return res


@dataclass(frozen=True)
class Ref(Type):
    ref: str

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        if not loader:
            return self.ref

        try:
            type = loader.load(self.ref)
        except Exception:
            return self.ref
        if loader.can_inline(self.ref):
            return type.print(loader, _parens)
        else:
            return self.ref

    def find_refs(self) -> list[Ref]:
        return [self]


@dataclass(frozen=True)
class Array(Type):
    item: Type

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return f"{self.item.print(loader, True)}[]"

    def find_refs(self) -> list[Ref]:
        return self.item.find_refs()


@dataclass(frozen=True)
class Set(Type):
    item: Type

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return f"{self.item.print(loader, True)}[]"

    def find_refs(self) -> list[Ref]:
        return self.item.find_refs()


@dataclass(frozen=True)
class Tuple(Type):
    items: tuple[Type, ...]
    tail: Type | None

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        items = ", ".join(item.print(loader, False) for item in self.items)
        if self.tail:
            if items:
                items += ", "
            items += str(self.tail) + "..."
        return f"[{items}]"

    def find_refs(self) -> list[Ref]:
        res = [
            ref for ch in self.items for ref in ch.find_refs()
        ]
        if self.tail:
            res.extend(self.tail.find_refs())
        return res


@dataclass(frozen=True)
class Null(Type):
    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return "null" if self.const is None else json.dumps(self.const)


@dataclass(frozen=True)
class Boolean(Type):
    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return "boolean" if self.const is None else json.dumps(self.const)


@dataclass(frozen=True)
class Integer(Type):
    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return "integer" if self.const is None else json.dumps(self.const)


@dataclass(frozen=True)
class Number(Type):
    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return "number" if self.const is None else json.dumps(self.const)


@dataclass(frozen=True)
class String(Type):
    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return "string" if self.const is None else json.dumps(self.const)


@dataclass(frozen=True)
class Any(Type):
    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        return "any" if self.const is None else json.dumps(self.const)


@dataclass(frozen=True)
class OneOf(Type):
    items: tuple[Type, ...]
    contains_null: bool = False

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        items = (
            " | ".join(
                item.print(loader, True)
                for item in self.items
                if not isinstance(item, Null)
            )
            or "never"
        )

        if self.contains_null and len(self.items) > 2:
            return f"({items})?"
        elif self.contains_null:
            return f"{items}?"
        elif _parens:
            return f"({items})"
        else:
            return items

    def unwrap_optional(self) -> Type:
        if self.contains_null and len(self.items) == 2:
            for item in self.items:
                if not isinstance(item, Null):
                    return item.unwrap_optional()
        return self

    @staticmethod
    def create(types: _t.Iterable[Type]) -> Type:
        flat = []
        for type in types:
            if isinstance(type, OneOf):
                flat.extend(type.items)
            else:
                flat.append(type)
        result_set = set()
        for type in flat:
            if (
                isinstance(type, (Null, Boolean, Integer, Number, String, Any))
                and type.const is not None
            ):
                result_set.add(dataclasses.replace(type, const=None))
        result: list[Type] = []
        contains_null = False
        for type in flat:
            if isinstance(type, Null):
                contains_null = True
            if type in result_set:
                continue
            result.append(type)
            result_set.add(type)
        if len(result) == 1:
            return result[0]
        elif all(item.const is not None for item in result):
            return Enum(items=tuple(result), contains_null=contains_null)
        else:
            return OneOf(items=tuple(result), contains_null=contains_null)

    def find_refs(self) -> list[Ref]:
        return [ref for ch in self.items for ref in ch.find_refs()]


@dataclass(frozen=True)
class Enum(OneOf):
    pass


@dataclass(frozen=True)
class AllOf(Type):
    items: tuple[Type, ...]

    def print(self, loader: Loader | None = None, _parens: bool = False) -> str:
        res = " & ".join(item.print(loader, True) for item in self.items) or "never"
        if _parens:
            return f"({res})"
        else:
            return res

    @staticmethod
    def create(types: _t.Iterable[Type]) -> Type:
        flat = []
        for type in types:
            if isinstance(type, AllOf):
                flat.extend(type.items)
            else:
                flat.append(type)
        result_set = set()
        result: list[Type] = []
        for type in flat:
            if type in result_set:
                continue
            result.append(type)
            result_set.add(type)
        if len(result) == 1:
            return result[0]
        else:
            return AllOf(items=tuple(result))

    def find_refs(self) -> list[Ref]:
        return [ref for ch in self.items for ref in ch.find_refs()]


class Loader:
    def __init__(
        self,
        registry: referencing.Registry[dict[str, _t.Any]],
    ):
        self._registry = registry
        self._base_uris: list[str] = [""]
        self._cache: dict[str, Type] = {}
        self._ref_count: dict[str, int] = {}

    @property
    def _base_uri(self) -> str:
        return self._base_uris[-1]

    @contextlib.contextmanager
    def _push_base_uri(self, uri: str):
        self._base_uris.append(uri)
        try:
            yield
        finally:
            self._base_uris.pop()

    def load(self, uri: str) -> Type:
        if uri.startswith("#"):
            uri, fragment = self._base_uri, uri[1:]
        else:
            uri, fragment = urldefrag(urljoin(self._base_uri, uri))

        retrieved = self._registry.get_or_retrieve(uri)
        new_uri = retrieved.value.id() or uri
        id = new_uri
        if fragment:
            id += "#" + fragment

        if id not in self._cache:
            self._ref_count[id] = 0
            if fragment.startswith("/"):
                resolver = self._registry.resolver(new_uri)
                resolved = retrieved.value.pointer(pointer=fragment, resolver=resolver)
                assert resolver is resolved.resolver, (
                    "accessing sub-resources via pointers from base resource "
                    "is not supported; use resource URI instead"
                )
                contents = resolved.contents
            elif fragment:
                retrieved = retrieved.registry.anchor(uri, fragment)
                resolver = self._registry.resolver(new_uri)
                resolved = retrieved.value.resolve(resolver=resolver)
                assert resolver is resolved.resolver, (
                    "accessing sub-resources via pointers from base resource "
                    "is not supported; use resource URI instead"
                )
                contents = resolved.contents
            else:
                contents = retrieved.value.contents
            with self._push_base_uri(new_uri):
                self._cache[id] = dataclasses.replace(
                    self._load_type(contents),
                    id=id,
                )

        return self._cache[id]

    def can_inline(self, id: str) -> bool:
        if not id:
            return True
        type = self.load(id)
        return self._ref_count[id] <= 1 and type.can_inline()

    def get_all_loaded(self) -> list[tuple[str, Type]]:
        return sorted(self._cache.items())

    def _load_type(self, root: dict[str, _t.Any]) -> Type:
        if not isinstance(root, dict):
            return Any()

        if "$ref" in root:
            id = self.load(root["$ref"]).id
            self._ref_count[id] = self._ref_count.get(id, 0) + 1
            return Ref(
                id or root["$ref"],
                title=root.get("title", None),
                description=root.get("description", None),
                deprecated=root.get("deprecated", False),
                default=deepfreeze(root.get("default", None)),
                const=deepfreeze(root.get("const", None)),
            )

        if "type" not in root:
            types = []
        elif not isinstance(root["type"], list):
            types = [root["type"]]
        else:
            types = root["type"]

        result_types: list[Type] = []

        for type in types:
            match type:
                case "array":
                    item = self._load_type(root.get("items", {}))
                    if "prefixItems" in root:
                        items = [
                            self._load_type(prefix) for prefix in root["prefixItems"]
                        ]

                        if isinstance(max_items := root.get("maxItems", None), int):
                            if max_items <= len(items):
                                # Tail is not allowed.
                                item = None

                        return Tuple(items=tuple(items), tail=item)
                    elif root.get("uniqueItems", False):
                        result_types.append(Set(item=item))
                    else:
                        result_types.append(Array(item=item))
                case "object":
                    result_types.append(self._load_object(root))
                case "null":
                    result_types.append(Null())
                case "boolean":
                    result_types.append(Boolean())
                case "integer":
                    result_types.append(Integer())
                case "number":
                    result_types.append(Number())
                case "string":
                    result_types.append(String())
                case True:
                    result_types.append(Any())
                case _:
                    pass

        if one_of := root.get("anyOf", []):
            result_types.extend(self._load_type(type) for type in one_of)
        if one_of := root.get("oneOf", []):
            result_types.extend(self._load_type(type) for type in one_of)
        if enum := root.get("enum", []):
            for item in enum:
                result_types.append(self.load_const(item))

        if result_types:
            result_types = [OneOf.create(result_types)]
        else:
            result_types = []

        if all_of := root.get("allOf", []):
            result_types.extend(self._load_type(type) for type in all_of)

        if result_types:
            result = AllOf.create(result_types)
        else:
            result = Any()

        return dataclasses.replace(
            result,
            title=root.get("title", None),
            description=root.get("description", None),
            deprecated=root.get("deprecated", False),
            default=deepfreeze(root.get("default", None)),
            const=deepfreeze(root.get("const", None)),
        )

    def _load_object(self, root: dict[str, _t.Any]) -> Type:
        # This code repeats implementation of `Resolver.lookup`,
        # see https://github.com/python-jsonschema/referencing/issues/257
        if not isinstance(root.get("default", None), dict):
            default = {}
        else:
            default = root["default"]
        if not isinstance(root.get("properties", None), dict):
            properties = {}
        else:
            properties = root["properties"]
        if not isinstance(root.get("required", None), list):
            required = set()
        else:
            required = set(root["required"])

        children = {}
        for name, data in properties.items():
            child = self._load_type(data)
            child = dataclasses.replace(
                child,
                default=default.get(name, child.default),
                required=name in required,
            )
            children[name] = child

        if "additionalProperties" in root:
            properties = self._load_type(root["additionalProperties"])
        else:
            properties = None

        return Object(
            title=root.get("title", None),
            description=root.get("description", None),
            deprecated=root.get("deprecated", False),
            named_children=frozendict(children),
            properties=properties,
        )

    def load_const(self, value: _t.Any):
        if value is None:
            return Null()
        elif isinstance(value, bool):
            return Boolean(const=value)
        elif isinstance(value, int):
            return Integer(const=value)
        elif isinstance(value, float):
            return Number(const=value)
        elif isinstance(value, str):
            return String(const=value)
        else:
            return Any(const=deepfreeze(value))


def split_uri(base: str, uri: str) -> tuple[str, str]:
    if "#" in uri:
        uri, anchor = uri.split("#", maxsplit=1)
    else:
        anchor = ""
    return uri or base, "#" + anchor


def load_jsons(app: sphinx.application.Sphinx):
    path = pathlib.Path(app.srcdir, app.config["emmylua_schema"]).expanduser().resolve()
    return (
        referencing.Registry()
        .with_resource(
            "EmmyRc",
            referencing.Resource.from_contents(json.loads(path.read_text())),
        )
        .crawl()
    )
