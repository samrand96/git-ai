# Plugin hook management for git-ai
import os
import stat
from pathlib import Path
from core.config import settings

HOOK_TEMPLATES = {
	'pre-commit': """#!/bin/sh\n# git-ai pre-commit hook\npython git-ai.py --hook pre-commit\n""",
	'commit-msg': """#!/bin/sh\n# git-ai commit-msg hook\npython git-ai.py --hook commit-msg "$1"\n"""
}

def get_git_hooks_dir(repo_path: Path = Path.cwd()) -> Path:
    
	git_dir = repo_path / '.git'
	if not git_dir.exists():
		raise RuntimeError(f"No .git directory found in {repo_path}")
	return git_dir / 'hooks'

def install_hook(hook_name: str, repo_path: Path = Path.cwd()):
	hooks_dir = get_git_hooks_dir(repo_path)
	hooks_dir.mkdir(parents=True, exist_ok=True)
	hook_path = hooks_dir / hook_name
	script = HOOK_TEMPLATES.get(hook_name)
	if not script:
		raise ValueError(f"Unknown hook: {hook_name}")
	with open(hook_path, 'w') as f:
		f.write(script)
	os.chmod(hook_path, os.stat(hook_path).st_mode | stat.S_IEXEC)

def remove_hook(hook_name: str, repo_path: Path = Path.cwd()):
	hooks_dir = get_git_hooks_dir(repo_path)
	hook_path = hooks_dir / hook_name
	if hook_path.exists():
		hook_path.unlink()

def enable_hooks(repo_path: Path = Path.cwd()):
	"""Enable all supported hooks if enabled in settings."""
	if settings.get('HOOKS_ENABLED', 'false').lower() == 'true':
		for hook in HOOK_TEMPLATES:
			install_hook(hook, repo_path)

def disable_hooks(repo_path: Path = Path.cwd()):
	"""Remove all supported hooks."""
	for hook in HOOK_TEMPLATES:
		remove_hook(hook, repo_path)

def is_hook_enabled() -> bool:
	return settings.get('HOOKS_ENABLED', 'false').lower() == 'true'

# Example usage:

if __name__ == "__main__":
	import argparse
	parser = argparse.ArgumentParser(description="Manage git-ai hooks.")
	group = parser.add_mutually_exclusive_group(required=True)
	group.add_argument('--enable', action='store_true', help='Enable git hooks')
	group.add_argument('--disable', action='store_true', help='Disable git hooks')
	args = parser.parse_args()
	if args.enable:
		settings.set('HOOKS_ENABLED', 'true')
		enable_hooks()
		print("git-ai hooks enabled.")
	elif args.disable:
		settings.set('HOOKS_ENABLED', 'false')
		disable_hooks()
		print("git-ai hooks disabled.")
