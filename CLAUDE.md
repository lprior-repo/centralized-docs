# Developer Instructions

## Core Engineering Rules & Development Lifecycle

*These are absolute guarantees. The system MUST ALWAYS do these and MUST NEVER violate them.*

```jsonl
{"rule": "CODEBASE_LOCATION", "mandatory": true, "description": "Code lives in the current working directory. Always operate relative to this root."}
{"rule": "WORKSPACE_ISOLATION", "mandatory": true, "description": "ALWAYS use Jujutsu (jj) for workspace isolation. Create a new jj workspace/bookmark for every task."}
{"rule": "RUST_CONTRACTS", "mandatory": true, "description": "You MUST ALWAYS invoke the `rust-contract` skill to specify the Rust contract before coding."}
{"rule": "FUNCTIONAL_RUST", "mandatory": true, "description": "You MUST ALWAYS invoke the `functional-rust` skill to implement. Data->Calc->Actions, zero panics/unwrap/mut, clippy-flawless."}
{"rule": "COMBATIVE_TESTING", "mandatory": true, "description": "Always write combative tests. Tests MUST compile."}
{"rule": "TOOLING", "mandatory": true, "description": "Always use `moon`, NEVER `cargo`. 100% moon CICD run."}
{"rule": "QA_REVIEW", "mandatory": true, "description": "You MUST ALWAYS invoke the `qa-enforcer` skill for review, adversarial testing, and strict validation."}
{"rule": "QUALITY_GATES", "mandatory": true, "description": "ALL tests, linting, and code MUST compile. Fix ALL issues, even if they are not from your changes."}
{"rule": "NO_MIGRATIONS", "mandatory": true, "description": "Migrations don't exist."}

{"phase": "1_RESEARCH", "transition_condition": "Understand constraints and codebase context."}
{"phase": "2_PLAN", "transition_condition": "Draft a deterministic plan."}
{"phase": "3_CONTRACT_REVIEW", "transition_condition": "Invoke `rust-contract` skill. Output contract-spec.md and martin-fowler-tests.md."}
{"phase": "4_IMPLEMENTATION", "transition_condition": "Use `jj` to isolate. Invoke `functional-rust` skill. Implement via TDD."}
{"phase": "5_VERIFICATION", "transition_condition": "Verify implementation against the contract spec."}
{"phase": "6_QA_ENFORCEMENT", "transition_condition": "Invoke `qa-enforcer` skill. Fix all found issues."}
{"phase": "7_MOON_CICD", "transition_condition": "Execute 100% moon CICD run. Must pass completely."}
{"phase": "8_MERGE", "transition_condition": "Merge it IF AND ONLY IF all of the above phases are true and verified."}
```
