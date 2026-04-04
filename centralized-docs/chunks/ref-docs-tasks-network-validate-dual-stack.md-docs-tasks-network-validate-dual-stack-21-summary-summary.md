---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#21-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 88
summary: ![](/images/copycode.svg \"Copy service/networking/dual-stack-preferred-svc.yaml to clipboard\") ``` `apiVersion: v1 kind: Service metadata: name: my-service labels: app.kubernetes.io/name: MyApp spec:...
---

![](/images/copycode.svg "Copy service/networking/dual-stack-preferred-svc.yaml to clipboard")
```
`apiVersion: v1
kind: Service
metadata:
name: my-service
labels:
app.kubernetes.io/name: MyApp
spec:
ipFamilyPolicy: PreferDualStack
selector:
app.kubernetes.io/name: MyApp
ports:
- protocol: TCP
port: 80
`
```