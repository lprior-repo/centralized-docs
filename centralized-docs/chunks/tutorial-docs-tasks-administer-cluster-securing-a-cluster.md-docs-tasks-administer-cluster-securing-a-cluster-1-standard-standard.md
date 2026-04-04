---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 256
summary: - [Securing a Cluster](#securing-a-cluster)   - [Before you begin](#before-you-begin)   - [Controlling access to the Kubernetes API](#controlling-access-to-the-kubernetes-api)     - [Use Transport...
---

- [Securing a Cluster](#securing-a-cluster)
  - [Before you begin](#before-you-begin)
  - [Controlling access to the Kubernetes API](#controlling-access-to-the-kubernetes-api)
    - [Use Transport Layer Security (TLS) for all API traffic](#use-transport-layer-security-tls-for-all-api-traffic)
    - [API Authentication](#api-authentication)
    - [API Authorization](#api-authorization)
  - [Controlling access to the Kubelet](#controlling-access-to-the-kubelet)
  - [Controlling the capabilities of a workload or user at runtime](#controlling-the-capabilities-of-a-workload-or-user-at-runtime)
    - [Limiting resource usage on a cluster](#limiting-resource-usage-on-a-cluster)
    - [Controlling what privileges containers run with](#controlling-what-privileges-containers-run-with)
    - [Preventing containers from loading unwanted kernel modules](#preventing-containers-from-loading-unwanted-kernel-modules)
- [SCTP is not used in most Kubernetes clusters, and has also had](#sctp-is-not-used-in-most-kubernetes-clusters-and-has-also-had)