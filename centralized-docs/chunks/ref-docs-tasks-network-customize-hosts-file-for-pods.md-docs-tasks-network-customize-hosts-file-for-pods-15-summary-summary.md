---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#15-summary
chunk_level: summary
chunk_type: prose
heading: Why does the kubelet manage the hosts file?
token_count: 120
summary: ## Why does the kubelet manage the hosts file? The kubelet manages the `hosts` file for each container of the Pod to prevent the container runtime from modifying the file after the containers have...
---

## Why does the kubelet manage the hosts file?
The kubelet manages the
`hosts` file for each container of the Pod to prevent the container runtime from
modifying the file after the containers have already been started.
Historically, Kubernetes always used Docker Engine as its container runtime, and Docker Engine would
then modify the `/etc/hosts` file after each container had started.
Current Kubernetes can use a variety of container runtimes; even so, the kubelet manages the
hosts file within each container so that the outcome is as intended regardless of which
container runtime you use.