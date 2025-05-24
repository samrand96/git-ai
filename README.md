# Git-AI: Your Friendly AI-Powered Commit Companion

Git-AI transforms your staged changes into clear, human-sounding Git commit messages—powered by OpenAI’s GPT models. It handles staging, message generation, inline review, committing, and pushing, so you can focus entirely on code.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Usage](#usage)

   * [Generating a Commit](#generating-a-commit)
   * [Listing Models](#listing-models)
   * [Changing Format](#changing-format)
   * [Editing Configuration](#editing-configuration)
5. [Advanced: Standalone Binary](#advanced-standalone-binary)
6. [To-Do / Future Plans](#to-do--future-plans)
7. [License](#license)

---

## Prerequisites

Before you begin, ensure you have:

* **Python 3.8+** installed and on your `PATH`.
* **Git** installed and configured (username/email set).
* An **OpenAI API key**. You can obtain one at [https://platform.openai.com](https://platform.openai.com).
* (Optional) **pyinstaller** if you plan to create a standalone binary.

---

## Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/samrand96/git-ai.git
   cd git-ai
   ```

2. **Install dependencies**:

   ```bash
   pip install -r requirements.txt
   ```

3. **Make the script executable** (Unix/macOS):

   ```bash
   chmod +x git-ai.py
   ```

---

## Configuration

On first run, Git-AI will automatically prompt you to:

1. Enter your **OpenAI API key**.
2. Choose a **model** from the fetched list (default `gpt-4`).
3. Select your default **commit style**: `detailed` (multi-sentence) or `one-line`.

These settings are saved in an INI file at:

* **Linux**: `~/.config/samrand.git.ini`
* **macOS**: `~/.config/samrand.git.ini`
* **Windows**: `%APPDATA%\samrand.git.ini`

> **Tip**: You can open this file in any text editor to inspect or manually tweak values.

To reconfigure at any time, simply run:

```bash
python git-ai.py --config
```

This enters interactive mode, showing current values and letting you update only the fields you wish (leave blank to retain the existing setting).

---

## Usage

### Generating a Commit

1. **Stage your changes** in Git (e.g., `git add .`).
2. Run:

   ```bash
   python git-ai.py
   ```
3. The tool will:

   * Stage all unstaged changes.
   * Detect your **current branch** name and extract any ticket prefix (e.g. `ABC-123`).
   * Generate both a **detailed** and a **one-line** commit message.
   * Display the chosen message inline.
   * Prompt: **Edit before commit?**
     • Press **Enter** to skip edits.
     • Type your own message, ending with an empty line, to replace it.
   * Commit the message.
   * Prompt: **Push changes?** `[Y/n]`.

### Listing Models

To see all available OpenAI models:

```bash
python git-ai.py --list-models
```

Pick any model by its exact name or its index when reconfiguring.

### Changing Format

Override or switch your default commit style:

```bash
# One-off:
python git-ai.py -f one-line

# Persist default:
python git-ai.py -f detailed
```

This updates your INI so future runs use the selected style automatically.

### Editing Configuration

Enter full configuration mode to update API key, model, or format:

```bash
python git-ai.py -c
```

Follow the interactive prompts. Leave blank to keep existing values.

---

## Advanced: Standalone Binary

If you’d rather run `git-ai` without Python installed at runtime:

1. **Install PyInstaller** (if you haven’t already):

   ```bash
   pip install pyinstaller
   ```
2. **Build the executable**:

   ```bash
   pyinstaller --onefile git-ai.py
   ```
3. **Copy it into your PATH**:

   ```bash
   mv dist/git-ai /usr/local/bin/git-ai
   chmod +x /usr/local/bin/git-ai
   ```
4. Now run:

   ```bash
   git-ai
   ```

---

## To-Do / Future Plans

- [ ] Support for Conventional Commits: automatically enforce and generate messages in the `type(scope): description` format (feat, fix, docs, etc.).
- [ ] Pre-commit and commit-msg hooks: integrate Git hooks to validate or reformat the AI-generated commit before it’s saved.
- [ ] Issue tracker integration: detect and link JIRA, GitHub, or GitLab issue IDs to commits, and fetch issue titles for context.
- [ ] Automated semantic version tagging: analyze commit types and suggest or apply version bumps (major, minor, patch) and tag accordingly.
- [ ] Changelog automation: aggregate and format commit messages into a CHANGELOG.md, grouped by version and type.
- [ ] Custom branch naming rules: allow teams to define and validate branch patterns for consistent ticket prefixes.
- [ ] Offline/fallback mode: cache recent commit templates or model outputs so basic messages can be generated without internet.
- [ ] Performance profiling and optimization: track and reduce latency for commit generation in large repositories.
- [ ] AI security check-up: scan generated commit text for potential secrets, PII, or insecure patterns before committing.
- [ ] Template configuration: allow users to define custom commit templates with optional footers, co-author tags, or emojis.
- [ ] Multi-language support: enable commit messages in different languages based on project locale or user preference.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

*Git-AI: Let AI handle the words so you can handle the code.*
