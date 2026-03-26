---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 857
summary: ### Are you in the right namespace for the service? DNS queries that don't specify a namespace are limited to the pod's namespace. If the namespace of the pod and service differ, the DNS query must...
---

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
## Known issues
Some Linux distributions (e.g. Ubuntu) use a local DNS resolver by default (systemd-resolved).
Systemd-resolved moves and replaces `/etc/resolv.conf` with a stub file that can cause a fatal forwarding
loop when resolving names in upstream servers. This can be fixed manually by using kubelet's `--resolv-conf` flag
to point to the correct `resolv.conf` (With `systemd-resolved`, this is `/run/systemd/resolve/resolv.conf`).
kubeadm automatically detects `systemd-resolved`, and adjusts the kubelet flags accordingly.
Kubernetes installs do not configure the nodes' `resolv.conf` files to use the
cluster DNS by default, because that process is inherently distribution-specific.
This should probably be implemented eventually.
Linux's libc (a.k.a. glibc) has a limit for the DNS `nameserver` records to 3 by
default and Kubernetes needs to consume 1 `nameserver` record. This means that
if a local installation already uses 3 `nameserver`s, some of those entries will
be lost. To work around this limit, the node can run `dnsmasq`, which will
provide more `nameserver` entries. You can also use kubelet's `--resolv-conf`
flag.
If you are using Alpine version 3.17 or earlier as your base image, DNS may not
work properly due to a design issue with Alpine.
Until musl version 1.24 didn't include TCP fallback to the DNS stub resolver meaning any DNS call above 512 bytes would fail.
Please upgrade your images to Alpine version 3.18 or above.
## What's next
* See [Autoscaling the DNS Service in a Cluster](/docs/tasks/administer-cluster/dns-horizontal-autoscaling/).
* Read [DNS for Services and Pods](/docs/concepts/services-networking/dns-pod-service/)
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified February 08, 2026 at 3:03 PM PST: [Label uses kubernetes.io, not k8s.io (ed8ca6db68)](https://github.com/kubernetes/website/commit/ed8ca6db685b482a6dadb45cebb03a0ce3d12957)
## Related Pages

- [Using RBAC Authorization](docs-reference-access-authn-authz-rbac.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Hello Minikube](docs-tutorials-hello-minikube.md)
- [Debug Pods](docs-tasks-debug-debug-application-debug-pods.md)
- [Tools for Monitoring Resources](docs-tasks-debug-debug-cluster-resource-usage-monitoring.md)