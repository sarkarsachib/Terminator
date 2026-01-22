from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class Page:
    url: str
    body: bytes


def scrape(url: str) -> Page:
    return Page(url=url, body=b"")
