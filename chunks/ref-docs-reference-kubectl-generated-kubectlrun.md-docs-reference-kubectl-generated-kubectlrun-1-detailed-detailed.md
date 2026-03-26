---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Examples
token_count: 454
summary: # kubectl run Run a particular image on the cluster ## Synopsis Create and run a particular image in a pod. ``` `kubectl run NAME --image=image [--env=\"key=value\"] [--port=port]...
---

# kubectl run
Run a particular image on the cluster
## Synopsis
Create and run a particular image in a pod.
```
`kubectl run NAME --image=image [--env="key=value"] [--port=port] [--dry-run=server|client] [--overrides=inline-json] [--command] -- [COMMAND] [args...]
`
```
## Examples
```
` # Start a nginx pod
kubectl run nginx --image=nginx
# Start a hazelcast pod and let the container expose port 5701
kubectl run hazelcast --image=hazelcast/hazelcast --port=5701
# Start a hazelcast pod and set environment variables "DNS\_DOMAIN=cluster" and "POD\_NAMESPACE=default" in the container
kubectl run hazelcast --image=hazelcast/hazelcast --env="DNS\_DOMAIN=cluster" --env="POD\_NAMESPACE=default"
# Start a hazelcast pod and set labels "app=hazelcast" and "env=prod" in the container
kubectl run hazelcast --image=hazelcast/hazelcast --labels="app=hazelcast,env=prod"
# Dry run; print the corresponding API objects without creating them
kubectl run nginx --image=nginx --dry-run=client
# Start a nginx pod, but overload the spec with a partial set of values parsed from JSON
kubectl run nginx --image=nginx --overrides='{ "apiVersion": "v1", "spec": { ... } }'
# Start a busybox pod and keep it in the foreground, don't restart it if it exits
kubectl run -i -t busybox --image=busybox --restart=Never
# Start the nginx pod using the default command, but use custom arguments (arg1 .. argN) for that command
kubectl run nginx --image=nginx -- &lt;arg1&gt; &lt;arg2&gt; ... &lt;argN&gt;
# Start the nginx pod using a different command and custom arguments
kubectl run nginx --image=nginx --command -- &lt;cmd&gt; &lt;arg1&gt; ... &lt;argN&gt;
`
```