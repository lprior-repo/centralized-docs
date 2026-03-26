---
doc_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec
chunk_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec#2-standard
chunk_level: standard
chunk_type: table
heading: Options
token_count: 169
summary: ## Options |-c, --container string| || Container name. If omitted, use the kubectl.kubernetes.io/default-container annotation for selecting the container to be attached or the first container in the...
---

## Options
|-c, --container string|
||
Container name. If omitted, use the kubectl.kubernetes.io/default-container annotation for selecting the container to be attached or the first container in the pod will be chosen
|
|-f, --filename strings|
||
to use to exec into the resource
|
|-h, --help|
||
help for exec
|
|--pod-running-timeout durationDefault: 1m0s|
||
The length of time (like 5s, 2m, or 3h, higher than zero) to wait until at least one pod is running
|
|-q, --quiet|
||
Only print output from the remote session
|
|-i, --stdin|
||
Pass stdin to the container
|
|-t, --tty|
||
Stdin is a TTY
|