from typer import Typer 
from ._internal._init_command import internal_init_command

init_command = Typer()

@init_command.command("init")
def init() -> None:
	internal_init_command()