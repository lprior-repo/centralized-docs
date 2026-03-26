---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#9-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 123
summary: # Start a hazelcast pod and set labels \"app=hazelcast\" and \"env=prod\" in the container kubectl run hazelcast --image=hazelcast/hazelcast --labels=\"app=hazelcast,env=prod\" # Dry run; print the...
---

# Start a hazelcast pod and set labels "app=hazelcast" and "env=prod" in the container
kubectl run hazelcast --image=hazelcast/hazelcast --labels="app=hazelcast,env=prod"
# Dry run; print the corresponding API objects without creating them
kubectl run nginx --image=nginx --dry-run=client
# Start a nginx pod, but overload the spec with a partial set of values parsed from JSON
kubectl run nginx --image=nginx --overrides='{ "apiVersion": "v1", "spec": { ... } }'