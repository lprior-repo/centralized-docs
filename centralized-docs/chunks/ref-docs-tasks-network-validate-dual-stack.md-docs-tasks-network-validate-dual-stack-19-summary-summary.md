---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#19-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 128
summary: ``` `apiVersion: v1 kind: Service metadata: labels: app.kubernetes.io/name: MyApp name: my-service spec: clusterIP: 2001:db8:fd00::5118 clusterIPs: - 2001:db8:fd00::5118 ipFamilies: - IPv6...
---

```
`apiVersion: v1
kind: Service
metadata:
labels:
app.kubernetes.io/name: MyApp
name: my-service
spec:
clusterIP: 2001:db8:fd00::5118
clusterIPs:
- 2001:db8:fd00::5118
ipFamilies:
- IPv6
ipFamilyPolicy: SingleStack
ports:
- port: 80
protocol: TCP
targetPort: 80
selector:
app.kubernetes.io/name: MyApp
sessionAffinity: None
type: ClusterIP
status:
loadBalancer: {}
`
```