# Git-AI

Git-AI is a professional, provider-agnostic CLI that turns your staged Git changes into high-quality commit messages and AI-powered code reviews. This Rust rewrite delivers faster startup, safer error handling, and multi-platform release builds.

## Features
- AI-generated commit messages with ticket-prefix detection
- AI reviews with target-branch readiness checks, fast local mode, task/comment verification, and large-diff exclusions
- Provider-agnostic HTTP client (no vendor SDKs)
- Environment-based configuration for secrets
- Arrow-key interactive configuration for provider, model, commit format, and themes
- Saved terminal color themes with custom RGB palettes and automatic dark/light detection
- Cross-platform release builds for Linux, Windows, and macOS

## Installation

### From Source
```bash
git clone https://github.com/ivashchenko96/git-artificial-intelligent.git
cd git-artificial-intelligent
cargo build --release
```

The binary will be available at:
```
target/release/gai
```

### Cargo Install (local)
```bash
cargo install --path .
```

## Usage

### Generate a Commit Message
```bash
git add .
gai commit
```

Optional flags:
```bash
gai commit --format one-line
gai commit --provider openai --model gpt-4o-mini
gai commit --push
gai commit --no-verify
```

### AI Review
```bash
gai review
gai review --target develop
gai review --target origin/staging --task --comments
gai review --fast
gai review --fast --task --comments
gai review --large-threshold 800 --max-changed-files 50 --max-added-lines 800 --max-deleted-lines 500
gai review --no-large-check
gai review --markdown --output review.md
```

By default, `gai review` fetches remotes, lets you navigate `origin` branches with arrow-key folder navigation, compares the selected target branch against `HEAD`, detects large or binary file diffs, and lets you exclude those files before the AI review payload is sent.

Use `--fast` to skip remote fetching and target-branch comparison and review only current staged and unstaged changes. Use `--task` to paste the original ticket or acceptance criteria and have the report verify whether the branch appears to complete it. Use `--comments` to paste previous PR comments or reviewer feedback and check whether they were addressed; `--coments` is accepted as an alias. Large-diff checks are enabled by default with a `5000` changed-line threshold; use `--large-threshold` or the other max options to tune the policy, or `--no-large-check` to disable it explicitly.

### List Models
```bash
gai list-models
```

### Configure Settings
```bash
gai config --interactive
gai config --theme
gai config --set PROVIDER openai
gai config --set THEME matrix
gai config --set-provider openai MODEL gpt-4o-mini
```

The interactive config flow uses arrow-key menus for provider, commit format, theme selection, and model selection. It checks the configured endpoint for available models after the API key and base URL are set; if discovery fails or returns no models, it falls back to manual model entry. Custom themes can be created from RGB triples such as `80,255,145` or hex values such as `#50ff91`.

## Environment Variables
Secrets must be stored in environment variables. Git-AI loads them from your shell environment or from `~/.config/gai/.env`. Provider-specific keys take priority over the generic `AI_API_KEY`, so switching providers cannot silently reuse the wrong key.

Preferred variables:
- `AI_API_KEY` – API key for the active provider
- `AI_BASE_URL` – Override base URL (optional)
- `AI_MODEL` – Override model (optional)
- `AI_PROVIDER` – Override provider (optional)
- `AI_COMMIT_FORMAT` – `detailed` or `one-line`
- `AI_THEME` – Override saved theme (optional, defaults to `auto`)

Provider-specific variables:
- `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GEMINI_API_KEY`, `GROQ_API_KEY`, `DEEPSEEK_API_KEY`, `PERPLEXITY_API_KEY`
- `OPENAI_BASE_URL`, `OLLAMA_BASE_URL`, `LMSTUDIO_BASE_URL`, `GROQ_BASE_URL`, `DEEPSEEK_BASE_URL`, `PERPLEXITY_BASE_URL`

## Configuration File
Git-AI stores non-secret defaults in:
```
~/.config/gai/config.toml
```

Example:
```toml
provider = "openai"
commit_format = "detailed"
theme = "auto"

[providers.openai]
model = "gpt-4o-mini"
base_url = "https://api.openai.com/v1"

[themes.lab-green]
error = [255, 92, 92]
success = [60, 220, 110]
warning = [190, 210, 95]
info = [96, 190, 135]
header = [80, 255, 145]
highlight = [226, 255, 230]
dim = [86, 122, 96]
```

## Development Setup
```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Project Structure
```
src/
  main.rs
  lib.rs
  cli/
    config/
    review/
  config/
  services/
    ai/
  theme/
  utils/
```

For module ownership and extension points, see [ARCHITECTURE.md](ARCHITECTURE.md).

## GitHub Actions Builds
- `.github/workflows/build.yml` builds, formats, lints, tests, and uploads artifacts for Linux, Windows, and macOS.
- `.github/workflows/release.yml` builds tagged releases and uploads binaries to GitHub Releases.

## Release Builds
Tag a release to trigger automated binaries:
```bash
git tag v1.0.0
git push origin v1.0.0
```

Artifacts are packaged per target and attached to the GitHub release.

## Troubleshooting
- **Missing API key**: set the provider-specific key such as `OPENAI_API_KEY` or `DEEPSEEK_API_KEY` in your environment or `~/.config/gai/.env`; `AI_API_KEY` is only a fallback.
- **No changes to review/commit**: stage or modify files and retry.
- **Provider errors**: verify `AI_BASE_URL` and model name for your provider.


## To-Do / Future Plans

- [x] **AI Review**: Professional code analysis with CLI colors, Markdown reports, target-branch readiness checks, and fast local review mode.
- [ ] Support for Conventional Commits: automatically enforce and generate messages in the `type(scope): description` format (feat, fix, docs, etc.).
- [ ] Pre-commit and commit-msg hooks: integrate Git hooks to validate or reformat the AI-generated commit before it’s saved.
- [ ] Issue tracker integration: detect and link JIRA, GitHub, or GitLab issue IDs to commits, and fetch issue titles for context.
- [ ] Automated semantic version tagging: analyze commit types and suggest or apply version bumps (major, minor, patch) and tag accordingly.
- [ ] Changelog automation: aggregate and format commit messages into a CHANGELOG.md, grouped by version and type.
- [ ] Custom branch naming rules: allow teams to define and validate branch patterns for consistent ticket prefixes.
- [ ] Offline/fallback mode: cache recent commit templates or model outputs so basic messages can be generated without internet.
- [ ] Performance profiling and optimization: track and reduce latency for commit generation in large repositories.
- [ ] AI security check-up: scan generated commit text for potential secrets, PII, or insecure patterns before committing.
- [ ] Template configuration: allow users to define custom commit templates with optional footers or co-author tags.
- [ ] Multi-language support: enable commit messages in different languages based on project locale or user preference.

--
## License
MIT. See [LICENSE](LICENSE).
