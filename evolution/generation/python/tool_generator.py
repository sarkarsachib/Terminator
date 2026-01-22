from __future__ import annotations


def generate(spec: dict) -> str:
    return spec.get("name", "generated_tool")
