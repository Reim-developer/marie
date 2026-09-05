from typer import Exit, echo
from typing import NoReturn

println = echo


def die(code: int) -> NoReturn:
	raise Exit(code=code)
