from dataclasses import dataclass, asdict
from tomli_w import dumps
from typing import LiteralString


@dataclass
class Project:
	name: str
	version: str


@dataclass
class Output:
	to: LiteralString
	dir: str | None = ""


@dataclass
class MarionetteConfig:
	project: Project
	output_type: Output


def default_config() -> MarionetteConfig:
	config = MarionetteConfig(
		project=Project(name="My Project", version="0.0.1"),
		output_type=Output("html"),
	)

	return config


def to_toml(config: MarionetteConfig) -> str:
	return dumps(asdict(config))
