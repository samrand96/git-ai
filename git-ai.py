import os
import subprocess
import openai
import argparse
from dotenv import load_dotenv

# Load .env file if it exists
load_dotenv()

# Get the OpenAI API key from environment or .env
openai.api_key = os.getenv("OPENAI_API_KEY")
if not openai.api_key:
    print("Error: OpenAI API key not found. Set OPENAI_API_KEY in your environment or a .env file.")
    exit(1)

def get_git_diff():
    """
    Fetch the current git diff summary.
    """
    try:
        diff = subprocess.check_output(["git", "diff", "--staged"], text=True)
        if not diff.strip():
            print("No staged changes to generate a commit message for.")
            exit(0)
        return diff
    except subprocess.CalledProcessError as e:
        print("Error fetching git diff:", e)
        exit(1)

def generate_commit_message(diff):
    """
    Use OpenAI API to generate a commit message based on the diff summary.
    """
    prompt = f"""
    Generate a concise and professional Git commit message for the following changes:
    
    {diff}
    """
    try:
        response = openai.ChatCompletion.create(
            model="gpt-4",
            messages=[
                {"role": "user", "content": prompt}
            ],
        )
        commit_message = response['choices'][0]['message']['content'].strip()
        return commit_message
    except Exception as e:
        print("Error generating commit message:", e)
        exit(1)

def apply_commit(commit_message):
    """
    Apply the generated commit message using Git.
    """
    try:
        subprocess.run(["git", "commit", "-m", commit_message], check=True)
        print("Commit successfully applied!")
    except subprocess.CalledProcessError as e:
        print("Error applying commit:", e)
        exit(1)

def main():
    """
    Main function to run the CLI tool.
    """
    parser = argparse.ArgumentParser(description="Auto-generate Git commit messages using OpenAI GPT.")
    parser.add_argument("--commit", action="store_true", help="Automatically commit with the generated message.")
    args = parser.parse_args()

    # Get the git diff
    diff = get_git_diff()

    # Generate a commit message
    print("Generating commit message...")
    commit_message = generate_commit_message(diff)
    print("\nSuggested commit message:\n")
    print(commit_message)

    # Apply the commit if requested
    if args.commit:
        confirm = input("\nDo you want to use this commit message? [y/N]: ").strip().lower()
        if confirm == 'y':
            apply_commit(commit_message)
        else:
            print("Commit aborted.")
    else:
        print("\nUse the --commit flag to automatically commit with this message.")

if __name__ == "__main__":
    main()