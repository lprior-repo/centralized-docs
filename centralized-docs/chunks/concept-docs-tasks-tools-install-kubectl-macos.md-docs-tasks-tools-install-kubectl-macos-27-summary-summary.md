---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#27-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 104
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