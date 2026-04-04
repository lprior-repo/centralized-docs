---
doc_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
chunk_id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods#3-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 119
summary: # Adding entries to Pod /etc/hosts with HostAliases Adding entries to a Pod's `/etc/hosts` file provides Pod-level override of hostname resolution when DNS and other options are not applicable. You...
---

# Adding entries to Pod /etc/hosts with HostAliases
Adding entries to a Pod's `/etc/hosts` file provides Pod-level override of hostname resolution when DNS and other options are not applicable. You can add these custom entries with the HostAliases field in PodSpec.
The Kubernetes project recommends modifying DNS configuration using the `hostAliases` field
(part of the `.spec` for a Pod), and not by using an init container or other means to edit `/etc/hosts`
directly.
Change made in other ways may be overwritten by the kubelet during Pod creation or restart.