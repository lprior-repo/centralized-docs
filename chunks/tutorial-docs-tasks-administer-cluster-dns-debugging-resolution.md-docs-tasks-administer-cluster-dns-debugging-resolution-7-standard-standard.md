---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#7-standard
chunk_level: standard
chunk_type: code
heading: Before you begin
token_count: 473
summary: ### Does CoreDNS have sufficient permissions? CoreDNS must be able to list [service](/docs/concepts/services-networking/service/) and...
---

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
### Are you in the right namespace for the service?
DNS queries that don't specify a namespace are limited to the pod's
namespace.
If the namespace of the pod and service differ, the DNS query must include
the namespace of the service.
This query is limited to the pod's namespace:
```
`kubectl exec -i -t dnsutils -- nslookup &lt;service-name&gt;
`
```
This query specifies the namespace:
```
`kubectl exec -i -t dnsutils -- nslookup &lt;service-name&gt;.&lt;namespace&gt;
`
```
To learn more about name resolution, see
[DNS for Services and Pods](/docs/concepts/services-networking/dns-pod-service/#what-things-get-dns-names).