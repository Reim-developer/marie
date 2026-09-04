from typing import LiteralString
from pathlib import Path

_MARIONETTE_CONFIG_NAME: LiteralString = ".marionette.toml"
_MARIONETTE_CONFIG: LiteralString = ".marionette.toml".casefold()


class MarionetteFs:
	def __init__(self, start_path: Path = Path.cwd()) -> None:
		self.start_path = start_path

	def __working_dir(self) -> Path | None:
		parents: list[Path] = list(self.start_path.parents)
		for path in [self.start_path] + parents:
			try:
				for child in path.iterdir():
					if child.name.casefold() == _MARIONETTE_CONFIG:
						return path

			except PermissionError | Exception:
				return None

		return None

	def working_dir(self) -> Path | None:
		return self.__working_dir()

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
		return _MARIONETTE_CONFIG_NAME
