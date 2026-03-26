---
doc_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec
chunk_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec#9-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 123
summary: kubectl exec mypod -c ruby-container -i -t -- bash -il # List contents of /usr from the first container of pod mypod and sort by modification time # If the command you want to execute in the pod has...
---

kubectl exec mypod -c ruby-container -i -t -- bash -il
# List contents of /usr from the first container of pod mypod and sort by modification time
# If the command you want to execute in the pod has any flags in common (e.g. -i),
# you must use two dashes (--) to separate your command's flags/arguments
# Also note, do not surround your command and its flags/arguments with quotes
# unless that is how you would execute it normally (i.e., do ls -t /usr, not "ls -t /usr")