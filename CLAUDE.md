# Developer & Agent Instructions

## 0. THE PRIME DIRECTIVE (SCIENTIFIC RIGOR & HONESTY)
*   **SKEPTICISM OVER PLEASING**: You are not here to make the user happy. You are here to write scientifically correct, accurate, and rigorous code. Be skeptical and critical towards the user and the code you write at all times.
*   **NEVER LIE. NEVER HALLUCINATE.** If you do not know something, or if a command fails, you MUST state explicitly that you are unsure or blocked. 
*   **Extreme Engineering Rigor**: All orders below are absolutely mandatory. You will not deviate ever.

## 1. Core Engineering Rules & Development Lifecycle

*These are absolute guarantees. The system MUST ALWAYS do these and MUST NEVER violate them.*

```jsonl
{"rule": "SCIENTIFIC_RIGOR", "mandatory": true, "description": "Do not aim to please. Write scientifically correct code. Be highly critical of the user and yourself. Never lie or hallucinate."}
{"rule": "CODEBASE_LOCATION", "mandatory": true, "description": "Code lives in the current working directory."}
{"rule": "WORKSPACE_ISOLATION", "mandatory": true, "description": "Git is BANNED. ALWAYS use Jujutsu (`jj new`) for isolation. NEVER run `git` commands."}
{"rule": "DESTRUCTIVE_OPS_BANNED", "mandatory": true, "description": "`rm -rf`, `git reset --hard`, and `git rebase --hard` are STRICTLY BANNED. You MUST ask explicitly before any destructive operation."}
{"rule": "DDD_ARCHITECTURE", "mandatory": true, "description": "Use Domain-Driven Design. Invoke `scott-ddd-refactor` skill. Make illegal states unrepresentable."}
{"rule": "RUST_CONTRACTS", "mandatory": true, "description": "ALWAYS invoke `rust-contract` BEFORE coding to specify preconditions, postconditions, and errors."}
{"rule": "FUNCTIONAL_RUST", "mandatory": true, "description": "Invoke `functional-rust-generator`. Data->Calc->Actions. Zero panics/unwrap/mut. Extreme DRY."}
{"rule": "COMBATIVE_TESTING", "mandatory": true, "description": "Invoke `red-queen` & `qa-enforcer`. Drive adversarial tests into every change. Execute dynamically."}
{"rule": "TOOLING", "mandatory": true, "description": "Raw Cargo commands and Clippy are STRICTLY BANNED. ALWAYS use `moon` CICD commands (e.g., `moon run :ci`). Flawless source, test code quality is irrelevant."}
{"rule": "NO_MIGRATIONS", "mandatory": true, "description": "Migrations don't exist. Use idempotent schema initialization (IF NOT EXISTS)."}
{"rule": "ISSUE_TRACKING", "mandatory": true, "description": "Use Beads (`bd`) exclusively for issue tracking. No markdown TODOs. Invoke `planner` skill to create/decompose new beads via CUE schema."}
{"rule": "GO_SKILL_WORKFLOW", "mandatory": true, "description": "When picking up a bead, ALWAYS invoke `go-skill` to execute the full BRCLI-first lifecycle: pick bead, isolate, implement, QA, CI, rebase, PR, cleanup."}
{"rule": "LANDING_SKILL", "mandatory": true, "description": "When ending a session, ALWAYS invoke `landing-skill` to audit, file beads, run quality gates, rebase onto main, verify, close beads, and cleanup workspaces."}

{"phase": "1_RESEARCH", "transition_condition": "Understand constraints and DDD context skeptically."}
{"phase": "2_PLAN", "transition_condition": "Draft deterministic plan. Decompose via `planner` skill if creating beads."}
{"phase": "3_CONTRACT_REVIEW", "transition_condition": "Invoke `rust-contract`. Output contract-spec.md and martin-fowler-tests.md."}
{"phase": "4_IMPLEMENTATION", "transition_condition": "Isolate via `jj new`. Invoke `functional-rust-generator` and `scott-ddd-refactor`. TDD."}
{"phase": "5_VERIFICATION", "transition_condition": "Verify against contract spec using `moon`."}
{"phase": "6_QA_ENFORCEMENT", "transition_condition": "Invoke `qa-enforcer` & `red-queen`. Break boundaries. Fix all found issues."}
{"phase": "7_MOON_CICD", "transition_condition": "Execute 100% `moon` CICD run. Must pass flawlessly. NEVER run cargo directly."}
{"rule": "FAILURE_GOES_BACK_TO_IMPLEMENTATION", "mandatory": true, "description": "If ANY issues are found in phases 5, 6, or 7, you MUST go back to phase 4 (implementation). NEVER skip or proceed past failures."}
{"phase": "8_LANDING", "transition_condition": "Push to main via `jj git fetch && jj rebase -d main && bd sync && jj git push` IF AND ONLY IF healthy."}
```

## 2. Functional Rust & DDD (Deep Doctrine)

```jsonl
{"principle": "FUNCTIONAL_CORE_IMPERATIVE_SHELL", "description": "Data (inert enums/structs) -> Calculations (pure functions returning Result) -> Actions (minimal I/O at absolute boundaries)."}
{"principle": "ZERO_PANICS_LAW", "description": "unwrap, expect, and panic! are STRICTLY FORBIDDEN. Handle all errors explicitly."}
{"principle": "FUNCTIONAL_PRIMITIVES", "description": "No mut. No for/while loops. Rely entirely on itertools, tap, and persistent data structures (im, rpds)."}
{"principle": "EXTREME_DRY", "description": "Never repeat logic. Extract, modularize, and reuse strictly."}
{"principle": "DDD_MAKE_ILLEGAL_STATES_UNREPRESENTABLE", "description": "Use enums for state machines. Each variant has exactly valid fields."}
{"principle": "DDD_PARSE_DONT_VALIDATE", "description": "Parse at boundary into trusted types. Once parsed, data is always valid."}
```

## 3. Issue Tracking (Beads & Planner)

```jsonl
{"tool": "BD", "description": "Steve Yege's CLI tool for planning and JIRA-like work. Use `bd` exclusively."}
{"cmd": "bd_ready", "description": "Find available work"}
{"cmd": "bd_update_claim", "description": "Claim work atomically: bd update <id> --claim"}
{"cmd": "bd_close", "description": "Complete work: bd close <id> --reason 'Done'"}
{"rule": "NO_MARKDOWN_TODOS", "description": "Do NOT use markdown TODOs or external trackers. Use bd exclusively."}
{"skill": "PLANNER", "description": "When creating NEW issues or decomposing work, MUST invoke planner skill to generate CUE schema beads."}
{"skill": "GO_SKILL", "description": "When picking up a bead, ALWAYS invoke go-skill to execute full lifecycle end-to-end."}
```

## 4. Execution Rules

```jsonl
{"rule": "DESTRUCTIVE_OPS_BANNED", "description": "rm -rf is completely banned. git reset --hard and git rebase --hard are completely banned."}
{"rule": "EXPLICIT_PERMISSION_REQUIRED", "description": "MUST explicitly ask user for permission before deleting files or abandoning state."}
{"rule": "NON_INTERACTIVE", "description": "Use flags to prevent hanging on standard terminal prompts."}
{"rule": "USE_FLAGS", "examples": ["rm -f file", "cp -f source dest", "mv -f source dest", "apt-get -y"]}
```

## 5. Verification Commands (JSONL)

```jsonl
{"cmd": "bd --version", "expect": "version output"}
{"cmd": "bd doctor | grep -E 'passed|failed'", "expect": "passed or failed status"}
{"cmd": "curl -sf http://127.0.0.1:909/restate/health", "expect": "healthy response"}
{"cmd": "ps aux | grep 'oya serve' | grep -v grep", "expect": "oya serve process running"}
{"cmd": "curl -sf http://127.0.0.1:9180/restate/health", "expect": "oya service healthy"}
{"cmd": "bd create 'test bead' --type feature --priority 1 --json", "expect": "bead id in JSON"}
{"cmd": "bd update <id> --claim", "expect": "claim success"}
{"cmd": "./target/release/oya lifecycle --bead <id> --repo <repo>", "expect": "lifecycle started"}
{"cmd": "curl -s http://127.0.0.1:909/OyaService/get_lifecycle -H 'Content-Type: application/json' -d '{\"key\":\"<id>\"}'", "expect": "lifecycle status JSON"}
```
