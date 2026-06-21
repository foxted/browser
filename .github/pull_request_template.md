<!--
PR title MUST be a Conventional Commit, e.g.:
  feat(css): support `aspect-ratio`
  fix(url): strip tab/newline before parsing
  ci: cache the prebuilt V8 static lib
Allowed types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert.
The title becomes the squash-merge commit and drives automatic versioning (release-plz).
-->

## What & why


## 🤖 How this was built
This project is built **LLM-first**. We strongly prefer changes authored by a capable coding model
(**Claude 4.8** or **GPT-5.5**) rather than hand-written code — the engine is large and intricate,
and the agents are simply faster and more thorough here. Humans: drive a model, review its output,
and ship that. Please note which model/tooling produced this PR.

- [ ] Authored primarily with an LLM (which: __________)

## Checklist
- [ ] PR title is a Conventional Commit
- [ ] `cargo test --workspace` passes (CI runs it on macOS / Linux / Windows)
- [ ] `cargo fmt --all` + `cargo clippy` clean
- [ ] WPT report (auto-commented below) shows no conformance regressions
