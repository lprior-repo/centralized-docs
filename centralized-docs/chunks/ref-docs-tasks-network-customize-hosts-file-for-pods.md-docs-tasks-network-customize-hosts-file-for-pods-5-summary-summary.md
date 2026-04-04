---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#5-summary
chunk_level: summary
chunk_type: prose
heading: Default hosts file content
token_count: 105
summary: The hosts file content would look like this: ``` `kubectl exec nginx -- cat /etc/hosts ` ``` ``` `# Kubernetes-managed hosts file. 127.0.0.1 localhost ::1 localhost ip6-localhost ip6-loopback fe00::0...
---

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