# Utility functions for input/output operations


import sys
from pathlib import Path

def read_file(path, encoding='utf-8', binary=False):
	"""Read the contents of a file. Supports text or binary."""
	mode = 'rb' if binary else 'r'
	with open(path, mode, encoding=None if binary else encoding) as f:
		return f.read()

def write_file(path, content, encoding='utf-8', binary=False, append=False):
	"""Write content to a file. Supports text or binary, and append mode."""
	mode = ('ab' if binary else 'a') if append else ('wb' if binary else 'w')
	with open(path, mode, encoding=None if binary else encoding) as f:
		f.write(content)

def file_exists(path):
	"""Check if a file exists."""
	return Path(path).exists()

def print_info(msg, color=True):
	_print_with_prefix(msg, prefix='[INFO]', color_code='34', color=color)

def print_error(msg, color=True):
	_print_with_prefix(msg, prefix='[ERROR]', color_code='31', color=color, file=sys.stderr)

def print_success(msg, color=True):
	_print_with_prefix(msg, prefix='[OK]', color_code='32', color=color)

def _print_with_prefix(msg, prefix='', color_code='37', color=True, file=sys.stdout):
	if color and sys.stdout.isatty():
		print(f"\033[{color_code}m{prefix} {msg}\033[0m", file=file)
	else:
		print(f"{prefix} {msg}", file=file)

# Future: Add more helpers for user prompts, colored output, progress bars, etc.
