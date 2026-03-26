---
doc_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun
chunk_id: ref/docs-reference-kubectl-generated-kubectlrun.md/docs-reference-kubectl-generated-kubectlrun#10-summary
chunk_level: summary
chunk_type: prose
heading: Examples
token_count: 120
summary: kubectl run nginx --image=nginx --overrides='{ \"apiVersion\": \"v1\", \"spec\": { ... } }' # Start a busybox pod and keep it in the foreground, don't restart it if it exits kubectl run -i -t busybox...
---

kubectl run nginx --image=nginx --overrides='{ "apiVersion": "v1", "spec": { ... } }'
# Start a busybox pod and keep it in the foreground, don't restart it if it exits
kubectl run -i -t busybox --image=busybox --restart=Never
# Start the nginx pod using the default command, but use custom arguments (arg1 .. argN) for that command
kubectl run nginx --image=nginx -- &lt;arg1&gt; &lt;arg2&gt; ... &lt;argN&gt;