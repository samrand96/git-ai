# Versatile Git utility functions
import subprocess
from pathlib import Path

def run_git_command(args, repo_path=None, capture_output=True, check=True, text=True):
	"""Run a git command and return output or raise error."""
	cmd = ['git'] + args
	kwargs = {
		'cwd': str(repo_path) if repo_path else None,
		'capture_output': capture_output,
		'check': check,
		'text': text,
		'encoding': 'utf-8',
		'errors': 'replace'  # Replace invalid characters instead of failing
	}
	result = subprocess.run(cmd, **{k: v for k, v in kwargs.items() if v is not None})
	if capture_output:
		return (result.stdout or "").strip()
	return result

def get_branch(repo_path=None):
	"""Get current branch name."""
	return run_git_command(['rev-parse', '--abbrev-ref', 'HEAD'], repo_path)

def get_status(repo_path=None):
	"""Get git status (short)."""
	return run_git_command(['status', '--short'], repo_path)

def get_diff(staged=False, repo_path=None, commit=None):
	"""Get git diff (staged, unstaged, or between commits)."""
	if commit:
		args = ['diff', commit]
	elif staged:
		args = ['diff', '--staged']
	else:
		args = ['diff']
	return run_git_command(args, repo_path)

def stage_all(repo_path=None):
	"""Stage all changes."""
	run_git_command(['add', '.'], repo_path, capture_output=False)

def commit(msg, repo_path=None):
	"""Commit staged changes with a message."""
	run_git_command(['commit', '-m', msg], repo_path, capture_output=False)

def push(repo_path=None):
	"""Push to remote."""
	run_git_command(['push'], repo_path, capture_output=False)

# Future: Add helpers for branch creation, tag, log, blame, etc.
