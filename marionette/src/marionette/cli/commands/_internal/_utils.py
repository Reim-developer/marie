from typer import Exit
from typing import NoReturn


def die(code: int) -> NoReturn:
	raise Exit(code=code)
