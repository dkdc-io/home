import os

import dkdc_home


def test_home_returns_dkdc_dir():
    prev = os.environ.pop("DKDC_HOME", None)
    try:
        h = dkdc_home.home()
        assert h.endswith(".dkdc")
    finally:
        if prev is not None:
            os.environ["DKDC_HOME"] = prev


def test_home_respects_env(tmp_path):
    custom = str(tmp_path / "custom")
    os.environ["DKDC_HOME"] = custom
    try:
        assert dkdc_home.home() == custom
    finally:
        del os.environ["DKDC_HOME"]


def test_home_ignores_empty_env():
    os.environ["DKDC_HOME"] = ""
    try:
        h = dkdc_home.home()
        assert h.endswith(".dkdc")
    finally:
        del os.environ["DKDC_HOME"]


def test_ensure_creates_subdir(tmp_path):
    os.environ["DKDC_HOME"] = str(tmp_path)
    try:
        result = dkdc_home.ensure("db")
        assert os.path.isdir(result)
        assert result.endswith("db")
    finally:
        del os.environ["DKDC_HOME"]


def test_ensure_creates_nested(tmp_path):
    os.environ["DKDC_HOME"] = str(tmp_path)
    try:
        result = dkdc_home.ensure("a/b/c")
        assert os.path.isdir(result)
        assert result.endswith("a/b/c")
    finally:
        del os.environ["DKDC_HOME"]


def test_ensure_idempotent(tmp_path):
    os.environ["DKDC_HOME"] = str(tmp_path)
    try:
        first = dkdc_home.ensure("db")
        second = dkdc_home.ensure("db")
        assert first == second
    finally:
        del os.environ["DKDC_HOME"]
