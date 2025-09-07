# Git-AI: Your Friendly AI-Powered Commit Companion

Git-AI transforms your staged changes into clear, human-sounding Git commit messages—powered by LLMs (OpenAI, Ollama, Anthropic, and more). It handles staging, message generation, inline review, committing, and pushing, so you can focus entirely on code. The new architecture is provider-agnostic, modular, and fully extensible.

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
   * [Switching Providers](#switching-providers)
5. [Cross-Platform Build Scripts](#advanced-standalone-binary)
6. [Advanced: Standalone Binary (DIY)](#advanced-standalone-binary)
7. [To-Do / Future Plans](#to-do--future-plans)
8. [License](#license)

---

## Prerequisites

Before you begin, ensure you have:

* **Python 3.8+** installed and on your `PATH`.
* **Git** installed and configured (username/email set).
* An **API key** for your chosen provider (OpenAI, Anthropic, etc). You can obtain an OpenAI key at [https://platform.openai.com](https://platform.openai.com). Ollama requires a local server running.
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
   chmod +x main.py
   ```

---

## Configuration

On first run, Git-AI will automatically prompt you to:


1. Select your **provider** (OpenAI, Ollama, Anthropic, ...).
2. Enter your **API key** (if required by provider).
3. Choose a **model** from the fetched list (or type your own).
4. Select your default **commit style**: `detailed` (multi-sentence) or `one-line`.

These settings are saved in an INI file at:


* **Linux**: `~/.config/sam.git.ini`
* **macOS**: `~/.config/sam.git.ini`
* **Windows**: `%APPDATA%\sam.git.ini`

> **Tip**: You can open this file in any text editor to inspect or manually tweak values.


To reconfigure at any time, simply run:

```bash
python main.py config --interactive
```

This enters interactive mode, showing current values and letting you update only the fields you wish (leave blank to retain the existing setting). You can also set values directly:

```bash
python main.py config --set PROVIDER ollama
python main.py config --set-provider openai API_KEY sk-...
python main.py config --set-provider ollama MODEL llama3
```

---

## Usage

### Generating a Commit


1. **Stage your changes** in Git (e.g., `git add .`).
2. Run:

   ```bash
   python main.py commit
   ```
3. The tool will:
   * Stage all unstaged changes.
   * Detect your **current branch** name and extract any ticket prefix (e.g. `ABC-123`).
   * Generate a commit message using your selected provider and model.
   * Display the chosen message inline.
   * Prompt: **Edit before commit?**
     • Press **Enter** to skip edits.
     • Type your own message, ending with an empty line, to replace it.
   * Commit the message.
   * Prompt: **Push changes?** `[Y/n]`.


### Listing Models

To see all available models for your current provider:

```bash
python main.py list-models
```

Pick any model by its exact name or its index when reconfiguring.


### Changing Format

Override or switch your default commit style:

```bash
python main.py config --set COMMIT_FORMAT one-line
python main.py config --set COMMIT_FORMAT detailed
```

This updates your config so future runs use the selected style automatically.


### Editing Configuration

Enter full configuration mode to update provider, API key, model, or format:

```bash
python main.py config --interactive
```

Follow the interactive prompts. Leave blank to keep existing values.

You can also set any value directly (see above).

### Switching Providers

To switch between OpenAI, Ollama, Anthropic, Gemini, or LMStudio (or add your own):

```bash
python main.py config --set PROVIDER ollama
python main.py config --set PROVIDER openai
```

Then use `--set-provider` to update provider-specific settings as needed.

---

## Cross-Platform Build Scripts

To easily build a standalone executable and set up your environment, use the provided scripts:

### Windows

- Use `build.bat` (right-click and run as administrator):
   - Checks for PyInstaller and builds with no prompt.
   - Optionally adds the `dist` folder to your PATH for the session or permanently.
   - Shows a PowerShell command for session use if desired.

### Linux/macOS

- Use `build.sh`:
   - Make it executable: `chmod +x build.sh`
   - Run: `./build.sh`
   - Checks for PyInstaller and builds with no prompt.
   - Optionally adds the `dist` folder to your PATH for the session or permanently (shell profile).

After building, you can run the CLI from anywhere if you add it to your PATH.

---

## Advanced: Standalone Binary

If you’d rather run `git-ai` without Python installed at runtime:

1. **Install PyInstaller** (if you haven’t already):

   ```bash
   pip install pyinstaller
   ```
2. **Build the executable**:

   ```bash
   pyinstaller --onefile main.py
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
