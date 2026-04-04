---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#5-detailed
chunk_level: detailed
chunk_type: code
heading: Verify kubectl configuration
token_count: 777
summary: ### Introduction The kubectl completion script for Bash can be generated with the command `kubectl completion bash`. Sourcing the completion script in your shell enables kubectl autocompletion....
---

### Introduction
The kubectl completion script for Bash can be generated with the command `kubectl completion bash`.
Sourcing the completion script in your shell enables kubectl autocompletion.
However, the completion script depends on
[**bash-completion**](https://github.com/scop/bash-completion),
which means that you have to install this software first
(you can test if you have bash-completion already installed by running `type \_init\_completion`).
### Install bash-completion
bash-completion is provided by many package managers
(see [here](https://github.com/scop/bash-completion#installation)).
You can install it with `apt-get install bash-completion` or `yum install bash-completion`, etc.
The above commands create `/usr/share/bash-completion/bash\_completion`,
which is the main script of bash-completion. Depending on your package manager,
you have to manually source this file in your `\~/.bashrc` file.
To find out, reload your shell and run `type \_init\_completion`.
If the command succeeds, you're already set, otherwise add the following to your `\~/.bashrc` file:
```
`source /usr/share/bash-completion/bash\_completion
`
```
Reload your shell and verify that bash-completion is correctly installed by typing `type \_init\_completion`.
#### Bash
You now need to ensure that the kubectl completion script gets sourced in all
your shell sessions. There are two ways in which you can do this:
```
`
echo 'source &lt;(kubectl completion bash)' &gt;&gt;\~/.bashrc
`
```
```
`
kubectl completion bash | sudo tee /etc/bash\_completion.d/kubectl &gt; /dev/null
sudo chmod a+r /etc/bash\_completion.d/kubectl
`
```
If you have an alias for kubectl, you can extend shell completion to work with that alias:
```
`echo 'alias k=kubectl' &gt;&gt;\~/.bashrc
echo 'complete -o default -F \_\_start\_kubectl k' &gt;&gt;\~/.bashrc
`
```
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