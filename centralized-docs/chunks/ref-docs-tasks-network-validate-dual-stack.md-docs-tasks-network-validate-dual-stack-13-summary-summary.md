---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#13-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 83
summary: ``` `apiVersion: v1 kind: Service metadata: name: my-service labels: app.kubernetes.io/name: MyApp spec: selector: app.kubernetes.io/name: MyApp ports: - protocol: TCP port: 80 ` ``` Use `kubectl` to...
---

```
`apiVersion: v1
kind: Service
metadata:
name: my-service
labels:
app.kubernetes.io/name: MyApp
spec:
selector:
app.kubernetes.io/name: MyApp
ports:
- protocol: TCP
port: 80
`
```
Use `kubectl` to view the YAML for the Service.
```
`kubectl get svc my-service -o yaml
`
```