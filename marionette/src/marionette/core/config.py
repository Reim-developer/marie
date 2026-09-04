from dataclasses import dataclass
from typing import LiteralString


@dataclass
class Project:
	name: str
	version: str


@dataclass
class Output:
	to: LiteralString
	dir: str
	type: str


@dataclass
class MarionetteConfig:
	project: Project
	output_type: Output
