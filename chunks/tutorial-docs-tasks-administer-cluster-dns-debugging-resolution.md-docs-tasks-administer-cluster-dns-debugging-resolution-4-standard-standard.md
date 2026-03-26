---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#4-standard
chunk_level: standard
chunk_type: code
heading: Before you begin
token_count: 494
summary: ### Check the local DNS configuration first Take a look inside the resolv.conf file. (See [Customizing DNS Service](/docs/tasks/administer-cluster/dns-custom-nameservers/) and [Known...
---

### Check the local DNS configuration first
Take a look inside the resolv.conf file.
(See [Customizing DNS Service](/docs/tasks/administer-cluster/dns-custom-nameservers/) and
[Known issues](#known-issues) below for more information)
```
`kubectl exec -ti dnsutils -- cat /etc/resolv.conf
`
```
Verify that the search path and name server are set up like the following
(note that search path may vary for different cloud providers):
```
`search default.svc.cluster.local svc.cluster.local cluster.local google.internal c.gce\_project\_id.internal
nameserver 10.0.0.10
options ndots:5
`
```
Errors such as the following indicate a problem with the CoreDNS (or kube-dns)
add-on or with associated Services:
```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```
```
`Server: 10.0.0.10
Address 1: 10.0.0.10
nslookup: can't resolve 'kubernetes.default'
`
```
or
```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```
```
`Server: 10.0.0.10
Address 1: 10.0.0.10 kube-dns.kube-system.svc.cluster.local
nslookup: can't resolve 'kubernetes.default'
`
```
### Check if the DNS pod is running
Use the `kubectl get pods` command to verify that the DNS pod is running.
```
`kubectl get pods --namespace=kube-system -l k8s-app=kube-dns
`
```
```
`NAME READY STATUS RESTARTS AGE
...
coredns-7b96bf9f76-5hsxb 1/1 Running 0 1h
coredns-7b96bf9f76-mvmmt 1/1 Running 0 1h
...
`
```
#### Note:
The value for label `k8s-app` is `kube-dns` for both CoreDNS and kube-dns deployments.
If you see that no CoreDNS Pod is running or that the Pod has failed/completed,
the DNS add-on may not be deployed by default in your current environment and you
will have to deploy it manually.