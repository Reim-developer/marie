from typer import Typer
from marionette.cli.commands.init import init_command

class MarionetteCli:
	def __init__(self) -> None:
		self.app = Typer(
			pretty_exceptions_enable=False,
			rich_markup_mode=None
		)

		self.app.add_typer(init_command)

	def __call__(self) -> None:
		self.app()
