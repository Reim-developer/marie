from typing import LiteralString
from pathlib import Path

_MARIONETTE_CONFIG: LiteralString = ".marionette.toml"


class MarionetteFs:
	def __init__(self, start_path: Path = Path.cwd()) -> None:
		self.start_path = start_path

	@property
	def current_dir(self) -> Path:
		return Path.cwd()

	def __working_dir(self) -> Path | None:
		parents: list[Path] = list(self.start_path.parents)

		for path in [self.start_path] + parents:
			for child in path.iterdir():
				if child.name.casefold() == _MARIONETTE_CONFIG:
					return path

		return None

	def working_dir(self) -> Path | None:
		"""
		`self.__working_dir()` uses `iterdir()`, and
		it might return an exception, like `FileNotFoundError`,
		`NotADirectoryError`...

		Therefore, the caller will need to handle exceptions when
		using it.

		The return value `None` does not respresent that an exception
		will occur. Sometimes it simply means that the Marionette configuration
		does not exists.

		When only concerned with whether the configuration file exists, just check
		the return value instead.
		"""
		return self.__working_dir()

	def write_file(self, path: Path, content: str) -> None:
		"""
		Invariant:
		Caller must handle the exception when open or/and write file has
		failed.

		And, it will auto create file if it does not exists, otherwise if it
		already exists, it adds new `content` to the file.

		To avoid unwanted behavior such as overwriting files, caller need to
		perform self-check, like use `exists` before using it.
		"""

		with open(path, mode="a") as f:
			f.write(content)

	def create_file(self, path: Path) -> None:
		"""
		Invariant: Caller must handle the exception
		of `path.touch`
		"""
		if not path.exists():
			path.touch(exist_ok=True)

	def config_file(self) -> Path | None:
		"""
		Invariant:
		Will always assume that the return value is a `Path` that
		exists because of the check with `self.has_config` above.

		To ensure they exists until used, the caller should double-check
		with `exists()`
		"""

		if not self.has_config():
			return None

		working_dir = self.working_dir()
		config_file = Path(f"{working_dir}/{_MARIONETTE_CONFIG}")

		return config_file

	def empty_config(self) -> bool:
		if not self.has_config():
			return False

		config_file = self.config_file()
		# It is assumed that the config
		# already exists here because
		# `has_config` has checked.
		assert config_file, "'self.config_file()' should not None."

		return config_file.stat().st_size == 0

	def has_config(self) -> bool:
		working_dir = self.working_dir()

		if not working_dir or not working_dir.exists():
			return False

		for path in working_dir.iterdir():
			if path.is_file():
				if path.name.casefold() == _MARIONETTE_CONFIG:
					return True

			else:
				continue

		return False

	@staticmethod
	def config_name() -> LiteralString:
		return _MARIONETTE_CONFIG
