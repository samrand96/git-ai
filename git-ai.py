import os
import subprocess
from openai import OpenAI
from dotenv import load_dotenv
import argparse
import sys

# Load .env file if it exists
load_dotenv()

# Initialize OpenAI client
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    api_key = input("OpenAI API key not found. Please enter your API key: ").strip()
    with open(".env", "a") as env_file:
        env_file.write(f"\nOPENAI_API_KEY={api_key}")
    print("API key saved in .env file.")

client = OpenAI(api_key=api_key)

# Get the model to use (default to gpt-4)
model = os.getenv("OPENAI_MODEL", "gpt-4")

def save_model_choice(new_model):
    """
    Save the selected model in the .env file.
    """
    with open(".env", "a") as env_file:
        env_file.write(f"\nOPENAI_MODEL={new_model}")
    print(f"Model '{new_model}' saved in .env file.")

def list_available_models():
    """
    List all available OpenAI models.
    """
    try:
        response = client.models.list()
        print("Available models:")
        for model in response.data:
            print(f"- {model['id']}")
    except Exception as e:
        print("Error fetching available models:", e)
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
            model=model,
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
    confirm = input("\nDo you want to push the changes to the remote repository? [Y/n]: ").strip().lower()
    if confirm == 'n':
        print("Push aborted.")
    else:
        try:
            subprocess.run(["git", "push"], check=True)
            print("Changes successfully pushed to the remote repository!")
        except subprocess.CalledProcessError as e:
            print("Error pushing changes:", e)
            sys.exit(1)
        

def main():
    """
    Main function to run the CLI tool.
    """
    parser = argparse.ArgumentParser(description="Auto-generate Git commit messages using OpenAI GPT.")
    parser.add_argument("--list-models", action="store_true", help="List all available OpenAI models.")
    parser.add_argument("--set-model", type=str, help="Set the model to use for generating commit messages.")
    args = parser.parse_args()

    if args.list_models:
        list_available_models()
        sys.exit(0)

    if args.set_model:
        save_model_choice(args.set_model)
        sys.exit(0)

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