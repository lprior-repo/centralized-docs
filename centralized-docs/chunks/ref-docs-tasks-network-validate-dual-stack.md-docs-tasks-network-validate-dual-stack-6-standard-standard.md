---
doc_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack
chunk_id: ref/docs-tasks-network-validate-dual-stack.md/docs-tasks-network-validate-dual-stack#6-standard
chunk_level: standard
chunk_type: prose
heading: Validate Services
token_count: 294
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
Validate that the Service gets cluster IPs from the IPv4 and IPv6 address blocks using
`kubectl describe`. You may then validate access to the service via the IPs and ports.
```
`kubectl describe svc -l app.kubernetes.io/name=MyApp
`
```
```
`Name: my-service
Namespace: default
Labels: app.kubernetes.io/name=MyApp
Annotations: &lt;none&gt;
Selector: app.kubernetes.io/name=MyApp
Type: ClusterIP
IP Family Policy: PreferDualStack
IP Families: IPv4,IPv6
IP: 10.0.216.242
IPs: 10.0.216.242,2001:db8:fd00::af55
Port: &lt;unset&gt; 80/TCP
TargetPort: 9376/TCP
Endpoints: &lt;none&gt;
Session Affinity: None
Events: &lt;none&gt;
`
```