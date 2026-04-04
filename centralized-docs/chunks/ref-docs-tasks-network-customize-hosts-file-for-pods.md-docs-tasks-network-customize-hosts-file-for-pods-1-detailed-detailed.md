---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#1-detailed
chunk_level: detailed
chunk_type: code
heading: Adding additional entries with hostAliases
token_count: 875
summary: # Adding entries to Pod /etc/hosts with HostAliases Adding entries to a Pod's `/etc/hosts` file provides Pod-level override of hostname resolution when DNS and other options are not applicable. You...
---

# Adding entries to Pod /etc/hosts with HostAliases
Adding entries to a Pod's `/etc/hosts` file provides Pod-level override of hostname resolution when DNS and other options are not applicable. You can add these custom entries with the HostAliases field in PodSpec.
The Kubernetes project recommends modifying DNS configuration using the `hostAliases` field
(part of the `.spec` for a Pod), and not by using an init container or other means to edit `/etc/hosts`
directly.
Change made in other ways may be overwritten by the kubelet during Pod creation or restart.
## Default hosts file content
Start an Nginx Pod which is assigned a Pod IP:
```
`kubectl run nginx --image nginx
`
```
```
`pod/nginx created
`
```
Examine a Pod IP:
```
`kubectl get pods --output=wide
`
```
```
`NAME READY STATUS RESTARTS AGE IP NODE
nginx 1/1 Running 0 13s 10.200.0.4 worker0
`
```
The hosts file content would look like this:
```
`kubectl exec nginx -- cat /etc/hosts
`
```
```
`# Kubernetes-managed hosts file.
127.0.0.1 localhost
::1 localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
fe00::0 ip6-mcastprefix
fe00::1 ip6-allnodes
fe00::2 ip6-allrouters
10.200.0.4 nginx
`
```
By default, the `hosts` file only includes IPv4 and IPv6 boilerplates like
`localhost` and its own hostname.
## Adding additional entries with hostAliases
In addition to the default boilerplate, you can add additional entries to the
`hosts` file.
For example: to resolve `foo.local`, `bar.local` to `127.0.0.1` and `foo.remote`,
`bar.remote` to `10.1.2.3`, you can configure HostAliases for a Pod under
`.spec.hostAliases`:
[`service/networking/hostaliases-pod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/hostaliases-pod.yaml)![](/images/copycode.svg "Copy service/networking/hostaliases-pod.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: hostaliases-pod
spec:
restartPolicy: Never
hostAliases:
- ip: "127.0.0.1"
hostnames:
- "foo.local"
- "bar.local"
- ip: "10.1.2.3"
hostnames:
- "foo.remote"
- "bar.remote"
containers:
- name: cat-hosts
image: busybox:1.28
command:
- cat
args:
- "/etc/hosts"
`
```
You can start a Pod with that configuration by running:
```
`kubectl apply -f https://k8s.io/examples/service/networking/hostaliases-pod.yaml
`
```
```
`pod/hostaliases-pod created
`
```
Examine a Pod's details to see its IPv4 address and its status:
```
`kubectl get pod --output=wide
`
```
```
`NAME READY STATUS RESTARTS AGE IP NODE
hostaliases-pod 0/1 Completed 0 6s 10.200.0.5 worker0
`
```
The `hosts` file content looks like this:
```
`kubectl logs hostaliases-pod
`
```
```
`# Kubernetes-managed hosts file.
127.0.0.1 localhost
::1 localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
fe00::0 ip6-mcastprefix
fe00::1 ip6-allnodes
fe00::2 ip6-allrouters
10.200.0.5 hostaliases-pod
# Entries added by HostAliases.
127.0.0.1 foo.local bar.local
10.1.2.3 foo.remote bar.remote
`
```
with the additional entries specified at the bottom.