---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#22-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 92
summary: #### Note: The `kubectl get svc` command will only show the primary IP in the `CLUSTER-IP` field. ``` `kubectl get svc -l app.kubernetes.io/name=MyApp ` ``` ``` `NAME TYPE CLUSTER-IP EXTERNAL-IP...
---

#### Note:
The `kubectl get svc` command will only show the primary IP in the `CLUSTER-IP` field.
```
`kubectl get svc -l app.kubernetes.io/name=MyApp
`
```
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
my-service ClusterIP 10.0.216.242 &lt;none&gt; 80/TCP 5s
`
```