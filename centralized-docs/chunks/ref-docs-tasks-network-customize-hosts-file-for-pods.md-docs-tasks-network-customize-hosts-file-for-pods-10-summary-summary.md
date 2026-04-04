---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#10-summary
chunk_level: summary
chunk_type: prose
heading: Adding additional entries with hostAliases
token_count: 127
summary: ``` `apiVersion: v1 kind: Pod metadata: name: hostaliases-pod spec: restartPolicy: Never hostAliases: - ip: \"127.0.0.1\" hostnames: - \"foo.local\" - \"bar.local\" - ip: \"10.1.2.3\" hostnames: -...
---

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