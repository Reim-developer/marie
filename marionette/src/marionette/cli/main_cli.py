from typer import Typer


class MarionetteCli:
	def __init__(self) -> None:
		self.app = Typer(help=None, rich_help_panel=None, rich_markup_mode=None)
