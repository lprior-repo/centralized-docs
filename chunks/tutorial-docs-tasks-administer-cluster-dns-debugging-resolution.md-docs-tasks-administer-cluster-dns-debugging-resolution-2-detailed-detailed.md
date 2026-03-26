---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#2-detailed
chunk_level: detailed
chunk_type: code
heading: Before you begin
token_count: 926
summary: ## Before you begin You need to have a Kubernetes cluster, and the kubectl command-line tool must be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster...
---

## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)
Your cluster must be configured to use the CoreDNS
[addon](/docs/concepts/cluster-administration/addons/) or its precursor,
kube-dns.
Your Kubernetes server must be at or later than version v1.6.
To check the version, enter `kubectl version`.
### Create a simple Pod to use as a test environment
[`admin/dns/dnsutils.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/dns/dnsutils.yaml)![](/images/copycode.svg "Copy admin/dns/dnsutils.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: dnsutils
namespace: default
spec:
containers:
- name: dnsutils
image: registry.k8s.io/e2e-test-images/agnhost:2.39
imagePullPolicy: IfNotPresent
restartPolicy: Always
`
```
#### Note:
This example creates a pod in the `default` namespace. DNS name resolution for
services depends on the namespace of the pod. For more information, review
[DNS for Services and Pods](/docs/concepts/services-networking/dns-pod-service/#what-things-get-dns-names).
Use that manifest to create a Pod:
```
`kubectl apply -f https://k8s.io/examples/admin/dns/dnsutils.yaml
`
```
```
`pod/dnsutils created
`
```
…and verify its status:
```
`kubectl get pods dnsutils
`
```
```
`NAME READY STATUS RESTARTS AGE
dnsutils 1/1 Running 0 &lt;some-time&gt;
`
```
Once that Pod is running, you can exec `nslookup` in that environment.
If you see something like the following, DNS is working correctly.
```
`kubectl exec -i -t dnsutils -- nslookup kubernetes.default
`
```
```
`Server: 10.0.0.10
Address 1: 10.0.0.10
Name: kubernetes.default
Address 1: 10.0.0.1
`
```
If the `nslookup` command fails, check the following:
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