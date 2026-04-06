import os

import dkdc_home


def test_home_returns_dkdc_dir(monkeypatch):
    monkeypatch.delenv("DKDC_HOME", raising=False)
    h = dkdc_home.home()
    assert h.endswith(".dkdc")


def test_home_respects_env(monkeypatch, tmp_path):
    custom = str(tmp_path / "custom")
    monkeypatch.setenv("DKDC_HOME", custom)
    assert dkdc_home.home() == custom


def test_home_ignores_empty_env(monkeypatch):
    monkeypatch.setenv("DKDC_HOME", "")
    h = dkdc_home.home()
    assert h.endswith(".dkdc")


def test_ensure_creates_subdir(monkeypatch, tmp_path):
    monkeypatch.setenv("DKDC_HOME", str(tmp_path))
    result = dkdc_home.ensure("db")
    assert os.path.isdir(result)
    assert result.endswith("db")


def test_ensure_creates_nested(monkeypatch, tmp_path):
    monkeypatch.setenv("DKDC_HOME", str(tmp_path))
    result = dkdc_home.ensure("a/b/c")
    assert os.path.isdir(result)
    assert result.endswith("a/b/c")


def test_ensure_idempotent(monkeypatch, tmp_path):
    monkeypatch.setenv("DKDC_HOME", str(tmp_path))
    first = dkdc_home.ensure("db")
    second = dkdc_home.ensure("db")
    assert first == second
    assert os.path.isdir(first)
