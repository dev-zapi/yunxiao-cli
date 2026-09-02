#!/usr/bin/env python3
"""Retrieve trusted Flow Step Markdown without loading an AtomGit blob page."""

from __future__ import annotations

import hashlib
import os
import subprocess
import tempfile
from pathlib import Path
from typing import Callable
from urllib.parse import quote, unquote, urlsplit
from urllib.request import Request, urlopen


ATOMGIT_HOST = "atomgit.com"
REPOSITORY_OWNER = "flow-steps"
REPOSITORY_NAME = "system_steps"
GIT_REPOSITORY_URL = "https://gitcode.com/flow-steps/system_steps.git"
RAW_BASE_URL = "https://raw.gitcode.com/flow-steps/system_steps/raw"
USER_AGENT = "yunxiao-flow-step-query/1.1"
GIT_TIMEOUT_SECONDS = 30


class StepDocumentError(RuntimeError):
    """Raised when a trusted Step document cannot be read."""


def atomgit_blob_to_raw(docs_url: str) -> str | None:
    """Map a Flow Steps AtomGit blob URL to its trusted GitCode raw URL.

    The mapping intentionally accepts only the known public repository and a
    Markdown path below ``docs``.  It returns ``None`` for every other URL.
    """

    try:
        parsed = urlsplit(docs_url)
        port = parsed.port
    except ValueError:
        return None
    if (
        parsed.scheme != "https"
        or parsed.hostname != ATOMGIT_HOST
        or parsed.username is not None
        or parsed.password is not None
        or port is not None
        or parsed.query
        or parsed.fragment
    ):
        return None
    path = unquote(parsed.path)
    parts = path.split("/")
    if len(parts) < 7 or parts[:4] != ["", REPOSITORY_OWNER, REPOSITORY_NAME, "blob"]:
        return None
    ref = parts[4]
    document_parts = parts[5:]
    if (
        not ref
        or any(char not in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-" for char in ref)
        or document_parts[0] != "docs"
        or any(not part or part in {".", ".."} or "\x00" in part for part in document_parts)
        or not document_parts[-1].endswith(".md")
    ):
        return None
    document_path = "/".join(document_parts)
    return f"{RAW_BASE_URL}/{quote(ref, safe='')}/{quote(document_path, safe='/')}"


def _source_parts(docs_url: str) -> tuple[str, str]:
    raw_url = atomgit_blob_to_raw(docs_url)
    if raw_url is None:
        raise StepDocumentError(
            "detail URL is not a Flow Steps AtomGit blob URL for flow-steps/system_steps"
        )
    parts = unquote(urlsplit(docs_url).path).split("/")
    return parts[4], "/".join(parts[5:])


def cache_directory(cache_home: Path | None = None) -> Path:
    """Return the narrow XDG cache directory used by this skill."""

    if cache_home is None:
        configured = os.environ.get("XDG_CACHE_HOME")
        cache_home = Path(configured) if configured else Path.home() / ".cache"
    return cache_home / "yunxiao-flow-step-query"


def cache_path(raw_url: str, cache_home: Path | None = None) -> Path:
    """Return a collision-resistant cache location for one raw document URL."""

    digest = hashlib.sha256(raw_url.encode("utf-8")).hexdigest()
    return cache_directory(cache_home) / "step-docs" / f"{digest}.md"


def _write_cache(path: Path, content: str) -> None:
    path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        "w", encoding="utf-8", dir=path.parent, prefix=".tmp-", delete=False
    ) as temporary:
        temporary.write(content)
        temporary_name = temporary.name
    os.replace(temporary_name, path)


def _cache_best_effort(path: Path, content: str) -> None:
    """Persist a fetched document when possible without affecting retrieval."""

    try:
        _write_cache(path, content)
    except OSError:
        pass


def _read_raw(raw_url: str) -> str:
    request = Request(raw_url, headers={"User-Agent": USER_AGENT})
    with urlopen(request, timeout=20) as response:  # noqa: S310 - URL was strictly derived above
        return response.read().decode("utf-8")


def _validate_markdown(content: str) -> str:
    """Reject empty responses and common HTML/WAF wrapper pages."""

    prefix = content.lstrip().casefold()
    if not content.strip():
        raise StepDocumentError("detail response was empty")
    if prefix.startswith(("<!doctype html", "<html", "<head", "<body")):
        raise StepDocumentError("detail response was HTML, not Markdown")
    return content


def _run_git(args: list[str], *, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    try:
        return subprocess.run(
            args,
            cwd=cwd,
            check=True,
            capture_output=True,
            text=True,
            timeout=GIT_TIMEOUT_SECONDS,
        )
    except subprocess.TimeoutExpired as error:
        command = " ".join(str(part) for part in args)
        raise StepDocumentError(
            f"Git command timed out after {GIT_TIMEOUT_SECONDS}s: {command}"
        ) from error


def read_from_git(
    docs_url: str,
    *,
    cache_home: Path | None = None,
    refresh: bool = False,
    run_git: Callable[..., subprocess.CompletedProcess[str]] = _run_git,
) -> str:
    """Read a document from a cached shallow clone of the known public repo."""

    source_ref, document_path = _source_parts(docs_url)
    read_ref = source_ref
    repository = cache_directory(cache_home) / "system_steps.git"
    try:
        if not (repository / ".git").is_dir():
            repository.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            run_git(["git", "clone", "--depth", "1", GIT_REPOSITORY_URL, str(repository)])
        elif refresh:
            run_git(["git", "fetch", "--depth", "1", "origin", source_ref], cwd=repository)
            read_ref = "FETCH_HEAD"
        try:
            result = run_git(["git", "show", f"{read_ref}:{document_path}"], cwd=repository)
        except subprocess.CalledProcessError:
            # A SHA may not be in the default shallow clone. Fetch only that ref.
            run_git(["git", "fetch", "--depth", "1", "origin", source_ref], cwd=repository)
            result = run_git(["git", "show", f"FETCH_HEAD:{document_path}"], cwd=repository)
    except (OSError, subprocess.CalledProcessError) as error:
        raise StepDocumentError(f"Git fallback failed: {error}") from error
    return _validate_markdown(result.stdout)


def retrieve_document(
    docs_url: str,
    *,
    cache_home: Path | None = None,
    refresh: bool = False,
    read_raw: Callable[[str], str] = _read_raw,
    git_reader: Callable[..., str] = read_from_git,
) -> tuple[str, str, str]:
    """Return Markdown, its source (raw/cache/git), and its direct raw URL.

    A normal retrieval asks raw.gitcode.com first so ``master`` can advance.
    Network failure then uses an existing local document cache before a cached
    shallow Git clone. ``refresh`` skips the document cache after raw failure
    and refreshes the Git fallback before reading it.
    """

    raw_url = atomgit_blob_to_raw(docs_url)
    if raw_url is None:
        raise StepDocumentError(
            "detail URL is not a Flow Steps AtomGit blob URL for flow-steps/system_steps"
        )
    cached = cache_path(raw_url, cache_home)
    raw_error: Exception | None = None
    try:
        content = _validate_markdown(read_raw(raw_url))
        _cache_best_effort(cached, content)
        return content, "raw.gitcode.com", raw_url
    except (OSError, UnicodeError, ValueError, StepDocumentError) as error:
        raw_error = error
    if not refresh and cached.is_file():
        try:
            return _validate_markdown(cached.read_text(encoding="utf-8")), "cache", raw_url
        except (OSError, UnicodeError, StepDocumentError) as error:
            raw_error = error
    try:
        content = _validate_markdown(git_reader(docs_url, cache_home=cache_home, refresh=refresh))
        _cache_best_effort(cached, content)
        return content, "git", raw_url
    except StepDocumentError as error:
        raise StepDocumentError(
            f"raw.gitcode.com failed ({raw_error}); cached document unavailable; {error}"
        ) from error
