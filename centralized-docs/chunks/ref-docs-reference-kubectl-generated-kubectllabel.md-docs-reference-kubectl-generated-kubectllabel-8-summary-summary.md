---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#8-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 77
summary: kubectl label -f pod.json status=unhealthy # Update pod 'foo' only if the resource is unchanged from version 1 kubectl label pods foo status=unhealthy --resource-version=1 # Update pod 'foo' by...
---

kubectl label -f pod.json status=unhealthy
# Update pod 'foo' only if the resource is unchanged from version 1
kubectl label pods foo status=unhealthy --resource-version=1
# Update pod 'foo' by removing a label named 'bar' if it exists
# Does not require the --overwrite flag
kubectl label pods foo bar-
`
```