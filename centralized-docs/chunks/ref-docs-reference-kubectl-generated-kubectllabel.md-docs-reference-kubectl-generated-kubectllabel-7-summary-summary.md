---
doc_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel
chunk_id: ref/docs-reference-kubectl-generated-kubectllabel.md/docs-reference-kubectl-generated-kubectllabel#7-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 123
summary: ` # Update pod 'foo' with the label 'unhealthy' and the value 'true' kubectl label pods foo unhealthy=true # Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any...
---

` # Update pod 'foo' with the label 'unhealthy' and the value 'true'
kubectl label pods foo unhealthy=true
# Update pod 'foo' with the label 'status' and the value 'unhealthy', overwriting any existing value
kubectl label --overwrite pods foo status=unhealthy
# Update all pods in the namespace
kubectl label pods --all status=unhealthy
# Update a pod identified by the type and name in "pod.json"
kubectl label -f pod.json status=unhealthy
# Update pod 'foo' only if the resource is unchanged from version 1