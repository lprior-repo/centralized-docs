---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#29-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 111
summary: ![](/images/copycode.svg \"Copy service/networking/dual-stack-prefer-ipv6-lb-svc.yaml to clipboard\") ``` `apiVersion: v1 kind: Service metadata: name: my-service labels: app.kubernetes.io/name: MyApp...
---

![](/images/copycode.svg "Copy service/networking/dual-stack-prefer-ipv6-lb-svc.yaml to clipboard")
```
`apiVersion: v1
kind: Service
metadata:
name: my-service
labels:
app.kubernetes.io/name: MyApp
spec:
ipFamilyPolicy: PreferDualStack
ipFamilies:
- IPv6
type: LoadBalancer
selector:
app.kubernetes.io/name: MyApp
ports:
- protocol: TCP
port: 80
`
```
Check the Service: