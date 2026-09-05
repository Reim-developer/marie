from typer import Typer
from marionette.utils import todo
from ._internal._generate_config import GenerateConfig
from ._internal._utils import die, println
from marionette.core.fs import MarionetteFs
from marionette.core.config import MarionetteConfig


generate_command = Typer()


@generate_command.command(name="generate")
def generate(new: bool = False) -> None:
	if not new:
		try:
			current = MarionetteFs().current_dir

			default_config = MarionetteConfig.default()
			config_path = GenerateConfig(current, default_config).create()
			println(
				f"Successfully generate default configuration, as: {config_path}"
			)

		except Exception as e:
			println(
				f"Cannot generate default configuration, error: {e}", err=True
			)
			die(2)

	else:
		todo("Non-default generate configuration will not implemented yet.")
