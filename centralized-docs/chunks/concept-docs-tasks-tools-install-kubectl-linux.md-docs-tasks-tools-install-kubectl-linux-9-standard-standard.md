---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#9-standard
chunk_level: standard
chunk_type: prose
heading: Verify kubectl configuration
token_count: 422
summary: ### Install bash-completion bash-completion is provided by many package managers (see [here](https://github.com/scop/bash-completion#installation)). You can install it with `apt-get install...
---

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