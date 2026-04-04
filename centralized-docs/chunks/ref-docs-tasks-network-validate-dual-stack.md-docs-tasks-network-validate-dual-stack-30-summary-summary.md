---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#30-summary
chunk_level: summary
chunk_type: prose
heading: Validate Services
token_count: 124
summary: Check the Service: ``` `kubectl get svc -l app.kubernetes.io/name=MyApp ` ``` Validate that the Service receives a `CLUSTER-IP` address from the IPv6 address block along with an `EXTERNAL-IP`. You...
---

Check the Service:
```
`kubectl get svc -l app.kubernetes.io/name=MyApp
`
```
Validate that the Service receives a `CLUSTER-IP` address from the IPv6 address block
along with an `EXTERNAL-IP`. You may then validate access to the service via the IP and port.
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
my-service LoadBalancer 2001:db8:fd00::7ebc 2603:1030:805::5 80:30790/TCP 35s
`
```