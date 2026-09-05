from pathlib import Path
from marionette.core.fs import MarionetteFs
from marionette.core.config import MarionetteConfig


class GenerateConfig:
	def __init__(self, path: Path, config: MarionetteConfig) -> None:
		self.path = path
		self.config = config

	def create(self) -> Path:
		fs = MarionetteFs()
		full_path = Path(f"{self.path}/{fs.config_name()}")

		match full_path.exists():
			case True:
				if fs.empty_config():
					fs.write_file(full_path, self.config.toml_string)

			case False:
				fs.write_file(full_path, self.config.toml_string)

		return full_path
