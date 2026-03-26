---
doc_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint
chunk_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint#11-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 93
summary: # Remove from node 'foo' all the taints with key 'dedicated' kubectl taint nodes foo dedicated- # Add a taint with key 'dedicated' on nodes having label myLabel=X kubectl taint node -l myLabel=X...
---

# Remove from node 'foo' all the taints with key 'dedicated'
kubectl taint nodes foo dedicated-
# Add a taint with key 'dedicated' on nodes having label myLabel=X
kubectl taint node -l myLabel=X dedicated=foo:PreferNoSchedule
# Add to node 'foo' a taint with key 'bar' and no value
kubectl taint nodes foo bar:NoSchedule
`
```