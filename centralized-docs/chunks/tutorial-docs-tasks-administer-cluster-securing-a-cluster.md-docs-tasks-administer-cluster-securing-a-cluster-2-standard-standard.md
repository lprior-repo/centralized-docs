---
doc_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster
chunk_id: tutorial/docs-tasks-administer-cluster-securing-a-cluster.md/docs-tasks-administer-cluster-securing-a-cluster#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 304
summary: - [SCTP is not used in most Kubernetes clusters, and has also had](#sctp-is-not-used-in-most-kubernetes-clusters-and-has-also-had) - [vulnerabilities in the past.](#vulnerabilities-in-the-past)     -...
---

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