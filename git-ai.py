import os
import subprocess
from openai import OpenAI
import argparse
import sys

ZSHRC_PATH = os.path.expanduser("~/.zshrc")

def get_from_zshrc(variable_name):
    """
    Retrieve a variable value from ~/.zshrc.
    """
    if not os.path.exists(ZSHRC_PATH):
        return None
    with open(ZSHRC_PATH, "r") as file:
        for line in file:
            if line.startswith(f"export {variable_name}="):
                return line.split("=", 1)[1].strip().strip('"')
    return None

def save_to_zshrc(variable_name, value):
    """
    Save a variable to ~/.zshrc.
    """
    with open(ZSHRC_PATH, "a") as file:
        file.write(f'\nexport {variable_name}="{value}"\n')
    print(f"{variable_name} saved to ~/.zshrc. Run `source ~/.zshrc` to apply changes.")

def update_api_key():
    """
    Prompt the user for a new API key and save it to ~/.zshrc.
    """
    global api_key
    api_key = input("Enter a valid GroqAI API key: ").strip()
    save_to_zshrc("OPENAI_API_KEY", api_key)
    print("API key updated. Please run `source ~/.zshrc` to apply changes.")

# Load GroqAI API Key and Model from zshrc
api_key = get_from_zshrc("OPENAI_API_KEY")
if not api_key:
    update_api_key()

model = get_from_zshrc("OPENAI_MODEL") or "llama3-70b-8192"

# Initialize GroqAI client
client = OpenAI(base_url="https://api.groq.com/openai/v1", api_key=api_key)

def save_model_choice(new_model):
    """
    Save the selected model to ~/.zshrc.
    """
    save_to_zshrc("OPENAI_MODEL", new_model)
    print(f"Model '{new_model}' saved to ~/.zshrc.")

def list_available_models():
    """
    List all available GroqAI models.
    """
    try:
        response = client.models.list()
        print("Available models:")
        for model_info in response.data:
            print(f"- {model_info.id}")
    except Exception as e:
        if "invalid_api_key" in str(e).lower():
            print("Invalid API key. Please provide a valid key.")
            update_api_key()
            # Retry listing models after updating the API key
            list_available_models()
        else:
            print(f"Error fetching available models: {e}")
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
    Use GroqAI API to generate a commit message based on the diff summary.
    Handles `model_not_found` error by prompting the user to select another model dynamically.
    """
    # Fetch the current model dynamically
    current_model = get_from_zshrc("OPENAI_MODEL") or "llama3-70b-8192"
    
    prompt = f"""
    Generate a concise and professional Git commit message for the following changes:
    
    {diff}
    """
    try:
        response = client.chat.completions.create(
            model=current_model,
            messages=[
                {"role": "system", "content": "You are a helpful assistant for generating Git commit messages."},
                {"role": "user", "content": prompt}
            ]
        )
        commit_message = response.choices[0].message.content.strip()
        return commit_message
    except Exception as e:
        error_message = str(e).lower()
        
        if "model_not_found" in error_message:
            print(f"Error: The model '{current_model}' does not exist or you do not have access to it.")
            print("Fetching available models...")
            
            try:
                # List available models and prompt user to choose
                response = client.models.list()
                print("Available models:")
                available_models = [model_info.id for model_info in response.data]
                for idx, model_name in enumerate(available_models, start=1):
                    print(f"{idx}. {model_name}")
                
                # Prompt user for new model selection
                choice = int(input("Enter the number of the model you want to use: ")) - 1
                if choice < 0 or choice >= len(available_models):
                    raise ValueError("Invalid choice. Please select a valid model number.")
                
                # Update model and save to ~/.zshrc
                new_model = available_models[choice]
                save_model_choice(new_model)
                
                # Retry generating the commit message with the new model
                print(f"Switching to model '{new_model}' and retrying...")
                return generate_commit_message(diff)
            
            except Exception as model_error:
                print(f"Error fetching models or updating model: {model_error}")
                sys.exit(1)
        
        else:
            print(f"Error generating commit message: {e}")
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
    parser = argparse.ArgumentParser(description="Auto-generate Git commit messages using GroqAI.")
    
    # Add argument to list available models
    parser.add_argument("-l", "--list-models", action="store_true", help="List all available GroqAI models.")
    
    # Add argument to set the model
    parser.add_argument("-s", "--set-model", type=str, help="Set the model to use for generating commit messages.")
    
    # Add argument to set a new API key
    parser.add_argument("-k", "--set-key", action="store_true", help="Set a new GroqAI API key.")
    
    # Parse arguments
    args = parser.parse_args()

    if args.list_models:
        list_available_models()
        sys.exit(0)

    if args.set_model:
        save_model_choice(args.set_model)
        sys.exit(0)

    if args.set_key:
        update_api_key()
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