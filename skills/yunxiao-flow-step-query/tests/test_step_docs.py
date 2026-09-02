from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


SCRIPTS = Path(__file__).resolve().parents[1] / "scripts"
sys.path.insert(0, str(SCRIPTS))

import query_steps  # noqa: E402
import step_docs  # noqa: E402


JAVA_DOCS_URL = (
    "https://atomgit.com/flow-steps/system_steps/blob/master/"
    "docs/%E6%AD%A5%E9%AA%A4%20steps%20%E6%B8%85%E5%8D%95/build/"
    "Java%20%E6%9E%84%E5%BB%BA%20JavaBuild.md"
)


class StepDocumentTests(unittest.TestCase):
    def test_converts_only_the_known_atomgit_repository(self) -> None:
        raw = step_docs.atomgit_blob_to_raw(JAVA_DOCS_URL)
        self.assertEqual(
            raw,
            "https://raw.gitcode.com/flow-steps/system_steps/raw/master/"
            "docs/%E6%AD%A5%E9%AA%A4%20steps%20%E6%B8%85%E5%8D%95/build/"
            "Java%20%E6%9E%84%E5%BB%BA%20JavaBuild.md",
        )
        for untrusted in (
            "https://atomgit.com/other/repository/blob/master/docs/x.md",
            "https://example.test/flow-steps/system_steps/blob/master/docs/x.md",
            "https://atomgit.com/flow-steps/system_steps/blob/master/docs/../secret.md",
            "https://atomgit.com:bad/flow-steps/system_steps/blob/master/docs/x.md",
        ):
            self.assertIsNone(step_docs.atomgit_blob_to_raw(untrusted))

    def test_official_catalog_failure_uses_bundled_snapshot(self) -> None:
        with patch.object(query_steps, "_load", side_effect=OSError("offline")):
            steps, source = query_steps.load_catalog()
        self.assertTrue(source.startswith("bundled snapshot dated"))
        self.assertEqual(query_steps.find_exact_step(steps, "JavaBuild").category, "构建")

    def test_raw_failure_reads_existing_document_cache(self) -> None:
        raw_url = step_docs.atomgit_blob_to_raw(JAVA_DOCS_URL)
        assert raw_url is not None
        with tempfile.TemporaryDirectory() as directory:
            cache_home = Path(directory)
            path = step_docs.cache_path(raw_url, cache_home)
            path.parent.mkdir(parents=True)
            path.write_text("cached markdown", encoding="utf-8")
            content, source, _ = step_docs.retrieve_document(
                JAVA_DOCS_URL,
                cache_home=cache_home,
                read_raw=lambda _: (_ for _ in ()).throw(OSError("rate limited")),
                git_reader=lambda **_: self.fail("Git should not run when cache is available"),
            )
        self.assertEqual((content, source), ("cached markdown", "cache"))

    def test_raw_failure_uses_git_and_caches_the_result(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            cache_home = Path(directory)
            content, source, raw_url = step_docs.retrieve_document(
                JAVA_DOCS_URL,
                cache_home=cache_home,
                read_raw=lambda _: (_ for _ in ()).throw(OSError("rate limited")),
                git_reader=lambda *_, **__: "git markdown",
            )
            self.assertEqual((content, source), ("git markdown", "git"))
            self.assertEqual(step_docs.cache_path(raw_url, cache_home).read_text(encoding="utf-8"), "git markdown")

    def test_html_response_is_not_cached_or_returned(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            cache_home = Path(directory)
            content, source, raw_url = step_docs.retrieve_document(
                JAVA_DOCS_URL,
                cache_home=cache_home,
                read_raw=lambda _: "<!doctype html><html>rate limited</html>",
                git_reader=lambda *_, **__: "# Git Markdown",
            )
            self.assertEqual((content, source), ("# Git Markdown", "git"))
            self.assertEqual(
                step_docs.cache_path(raw_url, cache_home).read_text(encoding="utf-8"),
                "# Git Markdown",
            )

    def test_git_reader_uses_existing_local_clone_without_updating(self) -> None:
        calls: list[tuple[list[str], Path | None]] = []

        def fake_git(args: list[str], *, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
            calls.append((args, cwd))
            return subprocess.CompletedProcess(args, 0, stdout="local markdown")

        with tempfile.TemporaryDirectory() as directory:
            cache_home = Path(directory)
            repository = step_docs.cache_directory(cache_home) / "system_steps.git" / ".git"
            repository.mkdir(parents=True)
            content = step_docs.read_from_git(JAVA_DOCS_URL, cache_home=cache_home, run_git=fake_git)
        self.assertEqual(content, "local markdown")
        self.assertEqual(calls[0][0][0:2], ["git", "show"])
        self.assertFalse(any(call[0][1] in {"clone", "fetch"} for call in calls))

    def test_refresh_reads_fetched_git_head_and_skips_document_cache(self) -> None:
        calls: list[tuple[list[str], Path | None]] = []

        def fake_git(args: list[str], *, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
            calls.append((args, cwd))
            return subprocess.CompletedProcess(args, 0, stdout="# refreshed")

        with tempfile.TemporaryDirectory() as directory:
            cache_home = Path(directory)
            repository = step_docs.cache_directory(cache_home) / "system_steps.git" / ".git"
            repository.mkdir(parents=True)
            raw_url = step_docs.atomgit_blob_to_raw(JAVA_DOCS_URL)
            assert raw_url is not None
            stale = step_docs.cache_path(raw_url, cache_home)
            stale.parent.mkdir(parents=True)
            stale.write_text("# stale", encoding="utf-8")
            content, source, _ = step_docs.retrieve_document(
                JAVA_DOCS_URL,
                cache_home=cache_home,
                refresh=True,
                read_raw=lambda _: (_ for _ in ()).throw(OSError("offline")),
                git_reader=lambda docs_url, **kwargs: step_docs.read_from_git(
                    docs_url, run_git=fake_git, **kwargs
                ),
            )
        self.assertEqual((content, source), ("# refreshed", "git"))
        self.assertEqual(calls[0][0][0:2], ["git", "fetch"])
        self.assertTrue(calls[1][0][2].startswith("FETCH_HEAD:"))


if __name__ == "__main__":
    unittest.main()
