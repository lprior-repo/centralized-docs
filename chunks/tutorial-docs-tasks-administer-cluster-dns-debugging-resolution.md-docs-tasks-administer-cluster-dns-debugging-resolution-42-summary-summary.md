---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#42-summary
chunk_level: summary
chunk_type: prose
heading: Known issues
token_count: 123
summary: Some Linux distributions (e.g. Ubuntu) use a local DNS resolver by default (systemd-resolved). Systemd-resolved moves and replaces `/etc/resolv.conf` with a stub file that can cause a fatal...
---

Some Linux distributions (e.g. Ubuntu) use a local DNS resolver by default (systemd-resolved).
Systemd-resolved moves and replaces `/etc/resolv.conf` with a stub file that can cause a fatal forwarding
loop when resolving names in upstream servers. This can be fixed manually by using kubelet's `--resolv-conf` flag
to point to the correct `resolv.conf` (With `systemd-resolved`, this is `/run/systemd/resolve/resolv.conf`).
kubeadm automatically detects `systemd-resolved`, and adjusts the kubelet flags accordingly.