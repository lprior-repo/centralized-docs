---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#31-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 88
summary: #### Note: Autocomplete for Fish requires kubectl 1.23 or later. The kubectl completion script for Fish can be generated with the command `kubectl completion fish`. Sourcing the completion script in...
---

#### Note:
Autocomplete for Fish requires kubectl 1.23 or later.
The kubectl completion script for Fish can be generated with the command `kubectl completion fish`. Sourcing the completion script in your shell enables kubectl autocompletion.
To do so in all your shell sessions, add the following line to your `\~/.config/fish/config.fish` file:
```
`kubectl completion fish | source
`
```