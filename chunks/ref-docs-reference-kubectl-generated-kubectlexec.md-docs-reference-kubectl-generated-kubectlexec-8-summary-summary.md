---
doc_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec
chunk_id: ref/docs-reference-kubectl-generated-kubectlexec.md/docs-reference-kubectl-generated-kubectlexec#8-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 111
summary: ` # Get output from running the 'date' command from pod mypod, using the first container by default kubectl exec mypod -- date # Get output from running the 'date' command in ruby-container from pod...
---

` # Get output from running the 'date' command from pod mypod, using the first container by default
kubectl exec mypod -- date
# Get output from running the 'date' command in ruby-container from pod mypod
kubectl exec mypod -c ruby-container -- date
# Switch to raw terminal mode; sends stdin to 'bash' in ruby-container from pod mypod
# and sends stdout/stderr from 'bash' back to the client
kubectl exec mypod -c ruby-container -i -t -- bash -il