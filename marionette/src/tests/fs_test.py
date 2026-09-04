from marionette.core.fs import MarionetteFs
from pytest_mock import MockerFixture
from typing import Final
from pathlib import Path
from unittest.mock import mock_open


def test_write_file_when_not_exists(mocker: MockerFixture) -> None:
	fs = MarionetteFs()
	fake_path = Path("/home/scheherazade/projects")
	content = "Marionette"

	mocker.patch.object(Path, "exists", return_value=False)
	mocked_file = mocker.patch("builtins.open", mock_open())

	fs.write_file(fake_path, content=content)

	mocked_file.assert_called_once_with(fake_path, mode="a")
	mocked_file().write.assert_called_once_with(content)


def test_get_config_file(mocker: MockerFixture) -> None:
	EXPECT_CONFIG: Final[Path] = Path(
		"/home/scheherazade/projects/.marionette.toml"
	)

	mock_working_dir = mocker.patch.object(
		target=MarionetteFs,
		attribute="working_dir",
		autospec=True,
		return_value="/home/scheherazade/projects",
	)

	mocker.patch.object(Path, "exists", return_value=True)
	mocker.patch.object(MarionetteFs, "has_config", return_value=True)
	fs = MarionetteFs()
	result = fs.config_file()

	assert result
	assert result == EXPECT_CONFIG
	mock_working_dir.assert_called_once()


def test_has_project_file(mocker: MockerFixture) -> None:
	EXPECT_CONFIG: Final[Path] = Path(
		"/home/scheherazade/projects/.marionette.toml"
	)

	mock_working_dir = mocker.patch.object(
		target=MarionetteFs,
		attribute="working_dir",
		autospec=True,
		return_value=EXPECT_CONFIG,
	)
	mocker.patch.object(Path, "exists", return_value=True)

	mock_file = mocker.Mock(spec=Path)
	mock_file.is_file.return_value = True
	mock_file.name = MarionetteFs.config_name()

	mocker.patch.object(Path, "iterdir", return_value=[mock_file])

	fs = MarionetteFs()
	result = fs.has_config()

	assert result
	mock_working_dir.assert_called_once()


def test_current_working_dir(mocker: MockerFixture) -> None:
	EXPECT_DIR: Final[Path] = Path("/home/scheherazade/projects")
	mock_method = mocker.patch.object(
		target=MarionetteFs,
		attribute="working_dir",
		autospec=True,
		return_value=EXPECT_DIR,
	)

	fs = MarionetteFs()
	working_dir = fs.working_dir()

	assert working_dir == EXPECT_DIR

	mock_method.assert_called_once()
	mock_method.assert_called_once_with(fs)
