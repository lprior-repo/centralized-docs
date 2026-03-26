---
doc_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution
chunk_id: tutorial/docs-tasks-administer-cluster-dns-debugging-resolution.md/docs-tasks-administer-cluster-dns-debugging-resolution#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 288
summary: ## Table of Contents  - [Debugging DNS Resolution](#debugging-dns-resolution)   - [Before you begin](#before-you-begin)     - [Create a simple Pod to use as a test...
---

## Table of Contents

- [Debugging DNS Resolution](#debugging-dns-resolution)
  - [Before you begin](#before-you-begin)
    - [Create a simple Pod to use as a test environment](#create-a-simple-pod-to-use-as-a-test-environment)
      - [Note:](#note)
    - [Check the local DNS configuration first](#check-the-local-dns-configuration-first)
    - [Check if the DNS pod is running](#check-if-the-dns-pod-is-running)
      - [Note:](#note)
    - [Check for errors in the DNS pod](#check-for-errors-in-the-dns-pod)
    - [Is DNS service up?](#is-dns-service-up)
      - [Note:](#note)
    - [Are DNS endpoints exposed?](#are-dns-endpoints-exposed)
    - [Are DNS queries being received/processed?](#are-dns-queries-being-receivedprocessed)
    - [Does CoreDNS have sufficient permissions?](#does-coredns-have-sufficient-permissions)
    - [Are you in the right namespace for the service?](#are-you-in-the-right-namespace-for-the-service)
  - [Known issues](#known-issues)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---