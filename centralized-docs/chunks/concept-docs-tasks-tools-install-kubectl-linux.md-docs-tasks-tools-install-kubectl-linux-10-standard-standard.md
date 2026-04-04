---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#10-standard
chunk_level: standard
chunk_type: prose
heading: Verify kubectl configuration
token_count: 330
summary: #### Note: bash-completion sources all completion scripts in `/etc/bash\_completion.d`. Both approaches are equivalent. After reloading your shell, kubectl autocompletion should be working. To enable...
---

#### Note:
bash-completion sources all completion scripts in `/etc/bash\_completion.d`.
Both approaches are equivalent. After reloading your shell, kubectl autocompletion should be working.
To enable bash autocompletion in current session of shell, source the \~/.bashrc file:
```
`source \~/.bashrc
`
```
#### Note:
Autocomplete for Fish requires kubectl 1.23 or later.
The kubectl completion script for Fish can be generated with the command `kubectl completion fish`. Sourcing the completion script in your shell enables kubectl autocompletion.
To do so in all your shell sessions, add the following line to your `\~/.config/fish/config.fish` file:
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
If you have an alias for kubectl, kubectl autocompletion will automatically work with it.
After reloading your shell, kubectl autocompletion should be working.
If you get an error like `2: command not found: compdef`, then add the following to the beginning of your `\~/.zshrc` file:
```
`autoload -Uz compinit
compinit
`
```