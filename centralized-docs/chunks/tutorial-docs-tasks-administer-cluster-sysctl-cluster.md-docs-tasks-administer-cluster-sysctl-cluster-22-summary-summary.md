---
doc_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-sysctl-cluster.md/docs-tasks-administer-cluster-sysctl-cluster#22-summary
chunk_level: summary
chunk_type: prose
heading: Setting Sysctls for a Pod
token_count: 122
summary: * Those `net.\*` that can be set in container networking namespace. However, there are exceptions (e.g., `net.netfilter.nf\_conntrack\_max` and `net.netfilter.nf\_conntrack\_expect\_max` can be set...
---

* Those `net.\*` that can be set in container networking namespace. However,
there are exceptions (e.g., `net.netfilter.nf\_conntrack\_max` and
`net.netfilter.nf\_conntrack\_expect\_max` can be set in container networking
namespace but are unnamespaced before Linux 5.12.2).
Sysctls with no namespace are called *node-level* sysctls. If you need to set
them, you must manually configure them on each node's operating system, or by
using a DaemonSet with privileged containers.