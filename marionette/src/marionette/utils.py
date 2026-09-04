from typing import NoReturn


def todo(reason: str | None = "not implemented yet") -> NoReturn:
	"""
	An abstract layer for `NotImplementedError`, which will always return an
	exception.
	"""
	raise NotImplementedError(reason)
