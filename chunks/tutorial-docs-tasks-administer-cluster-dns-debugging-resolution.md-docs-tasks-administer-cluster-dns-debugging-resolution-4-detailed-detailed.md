---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#4-detailed
chunk_level: detailed
chunk_type: code
heading: Before you begin
token_count: 949
summary: ### Are DNS endpoints exposed? You can verify that DNS endpoints are exposed by using the `kubectl get endpointslice` command. ``` `kubectl get endpointslice -l kubernetes.io/service-name=kube-dns...
---

### Are DNS endpoints exposed?
You can verify that DNS endpoints are exposed by using the `kubectl get endpointslice`
command.
```
`kubectl get endpointslice -l kubernetes.io/service-name=kube-dns --namespace=kube-system
`
```
```
`NAME ADDRESSTYPE PORTS ENDPOINTS AGE
kube-dns-zxoja IPv4 53 10.180.3.17,10.180.3.17 1h
`
```
If you do not see the endpoints, see the endpoints section in the
[debugging Services](/docs/tasks/debug/debug-application/debug-service/) documentation.
### Are DNS queries being received/processed?
You can verify if queries are being received by CoreDNS by adding the `log` plugin to the CoreDNS configuration (aka Corefile).
The CoreDNS Corefile is held in a [ConfigMap](/docs/concepts/configuration/configmap/) named `coredns`. To edit it, use the command:
```
`kubectl -n kube-system edit configmap coredns
`
```
Then add `log` in the Corefile section per the example below:
```
`apiVersion: v1
kind: ConfigMap
metadata:
name: coredns
namespace: kube-system
data:
Corefile: |
.:53 {
log
errors
health
kubernetes cluster.local in-addr.arpa ip6.arpa {
pods insecure
upstream
fallthrough in-addr.arpa ip6.arpa
}
prometheus :9153
forward . /etc/resolv.conf
cache 30
loop
reload
loadbalance
}
`
```
After saving the changes, it may take up to minute or two for Kubernetes to propagate these changes to the CoreDNS pods.
Next, make some queries and view the logs per the sections above in this document. If CoreDNS pods are receiving the queries, you should see them in the logs.
Here is an example of a query in the log:
```
`.:53
2018/08/15 14:37:15 [INFO] CoreDNS-1.2.0
2018/08/15 14:37:15 [INFO] linux/amd64, go1.10.3, 2e322f6
CoreDNS-1.2.0
linux/amd64, go1.10.3, 2e322f6
2018/09/07 15:29:04 [INFO] plugin/reload: Running configuration MD5 = 162475cdf272d8aa601e6fe67a6ad42f
2018/09/07 15:29:04 [INFO] Reloading complete
172.17.0.18:41675 - [07/Sep/2018:15:29:11 +0000] 59925 "A IN kubernetes.default.svc.cluster.local. udp 54 false 512" NOERROR qr,aa,rd,ra 106 0.000066649s
`
```
### Does CoreDNS have sufficient permissions?
CoreDNS must be able to list [service](/docs/concepts/services-networking/service/) and [endpointslice](/docs/concepts/services-networking/endpoint-slices/) related resources to properly resolve service names.
Sample error message:
```
`2022-03-18T07:12:15.699431183Z [INFO] 10.96.144.227:52299 - 3686 "A IN serverproxy.contoso.net.cluster.local. udp 52 false 512" SERVFAIL qr,aa,rd 145 0.000091221s
`
```
First, get the current ClusterRole of `system:coredns`:
```
`kubectl describe clusterrole system:coredns -n kube-system
`
```
Expected output:
```
`PolicyRule:
Resources Non-Resource URLs Resource Names Verbs
--------- ----------------- -------------- -----
endpoints [] [] [list watch]
namespaces [] [] [list watch]
pods [] [] [list watch]
services [] [] [list watch]
endpointslices.discovery.k8s.io [] [] [list watch]
`
```
If any permissions are missing, edit the ClusterRole to add them:
```
`kubectl edit clusterrole system:coredns -n kube-system
`
```
Example insertion of EndpointSlices permissions:
```
`...
- apiGroups:
- discovery.k8s.io
resources:
- endpointslices
verbs:
- list
- watch
...
`
```