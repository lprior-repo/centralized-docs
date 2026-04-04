---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#38-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 121
summary: bash-completion is provided by many package managers (see [here](https://github.com/scop/bash-completion#installation)). You can install it with `apt-get install bash-completion` or `yum install...
---

bash-completion is provided by many package managers
(see [here](https://github.com/scop/bash-completion#installation)).
You can install it with `apt-get install bash-completion` or `yum install bash-completion`, etc.
The above commands create `/usr/share/bash-completion/bash\_completion`,
which is the main script of bash-completion. Depending on your package manager,
you have to manually source this file in your `\~/.bashrc` file.
To find out, reload your shell and run `type \_init\_completion`.
If the command succeeds, you'