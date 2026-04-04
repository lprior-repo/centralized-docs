---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#15-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 115
summary: ``` `apiVersion: v1 kind: Service metadata: name: my-service namespace: default spec: clusterIP: 10.0.217.164 clusterIPs: - 10.0.217.164 ipFamilies: - IPv4 ipFamilyPolicy: SingleStack ports: - port:...
---

```
`apiVersion: v1
kind: Service
metadata:
name: my-service
namespace: default
spec:
clusterIP: 10.0.217.164
clusterIPs:
- 10.0.217.164
ipFamilies:
- IPv4
ipFamilyPolicy: SingleStack
ports:
- port: 80
protocol: TCP
targetPort: 9376
selector:
app.kubernetes.io/name: MyApp
sessionAffinity: None
type: ClusterIP
status:
loadBalancer: {}
`
```