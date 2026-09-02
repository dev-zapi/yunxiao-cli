#!/usr/bin/env python3
"""Query the official Yunxiao Flow YAML built-in Steps catalog."""

from __future__ import annotations

import argparse
import html
import json
import re
import sys
from dataclasses import asdict, dataclass
from html.parser import HTMLParser
from pathlib import Path
from urllib.request import Request, urlopen

from step_docs import atomgit_blob_to_raw


CATALOG_URL = "https://help.aliyun.com/zh/yunxiao/user-guide/step-steps-list.md"
SNAPSHOT_PATH = Path(__file__).resolve().parents[1] / "references" / "steps-catalog.json"
SNAPSHOT_DATE = "2026-09-02"


@dataclass(frozen=True)
class Step:
    """One Step entry from the official catalog."""

    identifier: str
    category: str
    display_name: str
    description: str
    docs_url: str
    raw_docs_url: str | None = None


class _TableParser(HTMLParser):
    """Extract table rows while preserving anchor URLs."""

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.rows: list[list[tuple[str, str]]] = []
        self._row: list[dict[str, object]] | None = None
        self._cell: dict[str, object] | None = None
        self._anchor_href = ""

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag == "tr":
            self._row = []
        elif tag == "td" and self._row is not None:
            self._cell = {"text": [], "href": ""}
            self._row.append(self._cell)
        elif tag == "a" and self._cell is not None:
            self._anchor_href = dict(attrs).get("href") or ""

    def handle_endtag(self, tag: str) -> None:
        if tag == "a":
            if self._cell is not None and not self._cell["href"]:
                self._cell["href"] = self._anchor_href
            self._anchor_href = ""
        elif tag == "td":
            self._cell = None
        elif tag == "tr" and self._row is not None:
            cells: list[tuple[str, str]] = []
            for cell in self._row:
                text = " ".join(str(part) for part in cell["text"])
                text = re.sub(r"\s+", " ", html.unescape(text)).strip()
                cells.append((text, str(cell["href"])))
            self.rows.append(cells)
            self._row = None

    def handle_data(self, data: str) -> None:
        if self._cell is not None:
            self._cell["text"].append(data)


def _load(source: str) -> str:
    if source == "-":
        return sys.stdin.read()
    if source.startswith(("http://", "https://")):
        request = Request(source, headers={"User-Agent": "yunxiao-flow-step-query/1.0"})
        with urlopen(request, timeout=20) as response:  # noqa: S310 - fixed/documented URL input
            return response.read().decode("utf-8")
    return Path(source).read_text(encoding="utf-8")


def _identifier(display_name: str) -> str | None:
    match = re.search(r"([A-Za-z][A-Za-z0-9]*)\s*$", display_name)
    return match.group(1) if match else None


def parse_catalog(markup: str) -> list[Step]:
    """Parse the official HTML table embedded in the Markdown response."""

    parser = _TableParser()
    parser.feed(markup)
    steps: list[Step] = []
    category = ""
    for cells in parser.rows:
        if not cells or cells[0][0] == "分类":
            continue
        if len(cells) == 3:
            category_cell, step_cell, description = cells
            if category_cell[0]:
                category = category_cell[0]
        elif len(cells) == 2:
            step_cell, description = cells
        else:
            continue
        identifier = _identifier(step_cell[0])
        if not identifier or not step_cell[1]:
            continue
        steps.append(
            Step(
                identifier=identifier,
                category=category,
                display_name=step_cell[0],
                description=description[0],
                docs_url=step_cell[1],
                raw_docs_url=atomgit_blob_to_raw(step_cell[1]),
            )
        )
    if not steps:
        raise ValueError("No Step entries found in the catalog")
    return steps


def _matches(steps: list[Step], query: str | None, category: str | None, exact: bool) -> list[Step]:
    query_fold = query.casefold() if query else None
    category_fold = category.casefold() if category else None
    result = []
    for step in steps:
        if category_fold and category_fold not in step.category.casefold():
            continue
        if query_fold:
            fields = (step.identifier, step.display_name, step.description)
            if exact:
                if step.identifier.casefold() != query_fold:
                    continue
            elif not any(query_fold in field.casefold() for field in fields):
                continue
        result.append(step)
    return result


def find_exact_step(steps: list[Step], identifier: str) -> Step | None:
    """Find a Step by its case-insensitive YAML identifier."""

    wanted = identifier.casefold()
    return next((step for step in steps if step.identifier.casefold() == wanted), None)


def _load_snapshot() -> list[Step]:
    steps = [Step(**entry) for entry in json.loads(SNAPSHOT_PATH.read_text(encoding="utf-8"))]
    if not steps:
        raise ValueError("bundled Steps snapshot is empty")
    return steps


def load_catalog(source: str | None = None, *, offline: bool = False) -> tuple[list[Step], str]:
    """Load the official catalog, falling back to the dated bundled snapshot."""

    if offline:
        return _load_snapshot(), f"bundled snapshot dated {SNAPSHOT_DATE}"
    selected = source or CATALOG_URL
    try:
        return parse_catalog(_load(selected)), selected
    except (OSError, ValueError, UnicodeError) as error:
        if source is not None:
            raise error
        return _load_snapshot(), f"bundled snapshot dated {SNAPSHOT_DATE}"


def _print_text(steps: list[Step]) -> None:
    if not steps:
        print("No matching Steps found.")
        return
    for index, step in enumerate(steps):
        if index:
            print()
        print(f"{step.identifier} [{step.category}]")
        print(f"  name: {step.display_name}")
        print(f"  description: {step.description}")
        print(f"  docs: {step.docs_url}")
        if step.raw_docs_url:
            print(f"  raw docs: {step.raw_docs_url}")
        print(f"  yaml: step: {step.identifier}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--query", "-q", help="Identifier, display name, or description keyword")
    parser.add_argument("--category", help="Category filter, such as 构建 or 工具")
    parser.add_argument("--exact", action="store_true", help="Match the Step identifier exactly")
    parser.add_argument("--all", action="store_true", help="Return every Step")
    parser.add_argument("--list-categories", action="store_true", help="List category names")
    parser.add_argument("--json", action="store_true", help="Emit JSON")
    parser.add_argument("--offline", action="store_true", help="Use the bundled dated catalog snapshot")
    parser.add_argument(
        "--source",
        "--input",
        dest="source",
        default=CATALOG_URL,
        help="Official URL, local file, or - for stdin",
    )
    args = parser.parse_args(argv)

    if not args.all and not args.query and not args.category and not args.list_categories:
        parser.error("provide --query, --category, --all, or --list-categories")
    try:
        steps, catalog_source = load_catalog(
            None if args.source == CATALOG_URL else args.source,
            offline=args.offline,
        )
    except (OSError, ValueError, UnicodeError) as error:
        print(f"error: unable to read Steps catalog: {error}", file=sys.stderr)
        return 1
    except Exception as error:  # network errors vary across Python versions
        print(f"error: unable to read Steps catalog: {error}", file=sys.stderr)
        return 1

    if catalog_source.startswith("bundled snapshot"):
        print(f"warning: catalog source is {catalog_source}", file=sys.stderr)

    if args.list_categories:
        categories = sorted({step.category for step in steps})
        if args.json:
            print(json.dumps(categories, ensure_ascii=False, indent=2))
        else:
            print("\n".join(categories))
        return 0

    matches = _matches(steps, args.query, args.category, args.exact)
    if args.json:
        print(json.dumps([asdict(step) for step in matches], ensure_ascii=False, indent=2))
    else:
        _print_text(matches)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
