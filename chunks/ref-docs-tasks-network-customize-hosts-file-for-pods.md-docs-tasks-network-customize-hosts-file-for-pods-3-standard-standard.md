---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#3-standard
chunk_level: standard
chunk_type: prose
heading: Why does the kubelet manage the hosts file?
token_count: 373
summary: Examine a Pod's details to see its IPv4 address and its status: ``` `kubectl get pod --output=wide ` ``` ``` `NAME READY STATUS RESTARTS AGE IP NODE hostaliases-pod 0/1 Completed 0 6s 10.200.0.5...
---

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
## Why does the kubelet manage the hosts file?
The kubelet manages the
`hosts` file for each container of the Pod to prevent the container runtime from
modifying the file after the containers have already been started.
Historically, Kubernetes always used Docker Engine as its container runtime, and Docker Engine would
then modify the `/etc/hosts` file after each container had started.
Current Kubernetes can use a variety of container runtimes; even so, the kubelet manages the
hosts file within each container so that the outcome is as intended regardless of which
container runtime you use.
#### Caution:
Avoid making manual changes to the hosts file inside a container.
If you make manual changes to the hosts file,
those changes are lost when the container exits.