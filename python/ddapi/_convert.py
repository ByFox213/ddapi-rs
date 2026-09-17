"""Convert plain dicts returned by the Rust extension into typed dataclasses."""

import types
from dataclasses import fields, is_dataclass
from datetime import date, datetime, timezone
from typing import Any, Union, get_args, get_origin, get_type_hints

_HINTS: dict = {}


def _hints(cls: type) -> dict:
    hints = _HINTS.get(cls)
    if hints is None:
        hints = _HINTS[cls] = get_type_hints(cls)
    return hints


def _parse_datetime(value: Any) -> datetime:
    if isinstance(value, (int, float)):
        return datetime.fromtimestamp(value, timezone.utc)
    text = value.replace("Z", "+00:00")
    return datetime.fromisoformat(text)


def _from_dict(cls: type, value: Any) -> Any:
    if value is None:
        return None
    if cls is datetime:
        return _parse_datetime(value)
    if cls is date:
        return date.fromisoformat(value)
    origin = get_origin(cls)
    if origin is list:
        (item,) = get_args(cls)
        return [_from_dict(item, v) for v in value]
    if origin is dict:
        _key, val = get_args(cls)
        return {k: _from_dict(val, v) for k, v in value.items()}
    if origin is Union or origin is types.UnionType:
        (non_none,) = [a for a in get_args(cls) if a is not type(None)]
        return _from_dict(non_none, value)
    if is_dataclass(cls):
        hints = _hints(cls)
        return cls(
            **{
                f.name: _from_dict(hints[f.name], value[f.metadata.get("json", f.name)])
                for f in fields(cls)
            }
        )
    return value