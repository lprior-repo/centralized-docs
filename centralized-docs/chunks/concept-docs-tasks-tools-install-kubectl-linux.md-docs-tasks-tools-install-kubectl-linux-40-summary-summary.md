---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#40-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 120
summary: #### Bash You now need to ensure that the kubectl completion script gets sourced in all your shell sessions. There are two ways in which you can do this: ``` ` echo 'source &lt;(kubectl completion...
---

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