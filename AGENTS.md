# AGENTS.md

This repository is maintained by coding agents and humans. Treat this file as mandatory operating procedure. Do not ignore it because a change looks small.

## Prime Directive

Git-AI must stay modular, production-ready, and maintainable by multiple developers. Do not create giant files, debug-dump user errors, hide failed provider calls, or mix unrelated responsibilities.

If a change does not have a clear home, create the right module first. Do not stuff it into the nearest file.

## Hard Rules

1. No monoliths.
   - Do not add broad new logic to files that are already responsible for orchestration.
   - Keep files focused. As a rule of thumb, prefer files under 200 lines. Going beyond that requires a strong reason and clean internal structure.
   - Split by responsibility, not by convenience.

2. No raw debug errors for users.
   - Never let `main` return a raw `Result` that prints `Error: ...`.
   - User-facing errors must go through `AppError::user_message()`.
   - Never print raw provider JSON bodies directly to the terminal.
   - Never expose full API keys, tokens, Authorization headers, or provider secrets.

3. Interactive config must be real interactive UI.
   - Use arrow-key prompts for choices.
   - Do not force users to type numbers for menus.
   - API keys must display as masked status, with the env var source when known.
   - Model selection must try provider discovery after provider, API key, and base URL are configured.
   - If model discovery fails or returns no models, show a readable warning and allow manual model input.

4. Themes must respect environment and user choice.
   - Default theme is `auto`.
   - `auto` must resolve using system dark/light detection when available.
   - Built-in and custom RGB themes must be handled by the theme subsystem, not scattered around CLI code.
   - Do not hardcode color escape sequences outside `src/theme`.

5. Provider keys must not bleed across vendors.
   - Provider-specific keys such as `OPENAI_API_KEY` and `DEEPSEEK_API_KEY` take priority.
   - Generic `AI_API_KEY` is only a fallback.
   - Interactive config should store newly entered secrets in the active provider-specific variable.

6. No silent network/provider failures.
   - If provider model listing fails, say that it failed and why in user-safe language.
   - If auth fails, show a clean `[auth]` error.
   - If rate-limited, show a clean `[rate]` error.
   - If the provider returns malformed data, show a clean `[data]` error.

7. Preserve user work.
   - Do not reset, checkout, or revert changes unless explicitly asked.
   - Do not stage or unstage files unless explicitly asked.
   - If the repo is already partially staged, keep that state intact and mention it in the final response when relevant.

## Current Architecture

### `src/main.rs`

Owns CLI dispatch and top-level error presentation only.

Allowed:
- Parse CLI args.
- Load config.
- Dispatch to command modules.
- Catch `AppResult` and render `AppError::user_message()`.

Forbidden:
- Provider-specific logic.
- Theme internals.
- Prompt-building logic.
- Raw `Debug` output.

### `src/cli/`

Owns user-facing command workflows.

Command modules should orchestrate work, not contain every detail.

Current command layout:
- `commit.rs`: commit-message workflow.
- `list_models.rs`: standalone model listing.
- `config/`: interactive and non-interactive configuration.
- `review/`: target-branch and fast local AI review workflow.

### `src/cli/config/`

Configuration UI is split by concern:
- `mod.rs`: command orchestration.
- `display.rs`: current config display and secret masking.
- `prompts.rs`: reusable prompt helpers.
- `provider_select.rs`: provider and commit-format selection.
- `model_select.rs`: endpoint model discovery and fallback input.
- `theme_select.rs`: built-in/custom theme selection and RGB creation.

Rules:
- Add prompt controls in `prompts.rs`.
- Add new config display text in `display.rs`.
- Add provider choice behavior in `provider_select.rs`.
- Add model lookup behavior in `model_select.rs`.
- Add theme behavior in `theme_select.rs`.

Do not move this logic back into `mod.rs`.

### `src/cli/review/`

Review is split by concern:
- `mod.rs`: command orchestration, provider readiness, AI invocation.
- `branches.rs`: git fetch status and origin branch tree navigation.
- `diff.rs`: numstat summaries, large/binary diff detection, exclusion prompts.
- `limits.rs`: changed-file and changed-line policy thresholds.
- `notes.rs`: optional task/ticket and reviewer-comment input capture.
- `prompt.rs`: review prompt construction.
- `output.rs`: terminal/Markdown report output.

Rules:
- Fetch remotes by default unless the user explicitly passes the skip-fetch flag or `--fast`.
- `--fast` must review current staged and unstaged changes without target-branch comparison.
- Branch selection must stay arrow-key driven and support folder-style navigation plus `..`.
- Always include `origin/<current-branch>` when it exists; mark it as current in the selector.
- Large or binary files must be detected before sending a diff to AI.
- Large file exclusion prompts must describe exclusion from the AI review payload, not deletion or git fetch.
- Do not send huge generated text, XML, lockfiles, or binary-style diffs without giving the user a chance to exclude them.
- Default large-diff checks must be enabled. `--no-large-check` is the explicit escape hatch.
- Task/ticket checks belong behind `--task`; prior reviewer-comment checks belong behind `--comments`.
- Prompt wording changes go in `prompt.rs`.
- Markdown/terminal rendering changes go in `output.rs`.
- Keep threshold policy in `limits.rs`.

### `src/config/`

Persistent configuration is split by concern:
- `schema.rs`: TOML schema structs only.
- `provider.rs`: provider defaults, known provider names, key lookup priority.
- `paths.rs`: config and env file locations.
- `store.rs`: `AppConfig` load/save/accessor API.

Rules:
- Do not put provider defaults in CLI code.
- Do not put TOML structs in prompt code.
- Do not put filesystem path logic in provider logic.
- Keep `AppConfig` as the public boundary for config consumers.

### `src/services/ai/`

AI provider transport is split by provider/protocol:
- `mod.rs`: `AiClient` public API and dispatch.
- `openai_compatible.rs`: OpenAI-compatible chat completions and models.
- `anthropic.rs`: Anthropic messages and models.
- `gemini.rs`: Gemini generateContent and models.
- `ollama.rs`: Ollama generation and tags.
- `messages.rs`: shared message composition helpers.

Rules:
- Add a new provider in its own module.
- Keep request/response parsing provider-specific.
- Shared helpers must stay small and protocol-neutral.
- Do not add provider-specific conditionals all over unrelated modules.

### `src/theme/`

Theme and terminal styling is split by concern:
- `palette.rs`: RGB struct, palette structs, built-in palettes, RGB parsing.
- `detect.rs`: system dark/light detection.
- `style.rs`: active theme, color application, CLI output highlighting.

Rules:
- No color escape sequences outside `src/theme`.
- No OS dark/light detection outside `detect.rs`.
- No RGB parsing outside `palette.rs`.
- `src/utils/colors.rs` is a compatibility re-export only.

### `src/utils/`

Small cross-cutting helpers only.

Allowed:
- Git command wrappers.
- HTTP wrapper.
- Environment file helpers.
- Cleanup helpers.
- JSON helpers.
- Error type and sanitization.
- Compatibility re-exports.

Forbidden:
- Whole features.
- Command workflows.
- Provider-specific logic.
- Theme implementation.

## Error Handling Standards

Use `AppResult<T>` for fallible operations.

Do:
- Return errors upward.
- Render final errors once at the top level.
- Use concise status prefixes: `[auth]`, `[cfg]`, `[net]`, `[http]`, `[data]`, `[git]`, `[prompt]`.
- Sanitize provider messages.

Do not:
- `unwrap()` or `expect()` in production paths.
- Print `Debug` representations to users.
- Dump raw HTTP bodies.
- Show full secrets.

Tests should cover sanitizer behavior when changing error formatting.

## Config UX Standards

Interactive config must follow this order:

1. Provider selection.
2. Commit format selection.
3. Theme selection.
4. API key status display and optional replacement.
5. Base URL input.
6. Model discovery from the selected provider and base URL.
7. Manual model input fallback when discovery fails or returns empty.

Model discovery failures must never abort config unless the prompt system itself fails.

## Theme UX Standards

Theme behavior must support:
- `auto` dark/light mode.
- Built-in themes.
- Custom RGB themes.
- Saved theme in config.
- `AI_THEME` env override.

Custom RGB values must accept:
- `R,G,B`
- `R G B`
- `#RRGGBB`

Invalid RGB input must re-prompt with a readable warning.

## Verification Requirements

For normal code changes, run:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

For CLI behavior changes, also run a smoke command where possible:

```bash
cargo build --target-dir target/codex-build
target/codex-build/debug/gai --help
target/codex-build/debug/gai config --help
```

If the normal `target/debug/gai.exe` is locked on Windows, do not kill user processes without permission. Use a separate target dir for verification.

## Pull Request Expectations

Every non-trivial PR should include:
- Summary of architectural areas touched.
- User-visible behavior changes.
- Error handling impact.
- Verification commands run.
- Any remaining risks or follow-up work.

If a PR makes a file significantly larger, explain why splitting was not appropriate.

## MCP / Agent Operating Rules

Agents working through MCP tools must:
- Inspect existing structure before editing.
- Prefer adding or modifying the correct module over patching a nearby file.
- Keep tool output and final summaries concise but specific.
- Never expose secrets found in env files, logs, provider payloads, or terminal output.
- Preserve dirty worktree and staged state.
- Use repository-local conventions over generic patterns.
- Update this file when architecture rules change.

Agents must not:
- Invent parallel architecture outside this layout.
- Create duplicate config or theme systems.
- Add broad dependencies without a clear reason.
- Replace interactive arrow-key flows with typed-number menus.
- Reintroduce emoji-heavy output or playful status messages.
- Let provider auth failures crash into raw debug output.
