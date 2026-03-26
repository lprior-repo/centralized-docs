---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#43-summary
chunk_level: summary
chunk_type: prose
heading: Known issues
token_count: 123
summary: ). kubeadm automatically detects `systemd-resolved`, and adjusts the kubelet flags accordingly. Kubernetes installs do not configure the nodes' `resolv.conf` files to use the cluster DNS by default,...
---

).
kubeadm automatically detects `systemd-resolved`, and adjusts the kubelet flags accordingly.
Kubernetes installs do not configure the nodes' `resolv.conf` files to use the
cluster DNS by default, because that process is inherently distribution-specific.
This should probably be implemented eventually.
Linux's libc (a.k.a. glibc) has a limit for the DNS `nameserver` records to 3 by
default and Kubernetes needs to consume 1 `nameserver` record. This means that
if a local installation already uses 3 `nameserver`s, some of those entries will