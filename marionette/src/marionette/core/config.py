from dataclasses import dataclass
from typing import LiteralString


@dataclass
class Project:
	name: str
	version: str


@dataclass
class Html:
	images: int


@dataclass
class Output:
	to: LiteralString
	dir: str
	type: Html


@dataclass
class MarionetteConfig:
	project: Project
	output_type: Output


def new_config(project: Project, output_type: Output) -> MarionetteConfig:
	return MarionetteConfig(project=project, output_type=output_type)
