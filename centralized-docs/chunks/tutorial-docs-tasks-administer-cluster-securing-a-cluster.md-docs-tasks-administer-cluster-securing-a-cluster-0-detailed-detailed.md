---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 529
summary: ## Table of Contents  - [Securing a Cluster](#securing-a-cluster)   - [Before you begin](#before-you-begin)   - [Controlling access to the Kubernetes API](#controlling-access-to-the-kubernetes-api)  ...
---

## Table of Contents

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
- [vulnerabilities in the past.](#vulnerabilities-in-the-past)
    - [Restricting network access](#restricting-network-access)
    - [Restricting cloud metadata API access](#restricting-cloud-metadata-api-access)
    - [Controlling which nodes pods may access](#controlling-which-nodes-pods-may-access)
  - [Protecting cluster components from compromise](#protecting-cluster-components-from-compromise)
    - [Restrict access to etcd](#restrict-access-to-etcd)
      - [Caution:](#caution)
    - [Enable audit logging](#enable-audit-logging)
    - [Restrict access to alpha or beta features](#restrict-access-to-alpha-or-beta-features)
    - [Rotate infrastructure credentials frequently](#rotate-infrastructure-credentials-frequently)
    - [Review third party integrations before enabling them](#review-third-party-integrations-before-enabling-them)
    - [Encrypt secrets at rest](#encrypt-secrets-at-rest)
    - [Receiving alerts for security updates and reporting vulnerabilities](#receiving-alerts-for-security-updates-and-reporting-vulnerabilities)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---