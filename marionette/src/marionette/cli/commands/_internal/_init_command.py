from marionette.core.fs import MarionetteFs
from ._utils import die, println


def internal_init_command() -> None:
	fs = MarionetteFs()
	config_file = fs.config_file()

	if config_file:
		println(
			f"Found Marionette configuration file in: {config_file}. No need to initialize."
		)
		die(1)
