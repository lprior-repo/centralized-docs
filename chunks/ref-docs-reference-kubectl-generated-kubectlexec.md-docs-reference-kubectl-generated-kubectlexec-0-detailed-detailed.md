---
doc_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec
chunk_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec#0-detailed
chunk_level: detailed
chunk_type: table
heading: Options
token_count: 1022
summary: ## Table of Contents    - [Synopsis](#synopsis)   - [Examples](#examples) - [Get output from running the 'date' command in ruby-container from pod...
---

## Table of Contents

  - [Synopsis](#synopsis)
  - [Examples](#examples)
- [Get output from running the 'date' command in ruby-container from pod mypod](#get-output-from-running-the-date-command-in-ruby-container-from-pod-mypod)
- [Switch to raw terminal mode; sends stdin to 'bash' in ruby-container from pod mypod](#switch-to-raw-terminal-mode-sends-stdin-to-bash-in-ruby-container-from-pod-mypod)
- [and sends stdout/stderr from 'bash' back to the client](#and-sends-stdoutstderr-from-bash-back-to-the-client)
- [List contents of /usr from the first container of pod mypod and sort by modification time](#list-contents-of-usr-from-the-first-container-of-pod-mypod-and-sort-by-modification-time)
- [If the command you want to execute in the pod has any flags in common (e.g. -i),](#if-the-command-you-want-to-execute-in-the-pod-has-any-flags-in-common-eg--i)
- [you must use two dashes (--) to separate your command's flags/arguments](#you-must-use-two-dashes----to-separate-your-commands-flagsarguments)
- [Also note, do not surround your command and its flags/arguments with quotes](#also-note-do-not-surround-your-command-and-its-flagsarguments-with-quotes)
- [unless that is how you would execute it normally (i.e., do ls -t /usr, not "ls -t /usr")](#unless-that-is-how-you-would-execute-it-normally-ie-do-ls--t-usr-not-ls--t-usr)
- [Get output from running 'date' command from the first pod of the deployment mydeployment, using the first container by default](#get-output-from-running-date-command-from-the-first-pod-of-the-deployment-mydeployment-using-the-first-container-by-default)
- [Get output from running 'date' command from the first pod of the service myservice, using the first container by default](#get-output-from-running-date-command-from-the-first-pod-of-the-service-myservice-using-the-first-container-by-default)
  - [Options](#options)
  - [Parent Options Inherited](#parent-options-inherited)
  - [Feedback](#feedback)

---

## Synopsis
Execute a command in a container.
```
`kubectl exec (POD | TYPE/NAME) [-c CONTAINER] [flags] -- COMMAND [args...]
`
```
## Examples
```
` # Get output from running the 'date' command from pod mypod, using the first container by default
kubectl exec mypod -- date
# Get output from running the 'date' command in ruby-container from pod mypod
kubectl exec mypod -c ruby-container -- date
# Switch to raw terminal mode; sends stdin to 'bash' in ruby-container from pod mypod
# and sends stdout/stderr from 'bash' back to the client
kubectl exec mypod -c ruby-container -i -t -- bash -il
# List contents of /usr from the first container of pod mypod and sort by modification time
# If the command you want to execute in the pod has any flags in common (e.g. -i),
# you must use two dashes (--) to separate your command's flags/arguments
# Also note, do not surround your command and its flags/arguments with quotes
# unless that is how you would execute it normally (i.e., do ls -t /usr, not "ls -t /usr")
kubectl exec mypod -i -t -- ls -t /usr
# Get output from running 'date' command from the first pod of the deployment mydeployment, using the first container by default
kubectl exec deploy/mydeployment -- date
# Get output from running 'date' command from the first pod of the service myservice, using the first container by default
kubectl exec svc/myservice -- date
`
```
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