import os
import subprocess
from openai import OpenAI
from dotenv import load_dotenv
import sys

# Load .env file if it exists
load_dotenv()

# Initialize OpenAI client
client = OpenAI(api_key=os.getenv("OPENAI_API_KEY"))

# Check if the OpenAI API key is set
if not client.api_key:
    print("Error: OpenAI API key not found. Set OPENAI_API_KEY in your environment or a .env file.")
    sys.exit(1)

def stage_changes():
    """
    Stage all changes in the Git repository.
    """
    try:
        subprocess.run(["git", "add", "."], check=True)
        print("All changes have been staged.")
    except subprocess.CalledProcessError as e:
        print("Error staging changes:", e)
        sys.exit(1)

def get_git_diff():
    """
    Fetch the current git diff summary.
    """
    try:
        diff = subprocess.check_output(["git", "diff", "--staged"], text=True)
        if not diff.strip():
            print("No staged changes to generate a commit message for.")
            sys.exit(0)
        return diff
    except subprocess.CalledProcessError as e:
        print("Error fetching git diff:", e)
        sys.exit(1)

def generate_commit_message(diff):
    """
    Use OpenAI API to generate a commit message based on the diff summary.
    """
    prompt = f"""
    Generate a concise and professional Git commit message for the following changes:
    
    {diff}
    """
    try:
        response = client.chat.completions.create(
            model="gpt-4",
            messages=[
                {"role": "system", "content": "You are a helpful assistant for generating Git commit messages."},
                {"role": "user", "content": prompt}
            ]
        )
        commit_message = response.choices[0].message.content.strip()
        return commit_message
    except Exception as e:
        print("Error generating commit message:", e)
        sys.exit(1)

def apply_commit(commit_message):
    """
    Apply the generated commit message using Git.
    """
    try:
        subprocess.run(["git", "commit", "-m", commit_message], check=True)
        print("Commit successfully applied!")
    except subprocess.CalledProcessError as e:
        print("Error applying commit:", e)
        sys.exit(1)

def push_changes():
    """
    Push the committed changes to the remote repository.
    """
    confirm = input("\nDo you want to push the changes to the remote repository? [y/N]: ").strip().lower()
    if confirm == 'y':
        try:
            subprocess.run(["git", "push"], check=True)
            print("Changes successfully pushed to the remote repository!")
        except subprocess.CalledProcessError as e:
            print("Error pushing changes:", e)
            sys.exit(1)
    else:
        print("Push aborted.")

def main():
    """
    Main function to run the CLI tool.
    """
    # Automatically stage all changes
    stage_changes()

    # Get the git diff
    diff = get_git_diff()

    # Generate a commit message
    print("Generating commit message...")
    commit_message = generate_commit_message(diff)
    print("\nSuggested commit message:\n")
    print(commit_message)

    # Automatically apply the commit
    apply_commit(commit_message)

    # Ask the user if they want to push the changes
    push_changes()

if __name__ == "__main__":
    main()