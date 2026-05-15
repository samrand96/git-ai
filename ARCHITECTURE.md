# Architecture

Git-AI is organized around feature boundaries instead of dumping command, provider, and UI logic into single files. Keep new work close to the responsibility it changes.

## Top-Level Layers

- `src/main.rs` owns CLI dispatch and top-level error presentation.
- `src/cli/` owns user-facing commands and interactive flows.
- `src/config/` owns persisted configuration, provider defaults, config paths, and config schema.
- `src/services/` owns provider communication and response parsing.
- `src/theme/` owns terminal palettes, dark/light detection, and output styling.
- `src/utils/` owns small cross-cutting helpers such as Git, HTTP transport, cleanup, environment loading, and compatibility re-exports.

## CLI Commands

- `src/cli/review/` handles target-branch reviews and fast local reviews:
  - `mod.rs` orchestrates review mode selection, fetch, diff filtering, and AI invocation.
  - `branches.rs` owns remote fetch handling and origin branch tree navigation.
  - `diff.rs` owns numstat summaries, large/binary file detection, and exclusion prompts.
  - `limits.rs` owns changed-file and changed-line threshold policy.
  - `notes.rs` captures optional task/ticket and previous-comment context.
  - `prompt.rs` builds the AI review prompt.
  - `output.rs` renders terminal/Markdown output.
- `src/cli/commit.rs` handles commit-message generation and commit orchestration.
- `src/cli/list_models.rs` handles the standalone model-listing command.
- `src/cli/config/` is split by workflow responsibility:
  - `mod.rs` orchestrates the command.
  - `display.rs` renders current config and masked secrets.
  - `prompts.rs` centralizes reusable interactive prompt helpers.
  - `provider_select.rs` owns provider and commit-format selection.
  - `model_select.rs` owns model discovery and manual fallback.
  - `theme_select.rs` owns built-in/custom theme selection and RGB theme creation.
## Configuration

- `src/config/schema.rs` defines TOML schema types only.
- `src/config/provider.rs` defines provider defaults, known provider names, and API-key lookup priority.
- `src/config/paths.rs` resolves config file locations.
- `src/config/store.rs` exposes the `AppConfig` API used by the rest of the app.

Provider-specific API keys intentionally take priority over `AI_API_KEY` so switching providers cannot silently reuse a key from another vendor.

## AI Providers

`src/services/ai/` keeps provider protocol logic isolated:

- `openai_compatible.rs` handles OpenAI-compatible chat completions and model listing.
- `anthropic.rs` handles Anthropic messages and model listing.
- `gemini.rs` handles Gemini generateContent and model listing.
- `ollama.rs` handles local Ollama generation and tags.
- `messages.rs` contains shared message-composition helpers.

Adding a provider should usually mean adding one provider module plus a small dispatch entry in `src/services/ai/mod.rs`.

## Theme System

- `src/theme/palette.rs` defines RGB values, built-in palettes, and RGB parsing.
- `src/theme/detect.rs` detects system dark/light preference.
- `src/theme/style.rs` applies the active palette to terminal output.
- `src/utils/colors.rs` is only a compatibility re-export for older imports.

## Error Handling

Top-level commands should return `AppResult<T>` and let `src/main.rs` render user-facing errors. Do not print raw debug structs or provider JSON bodies. Provider HTTP failures must go through `AppError::user_message()` so auth failures, rate limits, and server errors stay readable and sanitized.
