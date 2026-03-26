---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#13-summary
chunk_level: summary
chunk_type: prose
heading: Adding additional entries with hostAliases
token_count: 127
summary: ``` `kubectl logs hostaliases-pod ` ``` ``` `# Kubernetes-managed hosts file. 127.0.0.1 localhost ::1 localhost ip6-localhost ip6-loopback fe00::0 ip6-localnet fe00::0 ip6-mcastprefix fe00::1...
---

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