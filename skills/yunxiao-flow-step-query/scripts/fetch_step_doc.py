#!/usr/bin/env python3
"""Fetch the parameter documentation for one Yunxiao Flow Step."""

from __future__ import annotations

import argparse
import sys

from query_steps import find_exact_step, load_catalog
from step_docs import StepDocumentError, retrieve_document


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("identifier", help="Exact Step identifier, such as JavaBuild")
    parser.add_argument(
        "--refresh",
        action="store_true",
        help="Refresh the cached Git fallback if raw Markdown is unavailable",
    )
    parser.add_argument("--catalog-source", help="Official catalog URL, local file, or -")
    parser.add_argument("--offline", action="store_true", help="Use the bundled catalog snapshot")
    args = parser.parse_args(argv)

    try:
        steps, catalog_source = load_catalog(args.catalog_source, offline=args.offline)
    except (OSError, ValueError, UnicodeError) as error:
        print(f"error: unable to read Steps catalog: {error}", file=sys.stderr)
        return 1
    step = find_exact_step(steps, args.identifier)
    if step is None:
        print(f"error: unknown Step identifier: {args.identifier}", file=sys.stderr)
        return 2
    if not step.docs_url:
        print("error: snapshot has no detail URL for this Step; retry when the official catalog is available", file=sys.stderr)
        return 1
    try:
        content, document_source, raw_url = retrieve_document(step.docs_url, refresh=args.refresh)
    except StepDocumentError as error:
        print(f"error: unable to read Step detail: {error}", file=sys.stderr)
        return 1
    print(f"catalog source: {catalog_source}; detail source: {document_source}; raw URL: {raw_url}", file=sys.stderr)
    print(content, end="" if content.endswith("\n") else "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
