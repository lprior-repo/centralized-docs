---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#1-standard
chunk_level: standard
chunk_type: code
heading: Default hosts file content
token_count: 352
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