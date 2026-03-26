---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#8-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 112
summary: ` # Start a nginx pod kubectl run nginx --image=nginx # Start a hazelcast pod and let the container expose port 5701 kubectl run hazelcast --image=hazelcast/hazelcast --port=5701 # Start a hazelcast...
---

` # Start a nginx pod
kubectl run nginx --image=nginx
# Start a hazelcast pod and let the container expose port 5701
kubectl run hazelcast --image=hazelcast/hazelcast --port=5701
# Start a hazelcast pod and set environment variables "DNS\_DOMAIN=cluster" and "POD\_NAMESPACE=default" in the container
kubectl run hazelcast --image=hazelcast/hazelcast --env="DNS\_DOMAIN=cluster" --env="POD\_NAMESPACE=default"