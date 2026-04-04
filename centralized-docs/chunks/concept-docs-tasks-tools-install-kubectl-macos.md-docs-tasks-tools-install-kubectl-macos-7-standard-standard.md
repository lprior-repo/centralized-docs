---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#7-standard
chunk_level: standard
chunk_type: prose
heading: Verify kubectl configuration
token_count: 318
summary: #### Note: The Homebrew installation of bash-completion v2 sources all the files in the `BASH\_COMPLETION\_COMPAT\_DIR` directory, that's why the latter two methods work. In any case, after reloading...
---

#### Note:
The Homebrew installation of bash-completion v2 sources all the files in the
`BASH\_COMPLETION\_COMPAT\_DIR` directory, that's why the latter two methods work.
In any case, after reloading your shell, kubectl completion should be working.
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