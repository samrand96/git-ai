# Git-AI: AI-Powered Git Commit Message Generator

Git-AI is a powerful command-line tool that leverages OpenAI's GPT models to generate professional, concise, and context-aware Git commit messages. It simplifies your development workflow by automatically staging changes, creating commit messages, and optionally pushing them to your remote repository. With support for managing OpenAI API keys and models directly from your environment, Git-AI ensures a seamless experience for developers.

---

## Features

- **AI-Powered Commit Messages**: Generate clear and professional commit messages using OpenAI's GPT models.
- **Automatic Git Staging**: Automatically stages all changes before generating commit messages.
- **Model Management**: Easily list and select from available OpenAI models.
- **Zsh Integration**: Save and retrieve OpenAI API keys and models directly from your `~/.zshrc` file.
- **Push Confirmation**: Optionally push your commits to the remote repository after applying them.

---

## Requirements

- Python 3.7 or higher
- OpenAI API key
- Git installed and configured
- Zsh as the default shell

---

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/samrand96/git-ai.git
   cd git-ai
   ```

2. **Install Dependencies**:
   ```bash
   pip install openai
   ```

3. **Ensure Zsh Shell is Set**:
   Make sure you're using Zsh (`echo $SHELL` should return `/bin/zsh`).

---

## Usage

### Initial Setup

1. **Run the Tool**:
   If your OpenAI API key is not already set, the tool will prompt you to enter it:
   ```bash
   python git-ai.py
   ```
   - The API key will be saved to your `~/.zshrc` for future use.

2. **Set a Default Model**:
   To set a specific OpenAI model (e.g., `gpt-3.5-turbo`), use:
   ```bash
   python git-ai.py -s gpt-3.5-turbo
   ```
   - The model choice will also be saved to `~/.zshrc`.

---

### Workflow

1. **Generate and Apply a Commit Message**:
   ```bash
   python git-ai.py
   ```
   - Automatically stages changes.
   - Generates an AI-powered commit message.
   - Applies the commit.
   - Prompts to push the changes.

2. **List Available OpenAI Models**:
   ```bash
   python git-ai.py -l
   ```
   Example Output:
   ```
   Available models:
   - gpt-4
   - gpt-3.5-turbo
   - davinci
   - curie
   ```

3. **Change the OpenAI Model**:
   ```bash
   python git-ai.py -s gpt-4
   ```
   Example Output:
   ```
   Model 'gpt-4' saved to ~/.zshrc.
   ```

4. **Push Changes**:
   After generating and applying a commit message, Git-AI will ask:
   ```
   Do you want to push the changes to the remote repository? [Y/n]:
   ```
   Type `Y` or press Enter to push.

---

## Configuration Management

### Save API Key and Model to `~/.zshrc`
Git-AI stores your API key and selected model in `~/.zshrc`:
```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_MODEL="gpt-4"
```
Run `source ~/.zshrc` after any changes to apply them.

### Retrieve Configuration
Git-AI automatically retrieves the API key and model from your `~/.zshrc` file. If any value is missing, the tool will prompt you to provide it.

---

## Example Output

```bash
python git-ai.py
```

Output:
```
All changes have been staged.
Generating commit message...

Suggested commit message:
- Fix null pointer exception in user login flow
- Update documentation for API integration

Commit successfully applied!

Do you want to push the changes to the remote repository? [Y/n]: Y
Changes successfully pushed to the remote repository!
```

---

## Creating a Binary

You can generate a standalone executable (binary) of Git-AI using [PyInstaller](https://www.pyinstaller.org/). Here’s how:

1. **Install PyInstaller**:
   ```bash
   pip install pyinstaller
   ```
2. **Create the Executable**:
   ```bash
   pyinstaller --onefile git-ai.py
   ```
   - PyInstaller will bundle all dependencies into a single binary located in the `dist/` directory.
3. **Run the Binary**:
   ```bash
   ./dist/git-ai
   ```
4. **(Optional) Make It Global**:
   Move the binary to a directory in your system path:
   ```bash
   mv dist/git-ai /usr/local/bin/git-ai
   chmod +x /usr/local/bin/git-ai
   ```
   Now you can run `git-ai` from anywhere.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

Git-AI: Simplify your Git workflow with the power of AI! 🚀