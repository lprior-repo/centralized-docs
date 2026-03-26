---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#16-summary
chunk_level: summary
chunk_type: prose
heading: Why does the kubelet manage the hosts file?
token_count: 35
summary: #### Caution: Avoid making manual changes to the hosts file inside a container. If you make manual changes to the hosts file, those changes are lost when the container exits.
---

#### Caution:
Avoid making manual changes to the hosts file inside a container.
If you make manual changes to the hosts file,
those changes are lost when the container exits.