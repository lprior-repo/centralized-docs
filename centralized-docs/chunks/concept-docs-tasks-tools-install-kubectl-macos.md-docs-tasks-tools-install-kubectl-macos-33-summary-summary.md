---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#33-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 98
summary: ``` `source &lt;(kubectl completion zsh) ` ``` If you have an alias for kubectl, kubectl autocompletion will automatically work with it. After reloading your shell, kubectl autocompletion should be...
---

```
`source &lt;(kubectl completion zsh)
`
```
If you have an alias for kubectl, kubectl autocompletion will automatically work with it.
After reloading your shell, kubectl autocompletion should be working.
If you get an error like `2: command not found: compdef`, then add the following to the beginning of your `\~/.zshrc` file:
```
`autoload -Uz compinit
compinit
`
```