---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#3-standard
chunk_level: standard
chunk_type: code
heading: Before you begin
token_count: 399
summary: ### Create a simple Pod to use as a test environment [`admin/dns/dnsutils.yaml`...
---

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