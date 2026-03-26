---
doc_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec
chunk_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec#1-standard
chunk_level: standard
chunk_type: prose
heading: Examples
token_count: 349
summary: ## Synopsis Execute a command in a container. ``` `kubectl exec (POD | TYPE/NAME) [-c CONTAINER] [flags] -- COMMAND [args...] ` ``` ## Examples ``` ` # Get output from running the 'date' command from...
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