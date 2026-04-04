---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#3-detailed
chunk_level: detailed
chunk_type: code
heading: Verify kubectl configuration
token_count: 624
summary: ### Upgrade Bash The instructions here assume you use Bash 4.1+. You can check your Bash's version by running: ``` `echo $BASH\_VERSION ` ``` If it is too old, you can install/upgrade it using...
---

### Upgrade Bash
The instructions here assume you use Bash 4.1+. You can check your Bash's version by running:
```
`echo $BASH\_VERSION
`
```
If it is too old, you can install/upgrade it using Homebrew:
```
`brew install bash
`
```
Reload your shell and verify that the desired version is being used:
```
`echo $BASH\_VERSION $SHELL
`
```
Homebrew usually installs it at `/usr/local/bin/bash`.
#### Note:
As mentioned, these instructions assume you use Bash 4.1+, which means you will
install bash-completion v2 (in contrast to Bash 3.2 and bash-completion v1,
in which case kubectl completion won't work).
You can test if you have bash-completion v2 already installed with `type \_init\_completion`.
If not, you can install it with Homebrew:
```
`brew install bash-completion@2
`
```
As stated in the output of this command, add the following to your `\~/.bash\_profile` file:
```
`brew\_etc="$(brew --prefix)/etc" &amp;&amp; [[ -r "${brew\_etc}/profile.d/bash\_completion.sh" ]] &amp;&amp; . "${brew\_etc}/profile.d/bash\_completion.sh"
`
```
Reload your shell and verify that bash-completion v2 is correctly installed with `type \_init\_completion`.
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