---
doc_id: meta/contributing.md/contributing
chunk_id: meta/contributing.md/contributing#2-standard
chunk_level: standard
chunk_type: prose
heading: Development Workflow
token_count: 132
summary: # Contributing. ## Ways to Contribute
---

# Contributing


## Ways to Contribute

1. **Submit Examples**: Add `llms.txt` examples from popular projects to the `examples/` directory. Ensure they pass the validator.
2. **Report Bugs**: Use GitHub Issues to report bugs with steps to reproduce.
3. **Propose RFC Changes**: Open a GitHub Discussion for proposed changes.
4. **Improve Tools**: Contribute to the validator (`doc_transformer`) or parser (`llms-txt-parser`).
5. **Documentation**: Help improve existing docs or translations.

## Development Workflow

- Fork the repository.
- Create a feature branch.
- Make changes following Rust guidelines.
- Test your changes: `moon run :test`, `:clippy`, `:fmt`.
- Submit a PR.

