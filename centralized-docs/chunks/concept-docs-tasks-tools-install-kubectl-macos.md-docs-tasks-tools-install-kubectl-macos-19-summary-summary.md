---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#19-summary
chunk_level: summary
chunk_type: prose
heading: Verify kubectl configuration
token_count: 86
summary: ``` `kubectl cluster-info ` ``` If you see a URL response, kubectl is correctly configured to access your cluster. If you see a message similar to the following, kubectl is not configured correctly...
---

```
`kubectl cluster-info
`
```
If you see a URL response, kubectl is correctly configured to access your cluster.
If you see a message similar to the following, kubectl is not configured correctly
or is not able to connect to a Kubernetes cluster.
```
`The connection to the server &lt;server-name:port&gt; was refused - did you specify the right host or port?
`
```