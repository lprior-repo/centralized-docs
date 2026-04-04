---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#28-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 124
summary: #### Note: As mentioned, these instructions assume you use Bash 4.1+, which means you will install bash-completion v2 (in contrast to Bash 3.2 and bash-completion v1, in which case kubectl completion...
---

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