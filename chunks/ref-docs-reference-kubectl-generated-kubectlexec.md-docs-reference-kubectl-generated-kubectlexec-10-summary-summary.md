---
doc_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec
chunk_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec#10-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 115
summary: # unless that is how you would execute it normally (i.e., do ls -t /usr, not \"ls -t /usr\") kubectl exec mypod -i -t -- ls -t /usr # Get output from running 'date' command from the first pod of the...
---

# unless that is how you would execute it normally (i.e., do ls -t /usr, not "ls -t /usr")
kubectl exec mypod -i -t -- ls -t /usr
# Get output from running 'date' command from the first pod of the deployment mydeployment, using the first container by default
kubectl exec deploy/mydeployment -- date
# Get output from running 'date' command from the first pod of the service myservice, using the first container by default
kubectl exec svc/myservice -- date
`
```