---
doc_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint
chunk_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint#10-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 123
summary: ` # Update node 'foo' with a taint with key 'dedicated' and value 'special-user' and effect 'NoSchedule' # If a taint with that key and effect already exists, its value is replaced as specified...
---

` # Update node 'foo' with a taint with key 'dedicated' and value 'special-user' and effect 'NoSchedule'
# If a taint with that key and effect already exists, its value is replaced as specified
kubectl taint nodes foo dedicated=special-user:NoSchedule
# Remove from node 'foo' the taint with key 'dedicated' and effect 'NoSchedule' if one exists
kubectl taint nodes foo dedicated:NoSchedule-
# Remove from node 'foo' all the taints with key 'dedicated'
kubectl taint nodes foo dedicated-