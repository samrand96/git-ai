# CLI command to generate and make a commit using the configured provider
from core.config import settings
from providers.factory import get_provider
from utils import get_branch, get_diff, stage_all, commit, push, clean_commit_message, Colors

def get_ticket_prefix(branch):
	import re
	m = re.match(r'^([A-Za-z]+-\d+)', branch)
	return m.group(1) if m else ''

def main():
	import argparse
	parser = argparse.ArgumentParser(description="Generate and commit using AI")
	parser.add_argument('--format', choices=['detailed', 'one-line'], help='Commit message format')
	parser.add_argument('--provider', help='Provider to use (overrides config)')
	parser.add_argument('--model', help='Model to use (overrides config)')
	parser.add_argument('--push', action='store_true', help='Push after commit')
	args = parser.parse_args()

	if args.format:
		settings.set('COMMIT_FORMAT', args.format)
	provider_name = args.provider or settings.get('PROVIDER', 'openai')
	provider_kwargs = {}
	if args.model:
		provider_kwargs['model'] = args.model
	provider = get_provider(provider_name, **provider_kwargs)

	stage_all()
	diff = get_diff(staged=True)
	if not diff.strip():
		print(Colors.info("ℹ No staged changes to commit."))
		return
	branch = get_branch()
	prefix = get_ticket_prefix(branch)
	short = settings.get('COMMIT_FORMAT', 'detailed') == 'one-line'
	sys_msg = (
		"You are an expert Git commit assistant. When responding, return only the commit message text itself—no extra explanation, quotes, or formatting. "
		"First, inspect the current Git branch name. If it begins with a ticket code matching the pattern LETTERS-DIGITS (for example ABC-123 or EL-2024), capture that exact code and place it at the very start of your message, followed by a colon and a space. If no ticket code is present, do not include any prefix. "
		"Next, identify the primary change or task implied by the branch name and present it as the first action in your commit message. Then, describe any secondary updates, fixes, or refactoring included in this commit. "
		"Use a natural, professional tone that reads like a teammate clearly explaining the work you’ve done. Use bullet points to separate multiple actions if they exist."
	)
	user_msg = f"Branch: {branch}\nWrite a {'one-line' if short else 'detailed, human-friendly'} commit message for these changes:\n\n{diff}"
	
	print(Colors.header("🤖 Generating commit message with AI..."))
	commit_msg = provider.generate(
		prompt=user_msg,
		messages=[
			{"role": "system", "content": sys_msg},
			{"role": "user", "content": user_msg}
		]
	)
	
	# Clean up the AI-generated commit message
	commit_msg = clean_commit_message(commit_msg)
	
	if prefix and not commit_msg.startswith(prefix):
		commit_msg = f"{prefix}: {commit_msg}"
	
	print(Colors.success("\n✅ Generated commit message:"))
	print(Colors.highlight(commit_msg))
	print()
	
	edit = input(Colors.dim("Edit before commit? [y/N]: ")).strip().lower() == 'y'
	if edit:
		print(Colors.dim("Enter new commit message. End with an empty line:"))
		lines = []
		while True:
			line = input()
			if not line.strip():
				break
			lines.append(line)
		commit_msg = '\n'.join(lines) if lines else commit_msg
	
	commit(commit_msg)
	print(Colors.success("✅ Committed successfully."))
	
	if args.push:
		push()
		print(Colors.success("🚀 Pushed to remote."))
	else:
		print(Colors.dim("💡 Use --push flag to automatically push after commit."))

if __name__ == "__main__":
	main()
