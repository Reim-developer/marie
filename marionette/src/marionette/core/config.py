from dataclasses import dataclass, asdict
from typing import LiteralString
from tomli_w import dumps


@dataclass
class Project:
	name: str
	version: str


@dataclass
class Html:
	images: int = 0


@dataclass
class Output:
	to: LiteralString
	type: Html


@dataclass
class MarionetteConfig:
	project: Project
	output_type: Output

	@staticmethod
	def new_config(project: Project, output_type: Output) -> MarionetteConfig:
		return MarionetteConfig(project=project, output_type=output_type)

	@staticmethod
	def default() -> MarionetteConfig:
		config = MarionetteConfig(
			project=Project(name="My Project", version="0.0.1"),
			output_type=Output(".marionette-generated", type=Html()),
		)

		return config

	@property
	def toml_string(self) -> str:
		return dumps(asdict(self))
