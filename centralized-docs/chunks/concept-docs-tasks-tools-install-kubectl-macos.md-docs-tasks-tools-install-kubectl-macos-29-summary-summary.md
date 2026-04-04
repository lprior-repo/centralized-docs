---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#29-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 101
summary: As stated in the output of this command, add the following to your `\~/.bash\_profile` file: ``` `brew\_etc=\"$(brew --prefix)/etc\" &amp;&amp; [[ -r \"${brew\_etc}/profile.d/bash\_completion.sh\" ]]...
---

As stated in the output of this command, add the following to your `\~/.bash\_profile` file:
```
`brew\_etc="$(brew --prefix)/etc" &amp;&amp; [[ -r "${brew\_etc}/profile.d/bash\_completion.sh" ]] &amp;&amp; . "${brew\_etc}/profile.d/bash\_completion.sh"
`
```
Reload your shell and verify that bash-completion v2 is correctly installed with `type \_init\_completion`.