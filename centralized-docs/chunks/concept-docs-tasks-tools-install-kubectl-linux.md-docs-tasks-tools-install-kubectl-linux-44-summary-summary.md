---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#44-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 98
summary: ``` `kubectl completion fish | source ` ``` After reloading your shell, kubectl autocompletion should be working. The kubectl completion script for Zsh can be generated with the command `kubectl...
---

```
`kubectl completion fish | source
`
```
After reloading your shell, kubectl autocompletion should be working.
The kubectl completion script for Zsh can be generated with the command `kubectl completion zsh`. Sourcing the completion script in your shell enables kubectl autocompletion.
To do so in all your shell sessions, add the following to your `\~/.zshrc` file:
```
`source &lt;(kubectl completion zsh)
`
```